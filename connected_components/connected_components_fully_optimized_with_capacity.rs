use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

fn load_mappings_from_csv(filename: &str, entity_col: &str, identity_col: &str) -> (Vec<String>, Vec<String>) {
    let file = File::open(filename).expect("Could not open file");
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();

    let header_row = lines.next().expect("No header row").expect("Error reading header");
    let headers: Vec<&str> = header_row.split(',').collect();

    let entity_index = headers.iter().position(|&x| x == entity_col).expect("Entity column not found");
    let identity_index = headers.iter().position(|&x| x == identity_col).expect("Identity column not found");

    let mut vec_entities = Vec::new();
    let mut vec_identifiers = Vec::new();

    for line in lines {
        let record = line.expect("Error reading line");
        let record: Vec<&str> = record.split(',').collect();

        let entity_val = String::from(record[entity_index]);
        let identifier_val = String::from(record[identity_index]);

        vec_entities.push(entity_val);
        vec_identifiers.push(identifier_val);
    }

    (vec_entities, vec_identifiers)
}

fn first_hop<'a>(vec_entities: &'a Vec<String>, vec_identifiers: &'a Vec<String>, capacity: usize) -> HashMap<&'a str, Vec<&'a str>> {
    let mut entity_to_identifier: HashMap<&str, Vec<&str>> = HashMap::with_capacity(capacity);
    let mut identifier_to_entity: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut entity_to_entity: HashMap<&str, HashSet<&str>> = HashMap::with_capacity(capacity); // Use HashSet for deduplication

    for i in 0..vec_entities.len() {
        let entity_key = &vec_entities[i];
        let identifier = &vec_identifiers[i];

        // fill entities mapping
        entity_to_identifier.entry(entity_key.as_str()).or_insert_with(Vec::new).push(identifier.as_str());

        // fill identifiers mapping
        identifier_to_entity.entry(identifier.as_str()).or_insert_with(Vec::new).push(entity_key.as_str());

        // fulfill the 1st hop
        if let Some(entity_set) = entity_to_entity.get_mut(entity_key.as_str()) {
            if let Some(entity_vec) = identifier_to_entity.get(identifier.as_str()) {
                entity_set.extend(entity_vec.iter().cloned());
            } else {
                entity_set.insert(entity_key.as_str());
            }
        } else {
            if let Some(entity_vec) = identifier_to_entity.get(identifier.as_str()) {
                let entity_set: HashSet<&str> = entity_vec.iter().cloned().collect();
                entity_to_entity.insert(entity_key.as_str(), entity_set);
            }
        }
    }

    // Filter and collect only non-empty sets where k is not the only value in v
    let entity_to_entity: HashMap<&str, Vec<&str>> = entity_to_entity
        .into_iter()
        .filter(|(k, v)| {
            !v.is_empty() &&
            v.len() > 1 ||
            v.iter().any(|&x| x != *k)
        })
        .map(|(k, v)| (k, v.into_iter().collect()))
        .collect();

    entity_to_entity
}

fn multihop_iter<'a>(
    mut entity_to_entity: HashMap<&'a str, Vec<&'a str>>,
    mut shared_entities_length: HashMap<&'a str, usize>,
    capacity: usize
) -> (HashMap<&'a str, Vec<&'a str>>, HashMap<&'a str, usize>, bool) {
    let mut entity_to_entity_enhanced: HashMap<&str, HashSet<&str>> = HashMap::with_capacity(capacity);
    let mut any_chain_got_longer: bool = false;

    for (entity, shared_entities) in entity_to_entity.clone().into_iter() {
        let mut all_entities: HashSet<&str> = HashSet::new();

        for shared_entity in shared_entities.iter() {
            if let Some(entity_vec) = entity_to_entity.get(shared_entity) {
                all_entities.extend(entity_vec.iter().cloned());
            }
            if let Some(shared_entity_vec) = entity_to_entity.get(entity) {
                all_entities.extend(shared_entity_vec.iter().cloned());
            }
            if let Some(already_added_set) = entity_to_entity_enhanced.get(entity) {
                all_entities.extend(already_added_set.iter().cloned());
            }
            if let Some(already_added_set) = entity_to_entity_enhanced.get(shared_entity) {
                all_entities.extend(already_added_set.iter().cloned());
            }

            entity_to_entity_enhanced.insert(entity, all_entities.clone());
            entity_to_entity_enhanced.insert(shared_entity, all_entities.clone());

            let chain_size_before = shared_entities_length.get(shared_entity).cloned().unwrap_or(0);
            let chain_size_after = all_entities.len();
            shared_entities_length.insert(shared_entity, chain_size_after);

            if chain_size_after > chain_size_before {
                any_chain_got_longer = true;
            }
        }
    }

    let entity_to_entity_enhanced: HashMap<&str, Vec<&str>> = entity_to_entity_enhanced
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().collect()))
        .collect();

    (entity_to_entity_enhanced, shared_entities_length, any_chain_got_longer)
}


fn main() {
    let now = Instant::now();
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: {} <filename> <entity_col> <identity_col>", &args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let entity_col = &args[2];
    let identity_col = &args[3];

    println!("Start storing csv data in vectors.");
    let nodes_edges = load_mappings_from_csv(filename, entity_col, identity_col);
    let vec_entities: Vec<String> = nodes_edges.0;
    let vec_identifiers: Vec<String> = nodes_edges.1;
    let capacity: usize = vec_entities.len();

    println!("Calculate first hop");
    let mut entity_to_entity = first_hop(&vec_entities, &vec_identifiers, capacity);

    // executing the first hop
    let mut any_chain_got_longer: bool = true;
    let mut shared_entities_length: HashMap<&str, usize> = HashMap::with_capacity(capacity);

    while any_chain_got_longer {
        println!("Calculate iteration in multihop");
        let result = multihop_iter(entity_to_entity.clone(), shared_entities_length.clone(), capacity);
        entity_to_entity = result.0;
        shared_entities_length = result.1;
        any_chain_got_longer = result.2;
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

}

