/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
class Solution {
public:
    int sumOfLeftLeaves(TreeNode* root) {
        return sum(root, false);
    }

    int sum(TreeNode* cur, bool left) {
        int ret = 0;
    
        if (cur != NULL) {
            if (cur->left == NULL && cur->right == NULL && left) {
                ret = cur->val;
            }
    
            ret += sum(cur->left, true) + sum(cur->right, false);
        }
    
        return ret;
    }
};
