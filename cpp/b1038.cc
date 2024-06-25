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
    TreeNode* bstToGst(TreeNode* root) {
        sumDfs(root, 0);
        return root;
    }

    int sumDfs(TreeNode *n, int sum) {
        if (n == NULL) {
            return sum;
        }

        n->val += sumDfs(n->right, sum);
        return sumDfs(n->left, n->val);
    }
};
