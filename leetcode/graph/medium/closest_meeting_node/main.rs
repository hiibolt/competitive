/**
 * 2359. "Find Closest Node to Given Two Nodes"
 * 
 * Difficulty: Medium
 * Tags: Graph, DFS
 * Runtime: Beats 79%
 */

impl Solution {
    fn calculate_distance_vector (
        edges: &Vec<i32>,
        mut ind: usize
    ) -> Vec<i32> {
        let mut ret = vec!(-1; edges.len());

        let mut counter = 0;
        while ret[ind] == -1 {
            ret[ind] = counter;

            ind = if let Ok(ind_usize) = usize::try_from(edges[ind]) {
                ind_usize
            } else { break; };

            counter += 1;
        }

        ret
    }
    pub fn closest_meeting_node(
        edges: Vec<i32>,
        node1: i32,
        node2: i32
    ) -> i32 {
        let n1_distances = Self::calculate_distance_vector(&edges, node1 as usize);
        let n2_distances = Self::calculate_distance_vector(&edges, node2 as usize);

        let mut ret = None;
        for i in 0..edges.len() {
            if n1_distances[i] == -1 || n2_distances[i] == -1 {
                continue;
            }

            let dist = n1_distances[i].max(n2_distances[i]);
            if let Some((smallest, ind)) = ret {
                if dist >= smallest {
                    continue;
                }
            }
            ret = Some((dist, i as i32));
        }

        ret.map(|(_, ind)| ind as i32).unwrap_or(-1i32)
    }
}