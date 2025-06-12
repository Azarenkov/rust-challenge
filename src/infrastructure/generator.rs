use crate::domain::entities::transfer::Transfer;
use anyhow::Result;
use rand::{Rng, distributions::Alphanumeric};
use std::time::{SystemTime, UNIX_EPOCH};

pub trait TransferGenerator {
    fn generate(&self, count: usize) -> Result<Vec<Transfer>>;
}

#[derive(Debug, Clone)]
pub struct TransferGenConfig {
    pub min_amount: f64,
    pub max_amount: f64,
    pub min_price: f64,
    pub max_price: f64,
    pub max_age_secs: u64,
    pub address_pool_size: usize,
}

impl Default for TransferGenConfig {
    fn default() -> Self {
        Self {
            min_amount: 1.0,
            max_amount: 1000.0,
            min_price: 0.1,
            max_price: 2.0,
            max_age_secs: 86_400 * 30,
            address_pool_size: 40,
        }
    }
}

impl TransferGenerator for TransferGenConfig {
    fn generate(&self, count: usize) -> Result<Vec<Transfer>> {
        let mut rng = rand::thread_rng();
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        let address_pool: Vec<String> = (0..self.address_pool_size)
            .map(|_| rand_address(&mut rng))
            .collect();

        let data = (0..count)
            .map(|_| {
                let from_idx = rng.gen_range(0..address_pool.len());
                let mut to_idx = rng.gen_range(0..address_pool.len());

                while to_idx == from_idx {
                    to_idx = rng.gen_range(0..address_pool.len());
                }

                let from = address_pool[from_idx].clone();
                let to = address_pool[to_idx].clone();
                let amount = rng.gen_range(self.min_amount..self.max_amount);
                let usd_price = rng.gen_range(self.min_price..self.max_price);
                let ts = now - rng.gen_range(0..self.max_age_secs);

                Transfer {
                    ts,
                    from,
                    to,
                    amount,
                    usd_price,
                }
            })
            .collect();

        Ok(data)
    }
}

