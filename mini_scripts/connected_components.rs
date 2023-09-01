use std::collections::{HashMap, HashSet};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};


fn load_mappings_from_csv(filename: &str, entity_col: &str, identity_col: &str) -> (HashMap<u8, str>, HashMap<u8, str>) {

    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // Get header row and determine column indices for groupby and count columns
    let header_row = lines.next().unwrap().unwrap();
    let headers: Vec<&str> = header_row.split(',').collect();

    // store the index of every column so we find which index to parse
    let mut col_indices = HashMap::new();
    col_indices.insert(groupby_col.clone(), headers.iter().position(|&x| x == groupby_col).unwrap());
    col_indices.insert(count_col.clone(), headers.iter().position(|&x| x == count_col).unwrap());

    let mut entity_to_identifier: HashMap::new();
    let mut identity_to_identifier: Hashmap::new();
    for line in lines {
        let record = line.unwrap();
        let record: Vec<&str> = record.split(',').collect();
        let entity_val = record[*col_indices.get(entity_col).unwrap()].to_string();
        let identifier_val = Decimal::from_str(record[*col_indices.get(identity_col).unwrap()]).to_string();

    }

    return entity_to_identifier, identity_to_identifier
}

fn first_hop(entity_to_identifier: HashMap<u8, str>, identity_to_identifier: HashMap<u8, str>) -> HashMap<str, Vec<str>>{
    /* let mut my_vector: Vec<i32> = vec![0, 1, 2, 3, 4, 5];*/
    let entity_to_entity: HashMap::new();
    for (entity, identifier) in &*entity_to_identifier {
        let identifier_match = entity_to_identifier.get(&entity);
        // check if identifier_match is None
    }
}

fn main() {
    let filename = &env::args().nth(1).expect("file_name not provided");
    let entity_col= &env::args().nth(2).expect("Missing index of entity column");
    let identity_col = &env::args().nth(3).expect("Missing index of entity column");

    (entity_to_identifier, identity_to_identifier) = load_mappings_from_csv(filename, entity_col, identity_col);
}
