/*
WORK IN PROGRESS
- the column to groupby on
- the column name that shall be counted
- path to the csv file (has to include .csv)
 */
use::std::collections::HashMap;
use std::env;
use csv::ReaderBuilder;
use csv::Error;
use std::time::Instant;

fn check_or_create_groupby_entry(key: String, hashmap: &mut HashMap<String, HashMap<String, f64>>) {
    let exists: bool = hashmap.contains_key(&key);
    match exists {
        true => {
            ();
        }
        false => {
            let mut new_map: HashMap<String, f64> = HashMap::new();
            new_map.insert(String::from("count"), 0.0);
            new_map.insert(String::from("sum"), 0.0);
            new_map.insert(String::from("mean"), 0.0);
            hashmap.insert(key, new_map);
        }
    }
}

fn main() -> Result<(), Error> {
    let now = Instant::now();
    // get command line arguments
    let args: Vec<String> = env::args().collect();
    let groupby_col = String::from(&args[1]); // the column to groupby for
    let count_col = String::from(&args[2]); // the column name that shall be counted
    let csv = String::from(&args[3]); // path to the csv file (has to include .csv)

    let mut col_indices: HashMap<String, usize> = HashMap::new();
    let mut groups_map: HashMap<String, HashMap<String, f64>> = HashMap::new();
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
            // check if key exists in groupby_col map, otherwise create new nested hashmap for that key
            let group_val = String::from(&record[*col_indices.get(&groupby_col).unwrap()]);
            check_or_create_groupby_entry(group_val, &mut groups_map);


            }
        }
        // get all rows except header
        row_idx += 1;
    }
    println!("The distinct number of {} categories is {}", count_col, col_counts.keys().len());
    let elapsed = now.elapsed();
            println!("Elapsed: {:.2?}", elapsed);
    Ok(())
}
