use std::collections::{HashMap, VecDeque};

fn person_is_seller(name: &str) -> bool {
    name.contains('m')
}

fn search(graph: &HashMap<&str, Vec<&str>>, name: &str) -> bool {
    let mut search_queue = VecDeque::new();
    search_queue.push_back(name);
    let mut searched: Vec<&str> = Vec::new();

    while !search_queue.is_empty() {
        let person = search_queue.pop_front().unwrap();
        if !searched.contains(&person) {
            if person_is_seller(person) {
                println!("{person} is a mango seller!");
                return true;
            } else {
                for &idx in &graph[person] {
                    search_queue.push_back(idx);
                }
                searched.push(person);
            }
        }
    }
    println!("mango seller not found");
    return false;
}

fn main() {
    let mut graph = HashMap::new();
    graph.insert("you", vec!["alice", "bob", "claire"]);
    graph.insert("bob", vec!["anuj", "peggy"]);
    graph.insert("alice", vec!["peggy"]);
    graph.insert("claire", vec!["thom", "jonny"]);
    graph.insert("anuj", vec![]);
    graph.insert("peggy", vec![]);
    graph.insert("thom", vec![]);
    graph.insert("jonny", vec![]);

    search(&graph, "you");
}
