use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use csv::ReaderBuilder;
use std::time::Instant;
use std::env::args;
use std::env;

// Union-Find data structure for efficient transitive closure computation
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        let mut parent = vec![0; size];
        let rank = vec![0; size];
        for i in 0..size {
            parent[i] = i;
        }
        UnionFind { parent, rank }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            if self.rank[root_x] > self.rank[root_y] {
                self.parent[root_y] = root_x;
            } else {
                self.parent[root_x] = root_y;
                if self.rank[root_x] == self.rank[root_y] {
                    self.rank[root_y] += 1;
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let now = Instant::now();

    // HashMap to store mappings from entity strings to unique integers
    let mut entity_to_int: HashMap<String, usize> = HashMap::new();
    let mut next_entity_int = 0;

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
    let mut fwd: HashMap<usize, HashSet<usize>> = HashMap::new();

    for line in lines {
        let record = line.expect("Error reading line");
        let record: Vec<&str> = record.split(',').collect();
        if record.len() >= 2 {
            let entity_str = record[entity_index].to_string();
            let identity_str = record[identity_index].to_string();

            // Assign unique integers to entity strings
            let entity_int = *entity_to_int.entry(entity_str.clone()).or_insert_with(|| {
                let int = next_entity_int;
                next_entity_int += 1;
                int
            });

            // Insert the integers into the graph
            fwd.entry(entity_int).or_insert_with(HashSet::new).insert(entity_int);

            let identity_int = *entity_to_int.entry(identity_str.clone()).or_insert_with(|| {
                let int = next_entity_int;
                next_entity_int += 1;
                int
            });

            // Insert the integers into the graph
            fwd.entry(identity_int).or_insert_with(HashSet::new).insert(identity_int);

            // Union entities and identities in the Union-Find data structure
            fwd.entry(entity_int).or_insert_with(HashSet::new).insert(identity_int);
            fwd.entry(identity_int).or_insert_with(HashSet::new).insert(entity_int);
        }
    }

    println!("CSV data has been read");

    let mut entity_to_entity: HashMap<usize, HashSet<usize>> = HashMap::new();

    // Build the entity-to-entity mapping using the Union-Find data structure
    let mut union_find = UnionFind::new(next_entity_int);
    for &entity in fwd.keys() {
        let mut related_set: HashSet<usize> = HashSet::new();
        for &related_entity in &fwd[&entity] {
            let root = union_find.find(related_entity);
            related_set.insert(root);
        }
        entity_to_entity.insert(entity, related_set);
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    // Print or process the entity-to-entity mapping
    // You can translate integers back to strings using the entity_to_int mapping.
    // Print or process the entity-to-entity mapping
    for (entity_int, related_entities_int) in &entity_to_entity {
        // Translate the integer back to the entity string
        let entity_str: String = entity_to_int
            .iter()
            .find(|(_, &int)| int == *entity_int)
            .map(|(str, _)| str.clone())
            .unwrap_or_else(|| format!("Unknown entity: {}", entity_int));

        // Translate related integers back to entity strings
        let related_entities_str: HashSet<String> = related_entities_int
            .iter()
            .filter_map(|&related_int| {
                entity_to_int
                    .iter()
                    .find(|(_, &int)| int == related_int)
                    .map(|(str, _)| str.clone())
            })
            .collect();

        println!("Entity: {}, Related Entities: {:?}", entity_str, related_entities_str);
    }


    Ok(())
}
