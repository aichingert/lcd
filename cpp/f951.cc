// TODO: implement this in rust

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
    bool flipEquiv(TreeNode* root1, TreeNode* root2) {
        return sol(root1, root2);
    }

    bool sol(TreeNode* cur, TreeNode* oth) {
        if (cur == nullptr || oth == nullptr) {
            return cur == nullptr && oth == nullptr;
        }

        if (cur->val != oth->val) {
            return false;
        }

        if (unwrap(cur->left) == unwrap(oth->left) && unwrap(cur->right) == unwrap(oth->right)) {
            return sol(cur->left, oth->left) && sol(cur->right, oth->right);
        }

        if (unwrap(cur->left) == unwrap(oth->right) && unwrap(cur->right) == unwrap(oth->left)) {
            return sol(cur->left, oth->right) && sol(cur->right, oth->left);
        }

        return false;
    }

    int unwrap(TreeNode *n) {
        if (n == nullptr) {
            return -1;
        }

        return n->val;
    }
};
