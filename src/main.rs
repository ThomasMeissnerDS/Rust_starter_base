/*
Script that reads a csv in a loop. Takes a column and counts the number of distinct values within
that column. Accepts three arguments:
- the column to groupby on
- the column name that shall be counted
- path to the csv file (has to include .csv)
 */
use::std::collections::HashMap;
use std::env;
use csv::ReaderBuilder;
use csv::Error;
use std::time::Instant;

fn check_or_create_groupby_entry(key: String, hashmap: &mut HashMap<String, &mut HashMap<String, f64>>) {
    let exists: bool = hashmap.contains_key(&key);
    match exists {
        true => {
        }
        false => {
            let mut new_map: HashMap<String, f64> = HashMap::new();
            new_map.insert(String::from("count"), 0.0);
            new_map.insert(String::from("sum"), 0.0);
            new_map.insert(String::from("mean"), 0.0);
            hashmap.insert(key, &mut new_map);
        }
    }
}

fn update_col_statistics(key: String, hashmap: &mut HashMap<String, &mut HashMap<String, f64>>, value: f64) {
    let col_count: f64 = hashmap.get(&key).unwrap().get("count").unwrap() + 1.0;
    let col_sum: f64 = hashmap.get(&key).unwrap().get("sum").unwrap() + value;
    let col_mean: f64 = col_sum / col_count;

    hashmap.get(&key).unwrap().to_owned().insert(String::from("count"), col_count);

}

fn main() -> Result<(), Error> {
    let now = Instant::now();
    // get command line arguments
    let args: Vec<String> = env::args().collect();
    let groupby_col = String::from(&args[1]); // the column to groupby for
    let count_col = String::from(&args[2]); // the column name that shall be counted
    let csv = String::from(&args[3]); // path to the csv file (has to include .csv)

    let mut col_indices: HashMap<String, usize> = HashMap::new();
    let mut groups_map: HashMap<String, &mut HashMap<String, f64>> = HashMap::new();

    let mut reader = ReaderBuilder::new()
        .flexible(true)
        .delimiter(b',')
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
            println!("{}", {&group_val});
            check_or_create_groupby_entry(group_val, &mut groups_map);

            //println!("{:?}", {groups_map.clone()});


            let col_val: Option<&str> = Some(&record[col_indices.get(&count_col).unwrap_or_else(||&99999).to_owned()]); // get value of count column
            match col_val {
                Some("") => { // this happens if row value is Null
                    let col_val = String::from(&record[*col_indices.get(&count_col).unwrap_or_else(||&0)]);
                    println!("NUUUUULLLLL");
                    println!("{}", {&col_val});
                    let new_val: f64 = 0.0;
                    update_col_statistics(count_col.clone(), &mut groups_map, new_val);
                }
                Some(_str) => {
                    let col_val = String::from(&record[*col_indices.get(&count_col).unwrap_or_else(||&0)]);
                    println!("VALUE FOUND");
                    println!("{}", {&col_val});
                    let new_val: f64 = col_val.parse::<f64>().unwrap();
                    update_col_statistics(count_col.clone(), &mut groups_map, new_val);
                }
                None => { // this happens if row value is Null
                     let col_val = String::from(&record[*col_indices.get(&count_col).unwrap_or_else(||&0)]);
                     println!("NUUUUULLLLL");
                     println!("{}", {&col_val});
                     let new_val: f64 = 0.0;
                     update_col_statistics(count_col.clone(), &mut groups_map, new_val);
                }
            }


            //update_col_statistics(col_val, &mut groups_map, new_val);
            }
        }
        // get all rows except header
        row_idx += 1;
    }
    let elapsed = now.elapsed();
            println!("Elapsed: {:.2?}", elapsed);
    Ok(())
}
