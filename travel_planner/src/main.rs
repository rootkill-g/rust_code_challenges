use std::collections::{HashMap, BinaryHeap, HashSet};

type Node = usize;
type Cost = usize;

struct Graph {
    edges: HashMap<Node, Vec<(Node, Cost)>>,
    nodes: HashSet<Node>,
}

impl Graph {
    fn from_edge_list(edge_list: &Vec<(Node, Node, Cost)>) -> Self {
        let mut adjacency_list: HashMap<Node, Vec<(Node, Cost)>> = HashMap::new();
        let mut nodes = HashSet::new();

        for &(source, destination, cost) in edge_list.iter() {
            let destinations = adjacency_list.entry(source).or_insert_with(|| Vec::new());

            destinations.push((destination, cost));
            nodes.insert(source);
            nodes.insert(destination);
        }

        Graph {
            edges: adjacency_list,
            nodes,
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Step {
    cost: Cost,
    position: Node,
    history: Vec<Node>,
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost).then_with(|| self.position.cmp(
            &self.position
        ))
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Step {
    fn new(cost: Cost, position: Node, history: Vec<Node>) -> Self {
        Step {
            cost,
            position,
            history
        }
    }

    fn from_start(position: Node) -> Self {
        Step::new(0, position, vec![])
    }
}

fn shortest_path(g: &Graph, start: Node, goal: Node) -> Option<(Vec<Node>, Cost)> {
    let mut dist: HashMap<Node, Cost> = g.nodes.iter().map(|&x| if x == start { (x, 0) } else { (x, usize::MAX) }).collect();
    let mut priority_queue = BinaryHeap::new();

    priority_queue.push(Step::from_start(start));

    while let Some(Step {cost, position, mut history}) = priority_queue.pop() {
        if position == goal {
            history.push(goal);
            return Some((history, cost));
        }

        if let Some(destinations) = &g.edges.get(&position) {
            for &(next_destination, next_cost) in destinations.iter() {
                if next_cost < dist[&next_destination] {
                    let mut next = Step::new(next_destination, cost + next_cost, history.clone());
                    next.history.push(position);
                    priority_queue.push(next);

                    if let Some(old_cost) = dist.get_mut(&next_destination) {
                        *old_cost = next_cost;
                    }
                }
            }
        }
    }

    None
}

fn main() {
    let edge_list = include!("large_graph.in");
    let g = Graph::from_edge_list(&edge_list);

    if let Some((path, cost)) = shortest_path(&g, 1000, 9000) {
        println!("1000->9000, {:?} {}", path, cost);
    };
}
