/**
 * 1971. "Find if Path Exists in Graph"
 * 
 * Difficulty: Easy
 * Tags: DFS, BFS, Union Find, Graph
 * Runtime: Beats 29%
 */

use std::collections::{HashMap, HashSet};
fn check (
    adj: &HashMap<i32, HashSet<i32>>,
    checked: &mut HashSet<i32>,
    num: i32,
    target: i32
) -> bool {
    if num == target {
        return true;
    }
    if checked.contains(&num) {
        return false;
    }
    if !adj.contains_key(&num) || adj[&num].len() == 0 {
        checked.insert(num);
        return false;
    }

    checked.insert(num);
    for adj_nodes in adj[&num].iter() {
        if check(adj, checked, *adj_nodes, target) {
            return true;
        }
    }

    false
}
impl Solution {
    pub fn valid_path(
        n: i32,
        edges: Vec<Vec<i32>>,
        source: i32,
        destination: i32
    ) -> bool {
        let mut adj: HashMap<i32, HashSet<i32>> = HashMap::new();
        let mut checked = HashSet::new();
        for edge in edges {
            adj.entry(edge[0]).or_insert(HashSet::new())
                .insert(edge[1]);
            adj.entry(edge[1]).or_insert(HashSet::new())
                .insert(edge[0]);
        }

        if check(&adj, &mut checked, source, destination) {
            return true;
        }

        false
    }
}