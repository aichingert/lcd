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
    ListNode* mergeInBetween(ListNode* list1, int a, int b, ListNode* list2) {
        ListNode* ptr = list1;

        for (int i = 0; i < a - 1; ++i)
            ptr = ptr->next;

        ListNode* qtr = ptr;
        for (int i = 0; i < b - a + 1; ++i)
            qtr = qtr->next;

        ListNode* hr = list2;
        while (hr && hr->next) {
            hr = hr->next;
        }

        ptr->next = list2;
        hr->next = qtr->next;

        return list1;
    }
};
