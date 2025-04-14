use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::Dfs;
use petgraph::dot::{Dot, Config};
use std::fs::File;
use std::io::Write;

pub struct MyGraph {
    graph: Graph<&'static str, ()>,
    node1: NodeIndex,
}

impl MyGraph {
    pub fn new() -> Self {
        let mut graph = Graph::<&'static str, ()>::new();

        let n1 = graph.add_node("1");
        let n2 = graph.add_node("2");
        let n3 = graph.add_node("3");
        let n4 = graph.add_node("4");
        let n5 = graph.add_node("5");
        let n6 = graph.add_node("6");

        graph.add_edge(n1, n2, ());
        graph.add_edge(n1, n3, ());
        graph.add_edge(n2, n4, ());
        graph.add_edge(n2, n5, ());
        graph.add_edge(n3, n6, ());

        MyGraph { graph, node1: n1 }
    }

    pub fn dfs_from_node1(&self) -> Vec<&'static str> {
        let mut dfs = Dfs::new(&self.graph, self.node1);
        let mut visited = Vec::new();

        while let Some(nx) = dfs.next(&self.graph) {
            visited.push(self.graph[nx]);
        }

        visited
    }

    pub fn export_dot(&self, path: &str) {
        let dot_graph = Dot::with_config(&self.graph, &[Config::EdgeNoLabel]);
        let mut file = File::create(path).expect("o arquivo .dot, não pode ser criado");
        write!(file, "{:?}", dot_graph).expect("arquivo .dot não pode ser escrito");
        println!("Grafo exportado para {}", path);
    }
}

// -----------------------------------------------------------
// TESTES
// -----------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_visits_all_nodes() {
        let g = MyGraph::new();
        let result = g.dfs_from_node1();

        let mut sorted_result = result.clone();
        sorted_result.sort();

        let mut expected = vec!["1", "2", "3", "4", "5", "6"];
        expected.sort();

        assert_eq!(sorted_result, expected, "Todos os nós devem ser visitados");
    }

    #[test]
    fn test_dfs_starts_at_node1() {
        let g = MyGraph::new();
        let result = g.dfs_from_node1();
        assert_eq!(result.first(), Some(&"1"), "DFS deve começar pelo nó 1");
    }
}