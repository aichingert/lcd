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
    vector<ListNode*> splitListToParts(ListNode* head, int k) {
        vector<int> offs(k);
        ListNode* cur = head;
        size_t len = 0;
        size_t idx = 0;

        while (cur != nullptr) {
            cur = cur->next;
            offs[idx]++;

            len++;
            idx = (idx + 1) % k;
        }

        vector<ListNode*> parts(k);

        idx = 0;
        cur = head;

        while (cur != nullptr && idx < k) {
            if (offs[idx] <= 1) {
                auto nxt = cur->next;
                cur->next = nullptr;

                if (parts[idx] == nullptr) {
                    parts[idx] = cur;
                }

                cur = nxt;
                idx += 1;
            } else {
                if (parts[idx] == nullptr) {
                    parts[idx] = cur;
                }

                offs[idx]--;
                cur = cur->next;
            }
        }

        return parts;
    }
};
