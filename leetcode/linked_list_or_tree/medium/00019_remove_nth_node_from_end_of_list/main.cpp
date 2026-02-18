/**
 * 19. "Remove Nth Node From End of List"
 * 
 * Difficulty: Medium
 * Tags: Linked List, Two Pointers
 * Runtime: Beats 100%
 */
#include <algorithm>
class Solution {
public:
    ListNode* removeNthFromEnd ( ListNode* head, int n ) {
        ListNode** last = new learnListNode*[n + 1];
        for ( size_t i = 0; i <= n; i++ ) {
            last[i] = nullptr;
        }

        ListNode* cursor = head;
        while ( cursor != nullptr ) {
            std::rotate(last, last + 1, last + n + 1);
            last[n] = cursor;
            cursor = cursor->next;
        }

        if ( last[0] == nullptr ) {
            head = head->next;
        } else {
            last[0]->next = last[1]->next;
        }

        return head;
    }
};