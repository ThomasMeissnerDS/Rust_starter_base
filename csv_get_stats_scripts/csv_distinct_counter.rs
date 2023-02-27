/*
Script that reads a csv in a loop. Takes a column and counts the number of distinct values within
that column. Accepts two arguments:
- the column name that shall be counted
- path to the csv file (has to include .csv)
*/
use std::collections::HashMap;
use std::env;
use csv::{Error, ReaderBuilder};

fn main() -> Result<(), Error> {
    // get command line arguments
    let count_col = &env::args().nth(1).expect("count_col not provided");
    let csv = &env::args().nth(2).expect("file_name not provided");

    let mut col_indices: HashMap<String, usize> = HashMap::new();
    let mut col_counts: HashMap<String, Vec<(u32, f64, f64)>> = HashMap::new();

    let mut reader = ReaderBuilder::new()
        .flexible(true)
        .has_headers(false)
        .from_path(csv)?;
    let mut row_idx: u32 = 0;
    let mut nb_columns: usize;

    for record in reader.records() {
        let record = record?;
        // get the header
        if row_idx == 0 {
            nb_columns = record.len();
            // loop through columns..map column_name to column_index in hashmap
            for idx in 0..nb_columns {
                let temp_str = String::from(&record[idx]); // convert to String to be able to deep copy
                col_indices.insert(temp_str, idx); // hashmap takes ownership
            }
        }
        // get all rows except header
        else {
            let count_col_index = *col_indices.get(count_col).unwrap();
            let group_val = record[count_col_index].to_string();

            // update counts, sum, and mean for the current group value
            let count = col_counts.entry(group_val.clone())
                .or_insert_with(|| vec![(0, 0.0, 0.0)])
                .first_mut().unwrap();
            count.0 += 1;
            count.1 += 1.0;
            count.2 = count.1 / (count.0 as f64);

            // print some stats for each group value
            if row_idx % 100000 == 0 {
                println!("Group value: {}, count: {}, sum: {}, mean: {}", group_val, count.0, count.1, count.2);
            }
        }

        row_idx += 1;
    }

    println!("The distinct number of {} categories is {}", count_col, col_counts.keys().len());
    Ok(())
}
