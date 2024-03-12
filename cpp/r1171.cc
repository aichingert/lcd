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
    ListNode* removeZeroSumSublists(ListNode* head) {
        ListNode* cur = head;

        int sum = 0;
        while (cur != NULL) {
            if (cur->next != NULL && cur->next->val == 0) {
                cur->next = cur->next->next;
                continue;
            }

            Solution::rm(cur);
            sum += cur->val;
            cur = cur->next;

            if (!sum) {
                head = cur;
            }
        }

        return head;
    }

    void rm(ListNode* head) {
        if (head == NULL || head->next == NULL) {
            return;
        }

        ListNode* cur = head->next;
        int sum = cur->val;
        while (cur->next != NULL) { 
            sum += cur->next->val;

            if (sum == 0) {
                head->next = cur->next->next;
            }
            cur = cur->next;
        }

        if (!sum) {
            head->next = NULL;
        }
    }
};
