use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let g = Graph {
        nodes: [(0, vec![1, 2]),
            (1, vec![0, 3]),
            (2, vec![0]),
            (3, vec![1])]
            .iter().cloned().collect()
    };

    let p = dfs(&g, &mut HashSet::new(), 0, 3, vec![]);

    println!("{:?}", p);
}

fn dfs(
    graph: &Graph,
    visited: &mut HashSet<usize>,
    node: usize,
    match_condition: usize,
    path: Vec<usize>,
) -> Vec<usize> {
    // mark node as visited
    visited.insert(node);

    // if this is the node we're searching for return the path in this stack with the node pushed
    if node == match_condition {
        [vec![node], path].concat()
    } else {
        // otherwise continue searching
        // get a list of neighbors of the node that have not been visited
        let neighbours: Vec<&usize> = graph
            .nodes
            .get(&node)
            .unwrap()
            .iter()
            .filter(|neighbour| !visited.contains(neighbour))
            .collect();

        // if there any neighbors to visit
        if neighbours.len() > 0 {
            // map combine all the neighbors together into a flattened array
            neighbours
                .into_iter()
                .map(|neighbour| {
                    // map to the recursive calls to dfs
                    dfs(
                        graph,
                        visited,
                        *neighbour,
                        match_condition,
                        [vec![node], path.clone()].concat(),
                    )
                })
                // flatten all the paths
                .flatten()
                .collect()
        } else { // if there are no valid neighbors this branch is a dead end, return nothing
            vec![]
        }
    }
}

struct Graph {
    nodes: HashMap<usize, Vec<usize>>,
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_single_branch() {
        let graph = Graph {
            nodes: [(0, vec![1]), (1, vec![0, 2]), (2, vec![1])].iter().cloned().collect()
        };

        let path = dfs(&graph, &mut HashSet::new(), 0, 2, vec![]);

        assert_eq!(path, vec![2, 1, 0])
    }

    #[test]
    fn test_single_split() {
        let graph = Graph {
            nodes: [(0, vec![1, 2]),
                (1, vec![0, 3]),
                (2, vec![0]),
                (3, vec![1])]
                .iter().cloned().collect()
        };

        let path = dfs(&graph, &mut HashSet::new(), 0, 3, vec![]);

        assert_eq!(path, vec![3, 1, 0])
    }
}
