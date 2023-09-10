use std::collections::{HashMap, HashSet};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn load_mappings_from_csv(filename: &str, entity_col: &str, identity_col: &str) -> (Vec<String>, Vec<String>){
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let header_row = lines.next().unwrap().unwrap();
    let headers: Vec<&str> = header_row.split(',').collect();

    let mut col_indices = HashMap::new();
    col_indices.insert(entity_col.clone(), headers.iter().position(|&x| x == entity_col).unwrap());
    col_indices.insert(identity_col.clone(), headers.iter().position(|&x| x == identity_col).unwrap());

    let mut vec_entities: Vec<String> = vec![];
    let mut vec_identifiers: Vec<String> = vec![];

    for line in lines {
        let record = line.unwrap();
        let record: Vec<&str> = record.split(',').collect();

        let entity_val = String::from(record[*col_indices.get(entity_col).unwrap()]);
        let identifier_val = String::from(record[*col_indices.get(identity_col).unwrap()]);


        vec_entities.push(entity_val);
        vec_identifiers.push(identifier_val);
    }
    return (vec_entities, vec_identifiers)
    // Now you can use vec_entities and vec_identifiers as needed.
}

fn first_hop<'a>(vec_entities: &'a Vec<String>, vec_identifiers: &'a Vec<String>) -> HashMap<String, Vec<&'a str>>{
    // In this part we do the the first hop
    let mut entity_to_identifier: HashMap<String, Vec<&str>> = HashMap::new();
    let mut identifier_to_entity: HashMap<String, Vec<&str>> = HashMap::new();
    let mut entity_to_entity: HashMap<String, Vec<&str>> = HashMap::new();
    for i in 0..vec_entities.len() {
        let entity_key = vec_entities[i].clone(); // Clone the String, otherwise move happens
        let identifier = vec_identifiers[i].clone();

        // fill entities mappping
        if let Some(vec) = entity_to_identifier.get_mut(&entity_key) {
            vec.push(&vec_identifiers[i]);
        } else {
            let mut identifiers: Vec<&str> = vec![];
            identifiers.push(&vec_identifiers[i]);
            entity_to_identifier.insert(entity_key, identifiers);
        }

        let entity_key = vec_entities[i].clone(); // Clone the String, otherwise move happens
        let identifier = vec_identifiers[i].clone();

        // fill identifiers mapping
        if let Some(vec) = identifier_to_entity.get_mut(&identifier) {
            vec.push(&vec_entities[i]);
        } else {
            let mut entities: Vec<&str> = vec![];
            entities.push(&vec_entities[i]);
            identifier_to_entity.insert(identifier, entities);
        }

        let entity_key = vec_entities[i].clone(); // Clone the String, otherwise move happens
        let identifier = vec_identifiers[i].clone();

        // fulfill the 1st hop
        let mut entities_to_append: Vec<&str> = vec![];
        if let Some(vec) = identifier_to_entity.get_mut(&identifier) {
            let mut vec_clone = vec.clone(); // Clone the original vec
            for entity_shared in &mut *vec {
                let mut entities_original: Vec<&str> = vec![];
                if let Some(entity_vec) = entity_to_entity.get_mut(&entity_shared as &str) {
                    entities_original.append(entity_vec);
                    entities_to_append.append(&mut entities_original);
                    vec_clone.append(&mut entities_to_append);
                    entity_to_entity.insert(entity_shared.to_string(), vec_clone.clone());
                }
                else {
                    entities_to_append.append(&mut vec_clone.clone()); // Use the clone
                    entity_to_entity.insert(entity_shared.to_string(), entities_to_append.clone());
                }
                entity_to_entity.get_mut(&entity_shared as &str).expect("Entity not found").sort_unstable();
                entity_to_entity.get_mut(&entity_shared as &str).expect("Entity not found").dedup();
            }
        }
    }
    return entity_to_entity
}

fn multihop_iter<'a>(mut entity_to_entity: HashMap<String, Vec<&'a str>>, mut shared_entities_length: HashMap<&'a str, usize>) -> (HashMap<String, Vec<&'a str>> , HashMap<&'a str, usize>, bool ){
    let mut entity_to_entity_enhanced: HashMap<String, Vec<&str>> = HashMap::new();
    let mut any_chain_got_longer: bool = false;
    for (entity, mut shared_entities) in entity_to_entity.clone().into_iter() {
        let mut all_entities: Vec<&str> = vec![];
        for shared_entity in &mut *shared_entities {
            let mut chain_size_before: usize = 0;

            if let Some(entity_vec) = entity_to_entity.get_mut(&entity as &str) {
                all_entities.append(entity_vec);
            }
            if let Some(shared_entity_vec) = entity_to_entity.get_mut(&shared_entity as &str) {
                all_entities.append(shared_entity_vec);
            }
            // we need this to not overwrite our entries
            if let Some(already_added_vec) = entity_to_entity_enhanced.get_mut(&entity as &str) {
                all_entities.append(already_added_vec);
            }
            if let Some(already_added_vec) = entity_to_entity_enhanced.get_mut(&shared_entity as &str) {
                all_entities.append(already_added_vec);
            }
            all_entities.sort_unstable();
            all_entities.dedup();
            entity_to_entity_enhanced.insert(entity.clone(), all_entities.clone());
            entity_to_entity_enhanced.insert(shared_entity.to_string(), all_entities.clone());

            // checking if a chain got longer and storing chain length
            if let Some(chain_size) = shared_entities_length.get_mut(shared_entity) {
                chain_size_before = *chain_size;
            }
            let mut chain_size_after: usize = all_entities.len();
            shared_entities_length.insert(&shared_entity, chain_size_after);
            // early stopping condition
            if chain_size_after > chain_size_before {
                any_chain_got_longer = true;
            }

        }
    }
    return (entity_to_entity_enhanced, shared_entities_length, any_chain_got_longer)
}

fn main() {
    let now = Instant::now();
    let filename = &env::args().nth(1).expect("file_name not provided");
    let entity_col = &env::args().nth(2).expect("Missing index of entity column");
    let identity_col = &env::args().nth(3).expect("Missing index of identifier column");

    println!("Start storing csv data in vectors.");
    let nodes_edges = load_mappings_from_csv(filename, entity_col, identity_col);
    let vec_entities: Vec<String> = nodes_edges.0;
    let vec_identifiers: Vec<String> = nodes_edges.1;

    println!("Calculate first hop");
    let mut entity_to_entity: HashMap<String, Vec<&str>> = HashMap::new();
    entity_to_entity = first_hop(&vec_entities, &vec_identifiers);


    // executing the first hop
    let mut any_chain_got_longer: bool = true;
    let mut shared_entities_length: HashMap<&str, usize>= HashMap::new();
    while any_chain_got_longer {
        println!("Calculate iteration in multihop");
        (entity_to_entity, shared_entities_length, any_chain_got_longer) = multihop_iter(entity_to_entity, shared_entities_length);
    }
    //println!("{:?}", entity_to_entity);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
