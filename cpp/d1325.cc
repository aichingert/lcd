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
    TreeNode* removeLeafNodes(TreeNode* root, int target) {
        if (rem(root->left, target)) {
            root->left = NULL;
        }
        if (rem(root->right, target)) {
            root->right = NULL;
        }

        if (!root->left && !root->right && root->val == target) {
            return NULL;
        }

        return root;
    }

    bool rem(TreeNode* cur, int target) {
        if (cur == NULL) return true;

        if (rem(cur->left, target)) {
            cur->left = NULL;
        }
        if (rem(cur->right, target)) {
            cur->right = NULL;
        }

        return cur->left == NULL && cur->right == NULL && cur->val == target;
    }
};
