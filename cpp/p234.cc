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
    ListNode* front;
    bool check(ListNode* node){
        if (node!=NULL) {
            if (!check(node->next)) return 0;
            if (node->val!=front->val) return 0;
            front=front->next;
        }
        return 1;
    }

    bool isPalindrome(ListNode* head) {
        front = head;
        return check(head);
    }
};
