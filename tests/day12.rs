use std::collections::{hash_map::Entry, HashMap, HashSet};

const INPUT: &str = include_str!("res/12.txt");

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Vertex<'a> {
    Start,
    Big(&'a str),
    Small(&'a str),
    End,
}

impl Vertex<'_> {
    fn new(token: &str) -> Vertex<'_> {
        match token {
            "start" => return Vertex::Start,
            "end" => return Vertex::End,
            _ => {}
        };
        if token.chars().all(|c| c.is_ascii_lowercase()) {
            return Vertex::Small(token);
        }
        Vertex::Big(token)
    }

    fn big(&self) -> bool {
        matches!(self, &Vertex::Big(_))
    }
}

trait Graph<'a> {
    fn candidates(&self, current: &Vertex<'a>, visited: &[&Vertex<'a>]) -> Vec<&Vertex<'a>>;
}

fn parse<'b>(lines: impl Iterator<Item = &'b str>) -> HashMap<Vertex<'b>, HashSet<Vertex<'b>>> {
    let edges: HashSet<_> = lines
        .map(|token| {
            let (a, b) = token.split_once('-').unwrap();
            let (a, b) = (Vertex::new(a), Vertex::new(b));
            [(a.clone(), b.clone()), (b, a)]
        })
        .flat_map(|a| a.into_iter())
        .collect();
    let mut vertices: HashMap<_, HashSet<_>> = HashMap::new();
    for (a, b) in edges {
        match vertices.entry(a) {
            Entry::Occupied(mut occupied) => {
                occupied.get_mut().insert(b);
            }
            Entry::Vacant(vacant) => {
                let mut set = HashSet::new();
                set.insert(b);
                vacant.insert(set);
            }
        }
    }
    vertices
}

#[derive(Debug)]
struct SimpleGraph<'a>(HashMap<Vertex<'a>, HashSet<Vertex<'a>>>);

impl<'a> SimpleGraph<'a> {
    fn new(lines: impl Iterator<Item = &'a str>) -> Self {
        let vertices = parse(lines);
        Self(vertices)
    }
}

impl<'a> Graph<'a> for SimpleGraph<'a> {
    fn candidates(&self, current: &Vertex<'a>, visited: &[&Vertex<'a>]) -> Vec<&Vertex<'a>> {
        self.0
            .get(current)
            .unwrap()
            .iter()
            .filter(|n| n.big() || !visited.contains(n))
            .collect()
    }
}

#[derive(Debug)]
struct MultiVisitGraph<'a>(HashMap<Vertex<'a>, HashSet<Vertex<'a>>>);

impl<'a> MultiVisitGraph<'a> {
    fn new(lines: impl Iterator<Item = &'a str>) -> Self {
        let vertices = parse(lines);
        Self(vertices)
    }
}

impl<'a> Graph<'a> for MultiVisitGraph<'a> {
    /// for a `MultiVisitGraph`, a single small cave can be visited twice. All
    /// other small caves can be visited at most once.
    fn candidates(&self, current: &Vertex<'a>, visited: &[&Vertex<'a>]) -> Vec<&Vertex<'a>> {
        // count the number of times each vertex was visited
        let mut freqs: HashMap<_, u32> = HashMap::new();
        for vertex in visited {
            match freqs.entry(vertex) {
                Entry::Occupied(mut occupied) => {
                    *occupied.get_mut() += 1;
                }
                Entry::Vacant(vacant) => {
                    vacant.insert(1);
                }
            }
        }
        // find the one vertex that was potentially visited twice
        let twice = freqs
            .iter()
            .any(|(v, t)| matches!(v, Vertex::Small(_)) && *t >= 2);
        // if we have already visited a node twice and `current` is being
        // visited for a second time, we are on an invalid path.
        if twice && visited.contains(&current) {
            if let Vertex::Small(_) = current {
                return Vec::new();
            }
        }
        self.0
            .get(current)
            .unwrap()
            .iter()
            .filter(|v| match freqs.get(v) {
                Some(&times_visited) => match v {
                    Vertex::Start => false,
                    Vertex::End => true,
                    Vertex::Small(_) => match twice {
                        true => times_visited < 1,
                        false => times_visited < 2,
                    },
                    Vertex::Big(_) => true,
                },
                None => true,
            })
            .collect()
    }
}

/// `'v` is the lifetime of the vertex container, `'id` is the lifetime of the
/// string which makes up its identifier. Realistically this is static but I
/// don't want to prescribe that in the struct definition.
#[derive(Debug)]
struct Node<'v, 'id> {
    vertex: &'v Vertex<'id>,
    visited: Vec<&'v Vertex<'id>>,
}

impl<'v, 'id> Node<'v, 'id> {
    fn compute(
        vertex: &'v Vertex<'id>,
        parent: Option<&Node<'v, 'id>>,
        graph: &'v impl Graph<'id>,
    ) -> (Node<'v, 'id>, u32) {
        // the "path" for this node is the path of the parent plus the parent
        // itself. (or an empty path if no parent)
        let visited = match parent {
            Some(parent) => {
                let mut visited = parent.visited.clone();
                visited.push(parent.vertex);
                visited
            }
            None => Vec::new(),
        };
        // construct the node, but without the children computed
        let node = Node { vertex, visited };
        // if this is a terminal node, short circuit
        if let Vertex::End = node.vertex {
            return (node, 1);
        }
        // the candidates are calculated from the parent and the graph based on
        // the rules laid out for day 12.
        let candidates = graph.candidates(vertex, &node.visited);
        // counting the number of terminal paths as we go to avoid traversing
        // the tree a second time.
        let mut endpoints = 0;
        // compute the children by acting recursively on the candidates
        for candidate in candidates {
            let (_, child_endpoints) = Node::compute(candidate, Some(&node), graph);
            endpoints += child_endpoints;
        }
        return (node, endpoints);
    }
}

#[test]
fn part1() {
    let graph = SimpleGraph::new(INPUT.lines());
    let (_, result) = Node::compute(&Vertex::Start, None, &graph);
    println!("Day 12, part 1: {}", result);
    assert_eq!(4754, result);
}

#[test]
fn part2() {
    let graph = MultiVisitGraph::new(INPUT.lines());
    let (_, result) = Node::compute(&Vertex::Start, None, &graph);
    println!("Day 12, part 2: {}", result);
    assert_eq!(143562, result);
}
