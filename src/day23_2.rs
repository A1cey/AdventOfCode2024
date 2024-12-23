use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Node {
    #[allow(dead_code)]
    name: String,
    edges: Vec<usize>,
}

#[derive(Debug, Default)]
struct Graph {
    nodes: Vec<Node>,
    seen: HashMap<String, usize>,
}

impl Graph {
    fn add_node(&mut self, name: &str) -> usize {
        if let Some(&id) = self.seen.get(name) {
            return id;
        }
        let id = self.nodes.len();
        self.seen.insert(name.to_string(), id);
        self.nodes.push(Node {
            name: name.to_string(),
            edges: vec![],
        });
        id
    }

    fn add_edge(&mut self, a: &str, b: &str) {
        let a_id = self.add_node(a);
        let b_id = self.add_node(b);
        self.nodes[a_id].edges.push(b_id);
        self.nodes[b_id].edges.push(a_id);
    }
}

// Find maximum clique using Bron-Kerbosch algorithm
fn find_max_clique(graph: &Graph) -> HashSet<usize> {
    fn bron_kerbosch(
        graph: &Graph,
        r: &mut HashSet<usize>,
        p: &mut HashSet<usize>,
        x: &mut HashSet<usize>,
        max_clique: &mut HashSet<usize>,
    ) {
        if p.is_empty() && x.is_empty() {
            if r.len() > max_clique.len() {
                max_clique.clear();
                max_clique.extend(r.iter());
            }
            return;
        }

        let pivot = p
            .iter()
            .chain(x.iter())
            .max_by_key(|&&v| {
                // select pivot as the node with the most neighbors in p
                graph.nodes[v]
                    .edges
                    .iter()
                    .cloned()
                    .filter(|&c| p.contains(&c))
                    .count()
            })
            .cloned();

        if let Some(u) = pivot {
            let pivot_neighbors = graph.nodes[u].edges.iter().cloned().collect::<HashSet<_>>();
            let p_copy = p.clone();

            for &v in p_copy.difference(&pivot_neighbors) {
                r.insert(v);
                let v_neighbors: HashSet<_> = graph.nodes[v].edges.iter().cloned().collect();
                let mut new_p = p
                    .intersection(&v_neighbors)
                    .cloned()
                    .collect::<HashSet<_>>();
                let mut new_x = x
                    .intersection(&v_neighbors)
                    .cloned()
                    .collect::<HashSet<_>>();
                bron_kerbosch(graph, r, &mut new_p, &mut new_x, max_clique);
                r.remove(&v);
                p.remove(&v);
                x.insert(v);
            }
        }
    }

    let mut max_clique = HashSet::new();
    let mut r = HashSet::new();
    let mut p: HashSet<_> = (0..graph.nodes.len()).collect();
    let mut x = HashSet::new();

    bron_kerbosch(graph, &mut r, &mut p, &mut x, &mut max_clique);

    max_clique
}

pub fn run() {
    let input = include_str!("input/input23.txt");
    let mut g = Graph::default();
    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();
        g.add_edge(a, b);
    }
    let mut res = find_max_clique(&g)
        .into_iter()
        .map(|i| g.nodes[i].name.as_str())
        .collect::<Vec<_>>();
    res.sort();
    println!("{}", res.join(","));
    println!();
}