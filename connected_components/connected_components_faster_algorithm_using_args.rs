use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use csv::ReaderBuilder;
use std::time::Instant;
use std::env::args;
use std::env;


fn main() -> Result<(), Box<dyn Error>> {
    let now = Instant::now();
    let mut fwd: HashMap<String, HashSet<String>> = HashMap::new();

    let args: Vec<String> = env::args().collect();

    // Open the CSV file
    if args.len() < 4 {
            eprintln!("Usage: {} <filename> <entity_col> <identity_col>", &args[0]);
            std::process::exit(1);
        }

    let filename = &args[1];
    let entity_col = &args[2];
    let identity_col = &args[3];

    let file = File::open(filename).expect("Could not open file");
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();

    let header_row = lines.next().expect("No header row").expect("Error reading header");
    let headers: Vec<&str> = header_row.split(',').collect();

    let entity_index = headers.iter().position(|&x| x == entity_col).expect("Entity column not found");
    let identity_index = headers.iter().position(|&x| x == identity_col).expect("Identity column not found");

    // Read the CSV data row by row
    for line in lines {
        let record = line.expect("Error reading line");
        let record: Vec<&str> = record.split(',').collect();
        if record.len() >= 2 {
            let a = record[entity_index].to_string();
            let b = record[identity_index].to_string();
            if !a.is_empty() && !b.is_empty() && a != b {
                fwd.entry(a.clone()).or_insert_with(HashSet::new).insert(b.clone());
                fwd.entry(b.clone()).or_insert_with(HashSet::new).insert(a.clone());
            }
        }
    }
    println!("CSV data has been read");

    let mut entity_to_entity: HashMap<String, HashSet<String>> = HashMap::new();

    // Function to find the transitive closure of entities
    fn transitive_closure(entity: &str, fwd: &HashMap<String, HashSet<String>>, related_set: &mut HashSet<String>) {
        if let Some(related_entities) = fwd.get(entity) {
            for related_entity in related_entities {
                if related_set.insert(related_entity.clone()) {
                    transitive_closure(related_entity, fwd, related_set);
                }
            }
        }
    }

    // Build the entity-to-entity mapping
    for entity in fwd.keys() {
        let mut related_set: HashSet<String> = HashSet::new();
        transitive_closure(entity, &fwd, &mut related_set);
        entity_to_entity.insert(entity.clone(), related_set.clone());
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    // Print or process the entity-to-entity mapping
    //for (entity, related_entities) in &entity_to_entity {
    //    if !entity.chars().all(char::is_numeric) {
    //        let related_non_numeric: HashSet<String> = related_entities
    //            .iter()
    //            .filter(|&e| !e.chars().all(char::is_numeric))
    //            .cloned()
    //            .collect();
    //        println!("{}: {:?}", entity, related_non_numeric);
    //    }
    //}

    Ok(())
}
