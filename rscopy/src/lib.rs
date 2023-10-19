use polars::prelude::*;

pub fn read_csv_to_df(file_path: &str) -> Result<DataFrame, PolarsError> {
    // Read the CSV file into a Polars DataFrame
    let df = CsvReader::from_path(file_path)?.finish()?;

    // Display the DataFrame
    println!("DataFrame Contents:");
    println!("{:?}", df);

    Ok(df)
}

pub fn describe_data(df: &DataFrame) {
    // Calculate the mean of a specific column

    let columns = df.get_columns();
    let average0 = columns[0].mean().unwrap_or_default() as i64;
    let average1 = columns[1].mean().unwrap_or_default() as i64;
    let average2 = columns[2].mean().unwrap_or_default() as i64;
    let average3 = columns[3].mean().unwrap_or_default() as i64;

    println!(
        "Average: {},{},{},{}",
        average0, average1, average2, average3
    );
}
