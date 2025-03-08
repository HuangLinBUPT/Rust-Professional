use std::collections::{HashMap, HashSet};
use std::fs;

fn parse_json_manually(content: &str) -> HashMap<String, Vec<(String, Vec<String>)>> {
    let mut result = HashMap::new();
    let mut current_batch = String::new();
    let mut current_city = String::new();
    let mut inside_batch = false;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }

        if line.starts_with("{") || line.starts_with("}") {
            continue;
        }
        
        if line.starts_with("\"") {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.is_empty() { continue; }
            
            let key = parts[0].trim_matches(&['"', ' ', ',']);
            
            if parts.len() > 1 && parts[1].trim().starts_with("{") {
                // This is a batch number
                current_batch = key.to_string();
                inside_batch = true;
            } else if inside_batch {
                // This is a city
                current_city = key.to_string();
                
                if line.contains('[') {
                    let neighbors: Vec<String> = line
                        .split('[')
                        .nth(1)
                        .unwrap_or("")
                        .trim_matches(&[']', ',', ' '])
                        .split(',')
                        .map(|s| s.trim_matches(&['"', ' ']))
                        .filter(|s| !s.is_empty())
                        .map(String::from)
                        .collect();

                    result.entry(current_batch.clone())
                        .or_insert_with(Vec::new)
                        .push((current_city.clone(), neighbors));
                }
            }
        }
    }

    result
}

pub fn count_provinces() -> String {
    let content = fs::read_to_string("district.json").unwrap();
    println!("\nRaw JSON content:");
    println!("{}", content);
    
    let data = parse_json_manually(&content);
    println!("\nParsed structure:");
    println!("{:#?}", data);
    
    let mut results = vec!["0".to_string(); 5];
    
    // Process each batch number that exists in the data
    for (batch_str, connections) in &data {
        if let Ok(batch_num) = batch_str.parse::<usize>() {
            println!("\nProcessing batch {}", batch_num);
            
            let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
            
            // Build graph from all connections including duplicates
            for (city, neighbors) in connections {
                for neighbor in neighbors {
                    graph.entry(city.clone())
                        .or_insert_with(HashSet::new)
                        .insert(neighbor.clone());
                    graph.entry(neighbor.clone())
                        .or_insert_with(HashSet::new)
                        .insert(city.clone());
                }
            }

            println!("\nFinal graph structure:");
            for (city, neighbors) in &graph {
                println!("{}: {:?}", city, neighbors);
            }

            let mut visited = HashSet::new();
            let mut province_count = 0;

            for city in graph.keys() {
                if !visited.contains(city) {
                    let mut province_cities = HashSet::new();
                    dfs(city, &graph, &mut visited, &mut province_cities);
                    println!("Province {}: {:?}", province_count + 1, province_cities);
                    province_count += 1;
                }
            }
            
            results[batch_num as usize - 1] = province_count.to_string();
        }
    }

    results.join(",")
}

fn dfs(
    city: &str, 
    graph: &HashMap<String, HashSet<String>>, 
    visited: &mut HashSet<String>,
    province_cities: &mut HashSet<String>
) {
    if !visited.insert(city.to_string()) {
        return;
    }
    
    province_cities.insert(city.to_string());
    
    if let Some(neighbors) = graph.get(city) {
        for neighbor in neighbors {
            if !visited.contains(neighbor) {
                dfs(neighbor, graph, visited, province_cities);
            }
        }
    }
}
