#include <unordered_set>

struct ListNode {
    int val;
    ListNode* next;
    ListNode(int x)
        : val(x)
        , next(nullptr) { }
};

class Solution {
public:
    bool hasCycle(ListNode* head) {
        std::unordered_set<ListNode*> seen;

        while (head) {
            auto [_, inserted] = seen.insert(head);
            if (!inserted)
                return true;
            head = head->next;
        }

        return false;
    }

    // https://leetcode.com/problems/linked-list-cycle/solutions/6086375/video-using-two-pointers/
    bool hasCycleV2(ListNode* head) {
        auto* fast = head;
        auto* slow = head;

        while (fast and fast->next) {
            slow = slow->next;
            fast = fast->next->next;

            if (slow == fast)
                return true;
        }

        return false;
    }
};

int main() {
}
