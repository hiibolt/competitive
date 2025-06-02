/**
 * 133. "Clone Graph"
 * 
 * > Difficulty: Medium
 * > Tags: Hash Table, DFS, BFS, Graph
 * > Runtime: Beats 40%
 */

#include <unordered_map>
class Solution {
public:
    Node* cloneGraph ( Node* node ) {
        std::unordered_map<Node *, Node *> viewed;

        return cloneGraphHelper ( node, &viewed );
    }
    Node* cloneGraphHelper (
        Node* node,
        std::unordered_map<Node *, Node *> * viewed
    ) {
        if ( node == nullptr ) return nullptr;
        if ( viewed->contains(node) ) return (*viewed)[node];

        std::vector<Node *> neighbors;
        Node * cloned_node = new Node(node->val);
        (*viewed)[node] = cloned_node;

        for ( auto neighbor_it : node->neighbors ) {
            if ( viewed->contains(neighbor_it) ) {
                neighbors.push_back((*viewed)[neighbor_it]);
            } else {
                Node * new_node = cloneGraphHelper( neighbor_it, viewed );
                neighbors.push_back(new_node);
            }
        }
        cloned_node->neighbors = neighbors;

        return cloned_node;
    }
};