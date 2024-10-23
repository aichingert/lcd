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
    TreeNode* replaceValueInTree(TreeNode* root) {
        queue<tuple<TreeNode*, int>> q;
        queue<tuple<TreeNode*, TreeNode*, int>> s;
        int prv = 0;
        int sum = 0;

        q.push({root, 0});
        s.push({root, nullptr, 0});
        TreeNode* node, *l, *r;
        int dep, d;

        for (; !q.empty(); q.pop()) {
            tie(node, dep) = q.front();

            if (prv != dep) {
                while (true) {
                    tie(l, r, d) = s.front();
                    if (d != prv) break;

                    int diff = (l == nullptr ? 0 : l->val) + (r == nullptr ? 0 : r->val);

                    if (l != nullptr) l->val = sum - diff;
                    if (r != nullptr) r->val = sum - diff;
                    s.pop();
                }

                sum = 0;
                prv = dep;
            }

            if (node->left != nullptr) q.push({node->left, dep + 1});
            if (node->right != nullptr) q.push({node->right, dep + 1});

            s.push({node->left, node->right, dep + 1});
            sum += node->val;
        }

        for (; !s.empty(); s.pop()) {
            tie(l, r, d) = s.front();

            int diff = (l == nullptr ? 0 : l->val) + (r == nullptr ? 0 : r->val);

            if (l != nullptr) l->val = sum - diff;
            if (r != nullptr) r->val = sum - diff;
        }

        return root;
    }
};
