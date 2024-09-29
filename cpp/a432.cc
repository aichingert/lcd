class AllOne {
public:
    struct Node {
        int frq;

        Node *prv;
        Node *nxt;
        unordered_set<string> keys;
    };

    Node *head;
    Node *tail;
    unordered_map<string, Node*> map;

    AllOne() {
        head = new Node();
        head->frq = 0;

        tail = new Node();
        tail->frq = 0;

        head->nxt = tail;
        tail->prv = head;
    }
    
    void inc(string key) {
        if (map.find(key) != map.end()) {
            Node *cur = map[key];
            cur->keys.erase(key);

            Node *nxt = cur->nxt;

            if (nxt == tail || nxt->frq != cur->frq + 1) {
                Node *next = new Node();
                next->frq = cur->frq + 1;
                next->prv = cur;
                next->nxt = nxt;
                cur->nxt = next;
                nxt->prv = next;

                next->keys.insert(key);
                nxt = next;
            } else {
                nxt->keys.insert(key);
            }

            map[key] = nxt;
            if (cur->keys.empty()) {
                removeNode(cur);
            }
        } else {
            if (head->nxt == tail || head->nxt->frq != 1) {
                Node *nxt = new Node();
                nxt->frq = 1;
                nxt->prv = head;
                nxt->nxt = head->nxt;
                head->nxt->prv = nxt;
                head->nxt = nxt;

                nxt->keys.insert(key);
                map[key] = nxt;
            } else {
                head->nxt->keys.insert(key);
                map[key] = head->nxt;
            }
        }
    }
    
    void dec(string key) {
        if (map.find(key) != map.end()) {
            Node *cur = map[key];
            cur->keys.erase(key);
            map.erase(key);

            Node *prv = cur->prv;
            
            if (cur->frq == 1) {
                if (cur->keys.empty()) {
                    removeNode(cur);
                }
                return;
            }

            if (prv->frq != cur->frq - 1) {
                Node *newPrv = new Node();
                newPrv->frq = cur->frq - 1;
                newPrv->keys.insert(key);

                cur->prv = newPrv;
                prv->nxt = newPrv;
                newPrv->prv = prv;
                newPrv->nxt = cur;

                prv = newPrv;
            } else {
                prv->keys.insert(key);
            }

            if (cur->keys.empty()) {
                removeNode(cur);
            }

            map[key] = prv;
        }
    }
    
    string getMaxKey() {
        if (tail->prv == head) {
            return "";
        }

        return *(tail->prv->keys.begin());
    }
    
    string getMinKey() {
        if (head->nxt == tail) {
            return "";
        }

        return *(head->nxt->keys.begin());
    }

private:
    void removeNode(Node *n) {
        Node *prv = n->prv;
        Node *nxt = n->nxt;

        prv->nxt = nxt;
        nxt->prv = prv;

        delete n;
    }
};

/**
 * Your AllOne object will be instantiated and called as such:
 * AllOne* obj = new AllOne();
 * obj->inc(key);
 * obj->dec(key);
 * string param_3 = obj->getMaxKey();
 * string param_4 = obj->getMinKey();
 */
