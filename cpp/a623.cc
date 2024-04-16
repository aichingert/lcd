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
    TreeNode* addOneRow(TreeNode* root, int val, int depth) {
        if (depth <= 1) {
            TreeNode *ans = new TreeNode(val, root, NULL);
            root = ans;
        } else {
            insert(root, 1, depth, val);
        }

        return root;
    }

    void insert(TreeNode *root, int cd, int dd, int v) {
        if (!root) return;
        if (cd < dd - 1) {
            cd++;
            insert(root->left, cd, dd, v);
            insert(root->right, cd, dd, v);
            return;
        }

        TreeNode *l = new TreeNode(v, root->left, NULL);
        TreeNode *r = new TreeNode(v, NULL, root->right);

        root->left = l;
        root->right = r;
    }
};
