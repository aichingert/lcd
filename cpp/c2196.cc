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
#include <unordered_map>

class Solution {
public:
    TreeNode* createBinaryTree(vector<vector<int>>& descriptions) {
        std::unordered_map<int, TreeNode*> lookup = {};
        std::unordered_set<int> children = {};

        for (auto& desc : descriptions) {
            TreeNode *parent = NULL;
            TreeNode *child = NULL;

            if (lookup.contains(desc[0])) {
                parent = lookup.at(desc[0]);
            } else {
                parent = new TreeNode(desc[0]);
            }

            if (lookup.contains(desc[1])) {
                child = lookup.at(desc[1]);
            } else {
                child = new TreeNode(desc[1]);
            }
            children.insert(child->val);

            if (desc[2]) {
                parent->left = child;
            } else {
                parent->right = child;
            }

            lookup.insert({parent->val, parent});
            lookup.insert({child->val, child});
        }

        for (const std::pair<const int, TreeNode*>& n : lookup) {
            if (!children.contains(n.first)) {
                return n.second;
            }
        }

        return NULL;
    }
};
