use rust_decimal::prelude::*;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Parse command line arguments
    let groupby_col = &env::args().nth(1).expect("groupby_col not provided");
    let count_col = &env::args().nth(2).expect("count_col not provided");
    let filename = &env::args().nth(3).expect("file_name not provided");

    // Read CSV file
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // Get header row and determine column indices for groupby and count columns
    let header_row = lines.next().unwrap().unwrap();
    let headers: Vec<&str> = header_row.split(',').collect();
    let mut col_indices = HashMap::new();
    col_indices.insert(groupby_col, headers.iter().position(|&x| x == groupby_col).unwrap());
    col_indices.insert(count_col, headers.iter().position(|&x| x == count_col).unwrap());

    // Loop through remaining rows and accumulate counts, sum, and distinct values for each group
    let mut counts = HashMap::new();
    for line in lines {
        let record = line.unwrap();
        let record: Vec<&str> = record.split(',').collect();
        let group_val = record[*col_indices.get(groupby_col).unwrap()].to_string();
        let col_val = Decimal::from_str(record[*col_indices.get(count_col).unwrap()]).unwrap_or_else(|_| Decimal::new(0, 0));
        let (count, sum, values) = counts.entry(group_val.clone()).or_insert((0, Decimal::new(0, 0), HashSet::new()));
        *count += 1;
        *sum += col_val;
        values.insert(col_val);
    }

    // Print group, count, sum, mean, and distinct count pairs
    for (group_val, (count, sum, values)) in counts {
        let mean = if count == 0 { Decimal::new(0, 0) } else { sum / Decimal::new(count, 0) };
        let distinct_count = values.len();
        println!("{},{},{},{},{}", group_val, count, sum, mean, distinct_count);
    }
}
