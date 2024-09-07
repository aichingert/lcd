/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
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
    bool isSubPath(ListNode* head, TreeNode* root) {
        return found(head, root, false);
    }

    bool found(ListNode* l, TreeNode* n, bool t) {
        if (l == nullptr) {
            return true;
        }
        if (n == nullptr) {
            return false;
        }

        if (t) {
            if (l->val != n->val) {
                return false;
            }

            return found(l->next, n->left, true) || found(l->next, n->right, true);
        }

        if (l->val == n->val) {
            if (found(l->next, n->left, true) || found(l->next, n->right, true)) {
                return true;
            }
        }

        return found(l, n->left, false) || found(l, n->right, false);
    }
};
