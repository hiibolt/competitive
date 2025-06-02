/**
 * 1791. "Find Center of Star Graph"
 * 
 * Difficulty: Easy
 * Tags: Graph
 * Runtime: Beats 100%
 */

use std::collections::HashSet;
impl Solution {
    pub fn find_center(
        edges: Vec<Vec<i32>>
    ) -> i32 {
        let mut connections = HashSet::new();

        for edge in edges {
            if connections.contains(&edge[0]) {
                return edge[0];
            }
            if connections.contains(&edge[1]) {
                return edge[1];
            }
            connections.insert(edge[0]);
            connections.insert(edge[1]);
        }
        
        -1i32
    }
}