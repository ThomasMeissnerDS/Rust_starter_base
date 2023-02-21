use::std::collections::HashMap;
use csv::Reader;
use csv::ReaderBuilder;
use csv::Error;



// 1) accept an argument with file path to csv + 2) argument for column name to count

// hashmap to store the counts

// loop through csv
// logic for the header
// delimit/split each row
// get context of target columns

// return the length of key





fn main() -> Result<(), Error> {
    let mut col_indices: HashMap<String, usize> = HashMap::new();
    let mut col_counts: HashMap<String, u32> = HashMap::new();

    let count_col: String = String::from("    year"); // will be replaced by command line args

    let csv = "year,make,model,description
    year,make,model,description
    1948,Porsche,356,Luxury sports car
    1967,Ford,Mustang fastback 1967,American car
    ";

    let mut reader = ReaderBuilder::new()
        .flexible(true)
        .has_headers(true)
        .from_reader(csv.as_bytes());
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
            println!("+++++++++");
            println!("{:?}", col_indices);

            let temp_str = String::from(&record[*col_indices.get(&count_col).unwrap()]); // convert to String to be able to deep copy
            *col_counts.entry(temp_str.to_owned()).or_default() += 1;

        }

        row_idx += 1;
    }
    println!("{:?}", col_counts);
    println!("The distinct number of {} categories is {}", count_col, col_counts.keys().len());
    Ok(())
}
