use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let now = Instant::now();
    let mut fwd: HashMap<String, HashSet<String>> = HashMap::new();

    // Open the CSV file
    let file = File::open("/home/thomas/Desktop/phone_numbers_shared.csv")?;
    let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(file);

    // Read the CSV data row by row
    for result in rdr.records() {
        let record = result?;
        if record.len() >= 2 {
            let a = record[0].to_string();
            let b = record[1].to_string();
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
