/**
 * 559. "Maximum Depth of N-ary Tree"
 *
 * Difficulty: Easy
 * Tags: Tree, DFS, BFS
 * Runtime: Beats 99.15%
 */
#include <algorithm>
class Solution {
public:
    int maxDepth(
        Node* root
    ) {
        if ( root == nullptr ) {
            return 0;
        }
        int max = 0;
        for ( auto child : root->children ) {
            max = std::max(max, maxDepth(child)); 
        }
        
        return 1 + max;
    }
};
