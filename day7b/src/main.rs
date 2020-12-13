use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::io::BufRead;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Bag(String, String);

fn index_for(index_map: &mut HashMap<Bag, usize>, bag: &Bag) -> usize {
    if let Some(&idx) = index_map.get(bag) {
        return idx;
    } else {
        let idx = index_map.len();
        index_map.insert(bag.clone(), idx);
        return idx;
    }
}

fn amount_for(
    contains: &HashMap<usize, Vec<(usize, usize)>>,
    cache: &mut HashMap<usize, usize>,
    idx: usize,
) -> usize {
    if let Some(&amount) = cache.get(&idx) {
        amount
    } else {
        let mut amount = 0;
        if let Some(contents) = contains.get(&idx) {
            for &(other_idx, times) in contents {
                amount += (amount_for(contains, cache, other_idx) + 1) * times;
            }
        }
        cache.insert(idx, amount);
        amount
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();

    let mut contains: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    let mut index_map: HashMap<Bag, usize> = HashMap::new();

    for line_result in stdin.lock().lines() {
        if let Ok(line) = line_result {
            let line = line.trim();
            if line.len() > 0 {
                if let [out, contents, ..] = line.split(" contain ").collect::<Vec<&str>>()[..] {
                    let out: Vec<&str> = out.split(" ").collect();
                    let out = Bag(out[0].to_owned(), out[1].to_owned());
                    let out_idx = index_for(&mut index_map, &out);

                    let mut this_contains = Vec::new();

                    for content in contents.split(", ") {
                        let parts: Vec<&str> = content.split(" ").collect();
                        if parts.len() == 4 {
                            let content = Bag(parts[1].to_owned(), parts[2].to_owned());
                            let content_idx = index_for(&mut index_map, &content);
                            let amount = parts[0].parse().unwrap();
                            this_contains.push((content_idx, amount));
                        }
                    }

                    contains.insert(out_idx, this_contains);
                }
            }
        }
    }

    let start = Bag("shiny".to_owned(), "gold".to_owned());
    let start_idx = index_for(&mut index_map, &start);

    let mut cache = HashMap::new();

    println!("{}", amount_for(&contains, &mut cache, start_idx));
    return Ok(());
}
