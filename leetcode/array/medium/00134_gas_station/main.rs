/**
 * 134. "Gas Station"
 * 
 * Difficulty: Medium
 * Tags: Array, Greedy
 * Runtime: Beats 100%
 */
impl Solution {
    pub fn can_complete_circuit (
        gas: Vec<i32>,
        cost: Vec<i32>
    ) -> i32 {
        let mut net_costs: Vec<i32> = gas.into_iter()
            .zip(cost.into_iter())
            .map(|(gas, cost)| {
                gas - cost
            })
            .collect();

        let mut ind = 0;
        'outer: while ind < net_costs.len() {
            if net_costs[ind] < 0 {
                ind += 1;
                continue;
            }

            // Cyclically test each ind, starting at this one
            let mut gas = 0;
            for add_ind in (0..net_costs.len()).cycle().skip(ind).take(net_costs.len()) {
                gas += net_costs[add_ind];

                // If our gas is depleted, start at the depleting location
                //  or break if the depeleted location has been checked already
                if gas < 0 {
                    if add_ind + 1 <= ind {
                        break 'outer;
                    }
                    
                    ind = add_ind + 1;
                    continue 'outer;
                }
            }
            return ind as i32;
        }
        
        -1i32
    }
}