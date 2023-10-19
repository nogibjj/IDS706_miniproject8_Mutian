mod lib;
use lib::describe_data;
use lib::read_csv_to_df;
use std::env;

fn main() {
    let start_time = std::time::Instant::now();
    let current_dir = env::current_dir().unwrap();
    println!("Current directory: {:?}", current_dir);

    let csv_file = "../taker_buy_sell_volume.csv";
    match read_csv_to_df(csv_file) {
        Ok(data) => {
            describe_data(&data);
        }
        Err(err) => {
            println!("Error reading CSV: {:?}", err);
        }
    }
    let end_time = std::time::Instant::now();
    let processing_time = end_time.duration_since(start_time);
    println!("Processing time: {:?}", processing_time);
}
