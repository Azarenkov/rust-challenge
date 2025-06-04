mod domain;
mod generator;
mod pipeline;
// mod storage;

// use generator::generate_transfers;
// use pipeline::calcula   te_user_stats;

fn main() {
    let transfers = generate_transfers(10_000);

    let stats = calculate_user_stats(&transfers);

    for stat in stats.iter().take(10) {
        println!("{:?}", stat);
    }
}
