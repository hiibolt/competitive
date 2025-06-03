/**
 * 114. "Flatten Binary Tree to Linked List"
 *
 * Difficulty: Medium
 * Tags: Linked List, Stack, Tree, DFS, Binary Tree
 * Runtime: Beats 100%
 */
class Solution {
    public:
        void flatten ( TreeNode* root ) {
            if ( root == nullptr ) return;
            if ( root->left == nullptr ) flatten(root->right);
    
            if ( root->right != nullptr ) {
                TreeNode* cursor = root->left;
    
                if ( cursor == nullptr ) return;
                while ( cursor->right != nullptr ) {
                    cursor = cursor->right;
                } 
                
                cursor->right = root->right;
            }
    
            root->right = root->left;
            root->left = nullptr;
    
            flatten(root->right);
        }
};