use std::time::Instant;
use::std::collections::HashMap;
use std::{error::Error, io, process};
use std::fs::File;
use csv::Reader;
use csv::Error;



// 1) accept an argument with file path to csv + 2) argument for column name to count

// hashmap to store the counts

// loop through csv
// logic for the header
// delimit/split each row
// get context of target columns

// return the length of key


fn main() -> Result<(), dyn Error> {
    let csv = "year,make,model,description
        1948,Porsche,356,Luxury sports car
        1967,Ford,Mustang fastback 1967,American car";

    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    for record in reader.records() {
        let record = record?;
        //let nb_cols = record.len();
        //println!("Nb columns is {}", nb_cols);
        println!(
            "In {}, {} built the {} model. It is a {}.",
            &record[0],
            &record[1],
            &record[2],
            &record[3]
        );
    }

    Ok(())
}
