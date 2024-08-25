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
    vector<int> postorderTraversal(TreeNode* root) {
        if (root == nullptr) {
            return {};
        }

        vector<TreeNode*> s = {root};
        vector<int> a = {};

        while (!s.empty()) {
            auto cur = s[s.size() - 1];
            s.pop_back();

            if (cur->left == nullptr && cur->right == nullptr) {
                a.push_back(cur->val);
            } else {
                auto left = cur->left;
                auto right = cur->right;
                cur->left = nullptr;
                cur->right = nullptr;

                s.push_back(cur);
                
                if (right != nullptr) {
                    s.push_back(right);
                }
                if (left != nullptr) {
                    s.push_back(left);
                }
            }
        }

        return a;
    }
};
