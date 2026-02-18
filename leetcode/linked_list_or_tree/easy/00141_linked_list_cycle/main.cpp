/**
 * 141. "Linked List Cycle"
 *
 * Difficulty: Easy
 * Tags: Linked List, Hash Table, Two Pointers
 * Runtime: Beats 83%
 */
class Solution {
public:
    bool hasCycle(ListNode *head) { 
        if ( head == nullptr || head->next == nullptr ) return false;

        ListNode* cursor1 = head;
        ListNode* cursor2 = head->next;
        while ( cursor1 != nullptr && cursor2 != nullptr ) {
            if ( cursor1 == cursor2 ) return true;

            cursor1 = cursor1->next;
            cursor2 = cursor2->next;
            if ( cursor2 == nullptr ) return false;
            cursor2 = cursor2->next;
        }

        return false;
    }
};