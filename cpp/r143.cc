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
    void reorderList(ListNode* head) {
        if (!head || !head->next) {
            return;
        }

        ListNode* fast = head;
        ListNode* midd = head;

        while (fast && fast->next) {
            fast = fast->next->next;
            if (!fast || !fast->next) {
                ListNode* save = midd;
                midd = midd->next;
                save->next = NULL;
            } else {
                midd = midd->next;
            }
        }

        ListNode* prev = midd;
        ListNode* curr = midd->next;
        prev->next = NULL;

        while (curr) {
            ListNode* next = curr->next;
            curr->next = prev;
            prev = curr;
            curr = next;
        }

        fast = head->next;
        head->next = prev;

        while (fast) {
            ListNode* next = prev->next;
            prev->next = fast;
            fast = fast->next;
            prev->next->next = next;
            prev = next;
        }
    }
};
