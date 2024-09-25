class Solution {
public:
    struct Trie {
        int count;
        Trie* children[26];
    };

    vector<int> sumPrefixScores(vector<string>& words) {
        Trie* top[26] = {0};
        Trie* current = nullptr;

        for (auto& word : words) {
            if (top[word[0] - 'a'] == nullptr) {
                top[word[0] - 'a'] = new Trie();
            }

            current = top[word[0] - 'a'];
            current->count += 1;

            for (int i = 1; i < word.size(); i++) {
                int idx = word[i] - 'a';

                if (current->children[idx] == nullptr) {
                    current->children[idx] = new Trie();
                }
                current = current->children[idx];

                current->count += 1;
            }
        }

        vector<int> ans = {};

        for (auto& word : words) {
            current = top[word[0] - 'a'];
            int count = current->count;

            for (int i = 1; i < word.size(); i++) {
                current = current->children[word[i] - 'a'];
                count += current->count;
            }

            ans.push_back(count);
        }
        
        return ans;
    }
};
