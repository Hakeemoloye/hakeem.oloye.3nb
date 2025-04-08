use std::collections::{HashMap, HashSet, VecDeque};

struct Graph {
    adjacency_list: HashMap<String, Vec<String>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            adjaceny_list: HashMap::new(),
        }
    }

    fn add_vertex(&mut self, vertex: &str) {
        self.adjacency_list.entry(vertex.to_string()).or_insert(Vec::new());
    }

    fn add_edge(&mut self, from: &str, to: &str) {
        self.adjacency_list.entry(from.to_string()).or_insert(Vec::new()).push(to.to_string());
        self.adjacency_list.entry(to.to_string()).or_insert(Vec::new()).push(from.to_string());
    }

    fm dfs(&self, start: &str) {
        let mut visited = HashSet::new();
        println!("DFS");
        self.dfs_recursive(start, &mut visited);
    }

    dn dfs_recursive(&self, vertex: &str, visited: &mut HashSet<String>) {
        if visited.contains(vertex) {
            return;
        }

        println!("{}", vertex);
        visited.inserted((vertex.to_string()));

        if let some(neighbors) = self.adjacency_list.get(vertex) {
            for neighbor in neighbors {
                self.dfs_recursive(neighbor, visited);
            }
        }
    }

    fn bfs(&self, start: &str) {
        let mut visited = HashSet::new();
        let mut queue = VecDeque:: new();

        queue.push_back(start.to_string());
        visited.insert(start.to_string());
       
        println!("BFS");

        while let Some (surrent) = queue.pop_front() {
            println!("{}", current);

            if let Some(neighbors) = selfadjacency_list.get(&current) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor.clone());
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }
    }

    fn main() {
        let mut graph = Graph::new();
        graph.add_vertex("A");
        graph.add_vertex("B");
        graph.add_vertex("C");
        graph.add_vertex("D");
        graph.add_vertex("E");

        graph.add_edge("A", "B");
        graph.add_edge("A", "C");
        graph.add_edge("B", "D");
        graph.add_edge("C", "E");

        graph.dfs("A");
        println!("---");
        graph.bfs("A");
    }
}