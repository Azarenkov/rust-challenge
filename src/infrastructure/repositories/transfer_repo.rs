use async_trait::async_trait;
use clickhouse::Client;

use crate::domain::{
    entities::{transfer::Transfer, user_stats::UserStats},
    repositories::{
        errors::TransferRepoError,
        transfer_repo::{TransferRepoAbstract, TransferRepoResult},
    },
};

pub struct ClickHouseTransferRepo {
    client: Client,
}

impl ClickHouseTransferRepo {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create_table(&self) -> TransferRepoResult<()> {
        let query = r#"
            CREATE TABLE IF NOT EXISTS transfers (
                ts UInt64,
                from String,
                to String,
                amount Float64,
                usd_price Float64
            ) ENGINE = MergeTree()
            ORDER BY ts
        "#;

        self.client
            .query(query)
            .execute()
            .await
            .map_err(|e| TransferRepoError::QueryError(e.to_string()))?;

        Ok(())
    }
}

#[async_trait]
impl TransferRepoAbstract for ClickHouseTransferRepo {
    async fn save_all(&self, transfers: &[Transfer]) -> TransferRepoResult<()> {
        if transfers.is_empty() {
            return Ok(());
        }

        let mut insert = self
            .client
            .insert("transfers")
            .map_err(|e| TransferRepoError::DatabaseConnectionError(e.to_string()))?;

        for transfer in transfers {
            insert
                .write(transfer)
                .await
                .map_err(|e| TransferRepoError::QueryError(e.to_string()))?;
        }

        insert
            .end()
            .await
            .map_err(|e| TransferRepoError::QueryError(e.to_string()))?;

        Ok(())
    }

    async fn calculate_user_stats(&self) -> TransferRepoResult<Vec<UserStats>> {
        let query = r#"
            WITH
            address_operations AS (
                SELECT
                    to as address,
                    ts,
                    amount,
                    usd_price,
                    'buy' as operation_type
                FROM transfers

                UNION ALL

                SELECT
                    from as address,
                    ts,
                    -amount as amount,
                    usd_price,
                    'sell' as operation_type
                FROM transfers
                ORDER BY address, ts
            ),

            balance_calculations AS (
                SELECT
                    address,
                    ts,
                    amount,
                    usd_price,
                    operation_type,
                    sum(amount) OVER (
                        PARTITION BY address
                        ORDER BY ts
                        ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW
                    ) as running_balance
                FROM address_operations
            ),

            address_stats AS (
                SELECT
                    address,
                    sum(abs(amount)) as total_volume,

                    sum(CASE WHEN amount > 0 THEN amount ELSE 0 END) as buy_volume,
                    sum(CASE WHEN amount > 0 THEN amount * usd_price ELSE 0 END) as buy_value,

                    sum(CASE WHEN amount < 0 THEN -amount ELSE 0 END) as sell_volume,
                    sum(CASE WHEN amount < 0 THEN -amount * usd_price ELSE 0 END) as sell_value,

                    max(running_balance) as max_balance

                FROM balance_calculations
                GROUP BY address
            )

            SELECT
                address,
                total_volume,
                CASE
                    WHEN buy_volume > 0 THEN buy_value / buy_volume
                    ELSE 0
                END as avg_buy_price,
                CASE
                    WHEN sell_volume > 0 THEN sell_value / sell_volume
                    ELSE 0
                END as avg_sell_price,
                GREATEST(max_balance, 0) as max_balance
            FROM address_stats
            WHERE total_volume > 0
            ORDER BY total_volume DESC
        "#;

        let user_stats = self.client.query(query).fetch_all::<UserStats>().await?;

        Ok(user_stats)
    }
}
