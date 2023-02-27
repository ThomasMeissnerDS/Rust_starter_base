use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::borrow::Borrow;

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

    // Loop through remaining rows and accumulate counts, sum, and mean for each group
    let mut counts = HashMap::new();
    for line in lines {
        let record = line.unwrap();
        let record: Vec<&str> = record.split(',').collect();
        let group_val = record[*col_indices.get(groupby_col).unwrap()].to_string();
        let col_val = record[*col_indices.get(count_col).unwrap()].parse::<f64>().unwrap_or(0.0);

        let group_data = counts.entry(group_val.clone()).or_insert(vec![0.0, 0.0, 0.0]);
        group_data[0] += 1.0; // increment count
        group_data[1] += col_val; // add value to sum
        group_data[2] = group_data[1] / group_data[0]; // calculate mean
    }

    // Print group, count, sum, and mean
    println!("group, count, sum, mean");
    for (group_val, group_data) in counts {
        println!("{},{},{},{}", group_val, group_data[0], group_data[1], group_data[2]);
    }
}
