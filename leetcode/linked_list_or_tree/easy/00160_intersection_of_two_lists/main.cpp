/**
 * 160. "Intersection of Two Lists"
 *
 * Difficulty: Easy
 * Tags: Hash Table, Linked List, Two Pointers
 * Runtime: Beats 11%
 */
#include <set>

class Solution {
public:
    ListNode* getIntersectionNode(ListNode* headA, ListNode* headB) {
        std::set<ListNode*> first_seen_a;
        std::set<ListNode*> first_seen_b;

        ListNode* a_cursor = headA;
        ListNode* b_cursor = headB;

        while ( true ) {
            if ( a_cursor == nullptr && b_cursor == nullptr ) {
                break;
            }

            if ( a_cursor != nullptr ) {
                if ( first_seen_b.contains(a_cursor) ) {
                    return a_cursor;
                }

                first_seen_a.insert(a_cursor);

                a_cursor = (*a_cursor).next;
            }

            if ( b_cursor != nullptr ) {
                if ( first_seen_a.contains(b_cursor) ) {
                    return b_cursor;
                }

                first_seen_b.insert(b_cursor);

                b_cursor = (*b_cursor).next;
            }
        }

        return nullptr;
    }
};