class Solution {
public:
    bool hasCycle(ListNode *head) {
        while (head) {
            if (head->val >= 100000) {
                return true;
            }

            head->val = 100000;
            head = head->next;
        }

        return false;
    }
};