fn rand_address(rng: &mut impl Rng) -> String {
    let suffix: String = rng
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    format!("0x{}", suffix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_count() {
        let config = TransferGenConfig::default();
        let transfers = config.generate(10).unwrap();
        assert_eq!(transfers.len(), 10);
    }

    #[test]
    fn test_generate_zero_count() {
        let config = TransferGenConfig::default();
        let transfers = config.generate(0).unwrap();
        assert_eq!(transfers.len(), 0);
    }

    #[test]
    fn test_generated_values_in_range() {
        let config = TransferGenConfig {
            min_amount: 50.0,
            max_amount: 100.0,
            min_price: 1.0,
            max_price: 2.0,
            max_age_secs: 3600,
            address_pool_size: 10,
        };

        let transfers = config.generate(100).unwrap();

        for transfer in transfers {
            assert!(transfer.amount >= 50.0 && transfer.amount < 100.0);
            assert!(transfer.usd_price >= 1.0 && transfer.usd_price < 2.0);
            assert!(transfer.from.starts_with("0x"));
            assert!(transfer.to.starts_with("0x"));
            assert_eq!(transfer.from.len(), 12); // "0x" + 10 chars
            assert_eq!(transfer.to.len(), 12);
            assert!(transfer.ts > 0);
        }
    }

    #[test]
    fn test_addresses_are_different() {
        let config = TransferGenConfig::default();
        let transfers = config.generate(100).unwrap();

        for transfer in transfers {
            assert_ne!(
                transfer.from, transfer.to,
                "From and To addresses should be different"
            );
        }
    }

    #[test]
    fn test_address_pool_size_respected() {
        let config = TransferGenConfig {
            min_amount: 1.0,
            max_amount: 10.0,
            min_price: 1.0,
            max_price: 2.0,
            max_age_secs: 3600,
            address_pool_size: 5,
        };

        let transfers = config.generate(50).unwrap();

        let mut all_addresses = std::collections::HashSet::new();
        for transfer in &transfers {
            all_addresses.insert(&transfer.from);
            all_addresses.insert(&transfer.to);
        }

        assert!(all_addresses.len() <= 5);
        println!(
            "Generated {} unique addresses from {} transfers",
            all_addresses.len(),
            transfers.len()
        );
    }

    #[test]
    fn test_address_reuse() {
        let config = TransferGenConfig {
            min_amount: 1.0,
            max_amount: 10.0,
            min_price: 1.0,
            max_price: 2.0,
            max_age_secs: 3600,
            address_pool_size: 3,
        };

        let transfers = config.generate(20).unwrap();

        let mut address_usage: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();
        for transfer in &transfers {
            *address_usage.entry(transfer.from.clone()).or_insert(0) += 1;
            *address_usage.entry(transfer.to.clone()).or_insert(0) += 1;
        }

        assert!(address_usage.len() <= 3);

        for (address, count) in address_usage {
            assert!(
                count > 1,
                "Address {} should be used more than once",
                address
            );
        }
    }

    #[test]
    fn test_rand_address() {
        let mut rng = rand::thread_rng();

        let addr1 = rand_address(&mut rng);
        let addr2 = rand_address(&mut rng);

        assert!(addr1.starts_with("0x"));
        assert!(addr2.starts_with("0x"));
        assert_eq!(addr1.len(), 12);
        assert_eq!(addr2.len(), 12);
        assert_ne!(addr1, addr2);
    }

    #[test]
    fn test_timestamp_generation() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let config = TransferGenConfig {
            min_amount: 1.0,
            max_amount: 10.0,
            min_price: 1.0,
            max_price: 2.0,
            max_age_secs: 3600, // 1 час
            address_pool_size: 5,
        };

        let transfers = config.generate(10).unwrap();

        for transfer in transfers {
            assert!(transfer.ts <= now);
            assert!(transfer.ts >= now - 3600);
        }
    }

    #[test]
    fn test_error_handling() {
        let config = TransferGenConfig::default();
        let result = config.generate(10);
        assert!(result.is_ok());
    }

    #[test]
    fn test_result_unwrap_or_else() {
        let config = TransferGenConfig::default();
        let transfers = config.generate(5).unwrap_or_else(|e| {
            panic!("Failed to generate transfers: {}", e);
        });
        assert_eq!(transfers.len(), 5);
    }

    #[test]
    fn test_default_config() {
        let config = TransferGenConfig::default();

        assert_eq!(config.min_amount, 1.0);
        assert_eq!(config.max_amount, 1000.0);
        assert_eq!(config.min_price, 0.1);
        assert_eq!(config.max_price, 2.0);
        assert_eq!(config.max_age_secs, 86_400 * 30);
        assert_eq!(config.address_pool_size, 40);
    }

    #[test]
    fn test_single_address_pool() {
        let config = TransferGenConfig {
            min_amount: 1.0,
            max_amount: 10.0,
            min_price: 1.0,
            max_price: 2.0,
            max_age_secs: 3600,
            address_pool_size: 2,
        };

        let transfers = config.generate(10).unwrap();

        assert_eq!(transfers.len(), 10);

        for transfer in &transfers {
            assert_ne!(transfer.from, transfer.to);
        }

        let mut all_addresses = std::collections::HashSet::new();
        for transfer in &transfers {
            all_addresses.insert(&transfer.from);
            all_addresses.insert(&transfer.to);
        }
        assert_eq!(all_addresses.len(), 2);
    }

    #[test]
    fn test_large_address_pool() {
        let config = TransferGenConfig {
            min_amount: 1.0,
            max_amount: 10.0,
            min_price: 1.0,
            max_price: 2.0,
            max_age_secs: 3600,
            address_pool_size: 100,
        };

        let transfers = config.generate(50).unwrap();

        let mut all_addresses = std::collections::HashSet::new();
        for transfer in &transfers {
            all_addresses.insert(&transfer.from);
            all_addresses.insert(&transfer.to);
        }

        assert!(all_addresses.len() <= 100);
        assert!(all_addresses.len() >= 2);
    }
}
