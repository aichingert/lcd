class MyCircularDeque {
private:
    struct Node {
        int value;
        Node *nxt;
        Node *prv;
    };

    Node *head;
    Node *tail;
    int size;
    int cap;

public:
    MyCircularDeque(int k) {
        head = nullptr;
        tail = nullptr;

        size = 0;
        cap = k;
    }
    
    bool insertFront(int value) {
        if (size >= cap) {
            return false;
        }

        Node *n = new Node();
        n->value = value;
        size++;

        if (head == nullptr) {
            n->nxt = nullptr;
            n->prv = nullptr;
            head = n;
            tail = n;
            return true;
        }

        head->prv = n;
        n->nxt = head;
        n->prv = nullptr;
        head = n;
        return true;
    }
    
    bool insertLast(int value) {
        if (size >= cap) {
            return false;
        }

        Node *n = new Node();
        n->value = value;
        size++;

        if (tail == nullptr) {
            n->prv = nullptr;
            n->nxt = nullptr;
            head = n;
            tail = n;
            return true;
        }

        tail->nxt = n;
        n->prv = tail;
        n->nxt = nullptr;
        tail = n;
        return true;
    }
    
    bool deleteFront() {
        if (head == nullptr) {
            return false;
        }

        head = head->nxt;
        if (head) {
            head->prv = nullptr;
        } else {
            tail = nullptr;
        }

        size--;
        return true;
    }
    
    bool deleteLast() {
        if (tail == nullptr) {
            return false;
        }

        tail = tail->prv;
        if (tail) {
            tail->nxt = nullptr;
        } else {
            head = nullptr;
        }

        size--;
        return true;
    }
    
    int getFront() {
        if (head == nullptr) {
            return -1;
        }

        return head->value;
    }
    
    int getRear() {
        if (tail == nullptr) {
            return -1;
        }

        return tail->value;
    }
    
    bool isEmpty() {
        return size == 0;
    }
    
    bool isFull() {
        return size == cap;
    }
};

/**
 * Your MyCircularDeque object will be instantiated and called as such:
 * MyCircularDeque* obj = new MyCircularDeque(k);
 * bool param_1 = obj->insertFront(value);
 * bool param_2 = obj->insertLast(value);
 * bool param_3 = obj->deleteFront();
 * bool param_4 = obj->deleteLast();
 * int param_5 = obj->getFront();
 * int param_6 = obj->getRear();
 * bool param_7 = obj->isEmpty();
 * bool param_8 = obj->isFull();
 */
