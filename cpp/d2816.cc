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
class Solution {
public:
    ListNode* doubleIt(ListNode* head, int p = 0) {
        if (head == nullptr) {
            return head;
        }

        ListNode* node = head;
        ListNode* nxt = doubleIt(node->next, p + 1);

        node->val *= 2;

        if (nxt) {
            node->val += nxt->val / 10;
            nxt->val %= 10;
        }

        if (node->val > 9 && p == 0) {
            ListNode* h = new ListNode(node->val / 10, node);
            node->val %= 10;
            return h;
        }

        return node;
    }
};
