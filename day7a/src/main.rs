use std::collections::{HashMap, HashSet, VecDeque};
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

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();

    let mut can_be_in: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut index_map: HashMap<Bag, usize> = HashMap::new();

    for line_result in stdin.lock().lines() {
        if let Ok(line) = line_result {
            let line = line.trim();
            if line.len() > 0 {
                if let [out, contents, ..] = line.split(" contain ").collect::<Vec<&str>>()[..] {
                    let out: Vec<&str> = out.split(" ").collect();
                    let out = Bag(out[0].to_owned(), out[1].to_owned());
                    let out_idx = index_for(&mut index_map, &out);

                    for content in contents.split(", ") {
                        let parts: Vec<&str> = content.split(" ").collect();
                        if parts.len() == 4 {
                            let content = Bag(parts[1].to_owned(), parts[2].to_owned());
                            let content_idx = index_for(&mut index_map, &content);
                            can_be_in
                                .entry(content_idx)
                                .or_insert(HashSet::new())
                                .insert(out_idx);
                        }
                    }
                }
            }
        }
    }

    let target = Bag("shiny".to_owned(), "gold".to_owned());
    let target_idx = index_for(&mut index_map, &target);

    let mut count = 0;

    let mut touched: HashSet<usize> = HashSet::new();
    touched.insert(target_idx);
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(target_idx);

    while let Some(idx) = queue.pop_front() {
        if let Some(containers) = can_be_in.get(&idx) {
            for &container in containers {
                if !touched.contains(&container) {
                    count += 1;
                    touched.insert(container);
                    queue.push_back(container)
                }
            }
        }
    }

    println!("{}", count);
    return Ok(());
}
