mod lib;
use lib::describe_data;
use lib::read_csv_to_df;
use std::env;

fn main() {
    let start_time = std::time::Instant::now();
    let csv_file = "../taker_buy_sell_volume.csv";
    let end_time = std::time::Instant::now();
    let processing_time = end_time.duration_since(start_time);
    match read_csv_to_df(csv_file) {
        Ok(data) => {
            describe_data(&data);
        }
        Err(err) => {
            println!("Error reading CSV: {:?}", err);
        }
    }

    println!("Processing time: {:?}", processing_time);
}
