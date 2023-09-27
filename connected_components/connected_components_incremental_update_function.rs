use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;
use std::env::args;
use std::env;

struct Result {
    entity_to_set_map: HashMap<String, usize>,
    sets: Vec<HashSet<String>>,
    sets_identifier: Vec<HashSet<String>>,
    identifier_to_set_map: HashMap<String, usize>,
    new_set_id: usize,
}

impl Result {
    fn new() -> Result {
        Result {
            entity_to_set_map: HashMap::new(),
            sets: vec![HashSet::new()],
            sets_identifier: vec![HashSet::new()],
            identifier_to_set_map: HashMap::new(),
            new_set_id: 0,
        }
    }
}

fn incremental_update(
    new_entity: String,
    new_identifier: String,
    result: &mut Result,
) {
    let entity_known = result.entity_to_set_map.contains_key(&new_entity);
    let identifier_known = result.identifier_to_set_map.contains_key(&new_identifier);

    if entity_known && identifier_known {
        if result.identifier_to_set_map[&new_identifier] != result.entity_to_set_map[&new_entity] {
            let old_entity_set_id = result.entity_to_set_map[&new_entity];
            let old_identifier_set_id = result.identifier_to_set_map[&new_identifier];

            let entity_set = std::mem::replace(&mut result.sets[old_identifier_set_id], HashSet::new());
            let identifier_set = std::mem::replace(&mut result.sets_identifier[old_identifier_set_id], HashSet::new());

            result.sets[old_entity_set_id].extend(entity_set.into_iter());
            result.sets_identifier[old_entity_set_id].extend(identifier_set.into_iter());

            for entity in &result.sets[old_entity_set_id] {
                result.entity_to_set_map.insert(entity.clone(), old_entity_set_id);
            }

            for identifier in &result.sets_identifier[old_entity_set_id] {
                result.identifier_to_set_map.insert(identifier.clone(), old_entity_set_id);
            }
        }
    } else if entity_known && !identifier_known {
        let set_id = result.entity_to_set_map[&new_entity];
        result.sets[set_id].insert(new_entity.clone());
        result.sets_identifier[set_id].insert(new_identifier.clone());
        result.identifier_to_set_map.insert(new_identifier, set_id);
    } else if !entity_known && identifier_known {
        let set_id = result.identifier_to_set_map[&new_identifier];
        result.sets[set_id].insert(new_entity.clone());
        result.sets_identifier[set_id].insert(new_identifier.clone());
        result.entity_to_set_map.insert(new_entity.clone(), set_id);
    } else {
        result.new_set_id += 1;
        let new_set_id = result.new_set_id;

        let mut new_set = HashSet::new();
        new_set.insert(new_entity.clone());

        let mut new_identifier_set = HashSet::new();
        new_identifier_set.insert(new_identifier.clone());

        result.sets.push(new_set);
        result.sets_identifier.push(new_identifier_set);

        result.entity_to_set_map.insert(new_entity, new_set_id);
        result.identifier_to_set_map.insert(new_identifier, new_set_id);
    }
}

fn main() -> io::Result<()> {
    let now = Instant::now();
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

    let entity_idx = headers.iter().position(|&x| x == entity_col).expect("Entity column not found");
    let identity_idx = headers.iter().position(|&x| x == identity_col).expect("Identity column not found");

    let mut result = Result::new();


    for line in lines {
        let line = line?;
        let values: Vec<&str> = line.split(',').collect();
        let new_entity = values[entity_idx].to_string();
        let new_identifier = values[identity_idx].to_string();

        incremental_update(new_entity, new_identifier, &mut result);
    }
    //println!("{:?}", result.entity_to_set_map);
    // Calculate the longest chain
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    let longest_chain = result.sets.iter().map(|set| set.len()).max().unwrap_or(0);
    println!("Longest chain: {}", longest_chain);

    Ok(())
}
