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
    ListNode* insertGreatestCommonDivisors(ListNode* head) {
        ListNode* ans = nullptr;
        ListNode* lst = nullptr;
        int prv = -1;

        while (head != nullptr) {
            if (prv == -1) {
                ans = head;
                lst = head;
                prv = lst->val;
            } else {
                auto nxt = lst->next;
                lst->next = new ListNode(gcd(prv, head->val), nxt);
                prv = head->val;
                lst = nxt;
            }

            head = head->next;
        }

        return ans;
    }

    int gcd(int a, int b) {
        if (a == b) return a;
        if (a <  b) return gcd(b - a, a);
        return gcd(a - b, b);
    }
};
