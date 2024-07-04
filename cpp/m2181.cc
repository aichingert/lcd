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
    ListNode* mergeNodes(ListNode* head) {
        int sum = 0;
        head = head->next;
        ListNode *ans = head;

        while (head != NULL) {
            ListNode* cur = head;

            while (cur->val != 0) {
                sum += cur->val;
                cur = cur->next;
            }

            head->val = sum;
            head->next = cur->next;
            sum = 0;

            head = head->next;
        }

        return ans;
    }
};
