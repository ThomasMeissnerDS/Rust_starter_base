use csv;
use rayon::prelude::*;
use rust_decimal::prelude::*;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fs::OpenOptions;
use std::sync::{Arc, RwLock, RwLockReadGuard};
use std::time::Instant;
use std::thread::available_parallelism;


fn read_process_file_subset(filename: &str, groupby_col: &String, count_col: &String, cpu_core: u32, total_cores: u32) -> HashMap<String, (i32, Decimal)>{
    /*
    Every cpu core reads the csv separately and skips all rows except the ones
    of it's subset. Results are merged afterwards.
    */

    // Read CSV file
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut row_idx: u32 = cpu_core;

    // Get header row and determine column indices for groupby and count columns
    let header_row = lines.next().unwrap().unwrap();
    let headers: Vec<&str> = header_row.split(',').collect();
    let mut col_indices = HashMap::new();
    col_indices.insert(&groupby_col, headers.iter().position(|&x| x == groupby_col).unwrap());
    col_indices.insert(&count_col, headers.iter().position(|&x| x == count_col).unwrap());

    // Iterate 1st time through rows to get meta data of reference categories of zscores

    // Loop through remaining rows and accumulate counts, sum, and distinct values for each group
    let mut counts = HashMap::new();
    for line in lines {
        match row_idx % total_cores == cpu_core { // every core handles a different subset of data
            true => {
                let record = line.unwrap();
                let record: Vec<&str> = record.split(',').collect();
                let group_val = record[*col_indices.get(&groupby_col).unwrap()].to_string();  // TODO: ADD match statement
                let col_val = Decimal::from_str(record[*col_indices.get(&count_col).unwrap()]).unwrap_or_else(|_| Decimal::new(0, 0));
                let (count, sum) = counts.entry(group_val.clone()).or_insert((0, Decimal::new(0, 0)));
                *count += 1;
                *sum += col_val;
            }
            _ => {
                ();
            }
        }

        row_idx += 1;

    }
    return counts
}

fn get_total_deltas_subset(filename: &str, groupby_col: &String, count_col: &String, cpu_core: u32, total_cores: u32,
                           counts: RwLockReadGuard<HashMap<String, (i32, Decimal)>>, col_indices: HashMap<&String, usize>) -> HashMap<String, f64> {
    // Iterate 2nd time through rows to get standard deviation of reference categories of zscores
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut row_idx: u32 = cpu_core;

    let mut deltas:HashMap<String, f64> = HashMap::new();
    for line in lines {
        match row_idx % total_cores == cpu_core { // every core handles a different subset of data
            true => {
                let record = line.unwrap();
                let record: Vec<&str> = record.split(',').collect();
                let group_val = record[*col_indices.get(&groupby_col).unwrap()].to_string();
                //println!("{}", &group_val);
                let col_val = Decimal::from_str(&*record[*col_indices.get(&count_col).unwrap()]).unwrap_or_else(|_| Decimal::new(0, 0));

                let group_hash = counts.get(&group_val);
                match group_hash {
                    Some(value) => {
                        // calculate total of deltas from individual values to group mean
                        let sum: f64 = value.1.to_f64().unwrap();
                        let counts: f64 = value.0 as f64;
                        let mean = sum / counts;
                        *deltas.entry(String::from(group_val).to_owned()).or_default() += (col_val.to_f64().unwrap() - mean).powf(2.0);
                    }
                    _ => {
                        {};
                    }
                }
            }
            _ => {
                ();
            }
        }
        row_idx += 1;
    }
    return deltas
}


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
    let available_cores: u32 = available_parallelism().unwrap().get() as u32;  // get info how many threads we can use


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
    let range: Vec<u32> = (0..available_cores).collect();
    let mut results = Vec::new();
    let mut threads = Vec::new();
    for thread_idx in range {
        let f_name = filename.clone();
        let groupby_col = &env::args().nth(1).expect("groupby_col not provided");
        let count_col = &env::args().nth(2).expect("count_col not provided");
        threads.push(std::thread::spawn(move || {
            read_process_file_subset(&f_name, &groupby_col, &count_col, thread_idx, available_cores)
        }));
    }

    for thread in threads {
        results.extend(thread.join());
    };

    // join results of chunks back together
    let mut counts:HashMap<String, (i32, rust_decimal::Decimal)> = HashMap::new();
    for result in results {
        for (key, value) in result {
            let (count, sum) = counts.entry(key).or_insert((0, Decimal::new(0, 0)));
            *count += value.0;
            *sum += value.1;
        }
    }


    // Iterate 2nd time through rows to get standard deviation of reference categories of zscores
    let range: Vec<u32> = (0..available_cores).collect();
    let mut results = Vec::new();
    let mut threads = Vec::new();
    let counts_shared = Arc::new(RwLock::new(counts));
    let counts_shared_clone = counts_shared.clone();

    for thread_idx in range {
        let f_name = filename.clone();
        let groupby_col = &env::args().nth(1).expect("groupby_col not provided");
        let count_col = &env::args().nth(2).expect("count_col not provided");

        threads.push(std::thread::spawn(move || {
            get_total_deltas_subset(&f_name, &groupby_col,
                                    &count_col, thread_idx,
                                    available_cores, counts_shared_clone.read().unwrap(),
                                    col_indices.clone())
        }));
    }

    for thread in threads {
        results.extend(thread.join());
    };

    let counts = counts_shared_clone.read().unwrap();


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

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    // create results csv with header only
    write_to_file_header(&result_filename, &groupby_col,(&count_col).to_string());

    // Iterate 3rd time through rows to calculate zscores on the fly and export into results csv
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);
    let lines = reader.lines();

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
                        &groupby_col,
                        &count_col,
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
