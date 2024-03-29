use csv;
use rust_decimal::prelude::*;
use std::collections::{HashMap, HashSet};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fs::OpenOptions;
use std::time::Instant;


fn write_to_file_header(path: &str, groupby_col: &str, count_col: String) -> Result<(), Box<dyn Error>> {
    // Creates new `Writer` for `stdout`
    let mut writer = csv::Writer::from_path(path)?;

    // Write records one at a time including the header record.
    writer.write_record(&[
        groupby_col,
        &count_col,
        &format!("{}_zscore", &count_col),
    ])?;

    // A CSV writer maintains an internal buffer, so it's important
    // to flush the buffer when you're done.
    writer.flush()?;

    Ok(())
}

fn write_to_file_row(path: &str, groupby_col: &str, count_col: String, zscore: String) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(path)
        .unwrap();
    // Creates new `Writer` for `stdout`
    let mut writer = csv::Writer::from_writer(file);

    // Write records one at a time including the header record.
    writer.write_record(&[
        groupby_col,
        &count_col,
        &zscore,
    ])?;

    // A CSV writer maintains an internal buffer, so it's important
    // to flush the buffer when you're done.
    writer.flush()?;

    Ok(())
}


fn main() {
let now = Instant::now();
    // Parse command line arguments
    let groupby_col = &env::args().nth(1).expect("groupby_col not provided");
    let count_col = &env::args().nth(2).expect("count_col not provided");
    let filename = &env::args().nth(3).expect("file_name not provided");
    let result_filename = &env::args().nth(4).expect("result file_name not provided");


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

    // Iterate 1st time through rows to get meta data of reference categories of zscores

    // Loop through remaining rows and accumulate counts, sum, and distinct values for each group
    let mut counts = HashMap::new();
    for line in lines {
        let record = line.unwrap();
        let record: Vec<&str> = record.split(',').collect();
        let group_val = record[*col_indices.get(groupby_col).unwrap()].to_string();  // TODO: ADD match statement
        let col_val = Decimal::from_str(record[*col_indices.get(count_col).unwrap()]).unwrap_or_else(|_| Decimal::new(0, 0));
        let (count, sum, values) = counts.entry(group_val.clone()).or_insert((0, Decimal::new(0, 0), HashSet::new()));
        *count += 1;
        *sum += col_val;
        values.insert(col_val);
    }

    //println!("{:?}", &counts);

    // Iterate 2nd time through rows to get standard deviation of reference categories of zscores
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut deltas:HashMap<String, f64> = HashMap::new();
    for line in lines {
        let record = line.unwrap();
        let record: Vec<&str> = record.split(',').collect();
        let group_val = record[*col_indices.get(groupby_col).unwrap()].to_string();
        //println!("{}", &group_val);
        let col_val = Decimal::from_str(record[*col_indices.get(count_col).unwrap()]).unwrap_or_else(|_| Decimal::new(0, 0));

        let group_hash = counts.get(&group_val);
        match group_hash {
            Some(value) => {
            // calculate total of deltas from individual values to group mean
                let sum: f64 = value.1.to_f64().unwrap();
                let counts: f64 =  value.0 as f64;
                let mean = sum / counts;
                *deltas.entry(String::from(group_val).to_owned()).or_default() += (col_val.to_f64().unwrap() - mean).powf(2.0);
            }
            _ => {
                {};
            }
        }
    }
    // convert total distances to mean to standard deviation
    let mut stds: HashMap<String, f64> = HashMap::new();

    for (key, value) in deltas.into_iter() {
        let nb_unique = &counts.get(&key).unwrap().0;
        *stds.entry(String::from(key).to_owned()).or_default() += (value / (*nb_unique as f64)).sqrt();
    }

    println!("{:?}", stds);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    // create results csv with header only
    write_to_file_header(&result_filename, &groupby_col,(&count_col).to_string());

    // Iterate 3rd time through rows to calculate zscores on the fly and export into results csv
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&result_filename)
            .unwrap();

    let mut writer = csv::Writer::from_writer(file);

    for line in lines {
        let record = line.unwrap();
        let record: Vec<&str> = record.split(',').collect();
        let group_val = record[*col_indices.get(groupby_col).unwrap()].to_string();
        let col_val = Decimal::from_str(record[*col_indices.get(count_col).unwrap()]).unwrap_or_else(|_| Decimal::new(0, 0));

        let group_hash = counts.get(&group_val);
        let mut zscore: f64 = 0.0;
        match group_hash {
            Some(value) => {
            // calculate total of deltas from individual values to group mean
                let sum: f64 = value.1.to_f64().unwrap();
                let mean = sum / value.0 as f64;
                let std = stds.get(&group_val);
                zscore = (col_val.to_f64().unwrap() - mean) / std.unwrap();
                let zscore_str: String = zscore.to_string();
                writer.write_record(&[
                        &group_val.to_string(),
                        &col_val.to_string(),
                        &zscore_str,
                    ]);
            }
            _ => {
                {};
            }
        }

    }
    writer.flush();
    let elapsed = now.elapsed();
            println!("Elapsed: {:.2?}", elapsed);
}
