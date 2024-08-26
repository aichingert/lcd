/*
// Definition for a Node.
class Node {
public:
    int val;
    vector<Node*> children;

    Node() {}

    Node(int _val) {
        val = _val;
    }

    Node(int _val, vector<Node*> _children) {
        val = _val;
        children = _children;
    }
};
*/

class Solution {
public:
    vector<int> postorder(Node* root) {
        if (root == nullptr) return {};

        vector<Node*> s = {root};
        vector<int> a = {};

        while (!s.empty()) {
            auto last = s[s.size() - 1];
            s.pop_back();
            bool isLast = true;

            auto children = last->children;
            last->children = {};
            s.push_back(last);
            for (int i = children.size() - 1; i >= 0; i--) {
                isLast = false;
                s.push_back(children[i]);
            }

            if (isLast) {
                s.pop_back();
                a.push_back(last->val);
            }
        }

        return a;
    }
};
