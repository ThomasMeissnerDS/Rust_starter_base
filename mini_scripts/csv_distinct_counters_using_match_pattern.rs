/*
Script that reads a csv in a loop. Takes a column and counts the number of distinct values within
that column. Accepts two arguments:
- the column name that shall be counted
- path to the csv file (has to include .csv)
 */
use::std::collections::HashMap;
use std::env;
use csv::ReaderBuilder;
use csv::Error;

fn main() -> Result<(), Error> {
    // get command line arguments
    let args: Vec<String> = env::args().collect();
    let count_col = String::from(&args[1]);
    let csv = String::from(&args[2]);

    let mut col_indices: HashMap<String, usize> = HashMap::new();
    let mut col_counts: HashMap<String, u32> = HashMap::new();

    let mut reader = ReaderBuilder::new()
        .flexible(true)
        .has_headers(false)
        .from_path(csv)?;

    let mut nb_columns: usize;

    let mut row_idx: u32 = 0;

    for record in reader.records() {
        let record = record?;
        // get the header
        match row_idx {
            0 => {
            nb_columns = record.len();
            // loop through columns..map column_name to column_index in hashmap
            for idx in 0..nb_columns {
                let temp_str = String::from(&record[idx]); // convert to String to be able to deep copy
                col_indices.insert(temp_str, idx); // hashmap takes ownership
                }
            }
            _ => {
            let temp_str = String::from(&record[*col_indices.get(&count_col).unwrap()]); // convert to String to be able to deep copy
                        *col_counts.entry(temp_str.to_owned()).or_default() += 1;
            }
        }
        // get all rows except header
        row_idx += 1;
    }
    println!("The distinct number of {} categories is {}", count_col, col_counts.keys().len());
    Ok(())
}