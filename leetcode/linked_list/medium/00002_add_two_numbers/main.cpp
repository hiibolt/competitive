/**
 * 2. "Add Two Numbers"
 *
 * Difficulty: Medium
 * Tags: Linked List, Math, Recursion
 * Runtime: Beats 100%
 */
 
class Solution {
    public:
        ListNode * addTwoNumbers(
            ListNode* l1,
            ListNode* l2
        ) {
            ListNode* ret = nullptr;

            ListNode* cursor_1 = l1;
            ListNode* cursor_2 = l2;
            int carry = 0;

            while ( cursor_1 != nullptr || cursor_2 != nullptr ) {
                int cursor_1_val = cursor_1 == nullptr ? 0 : cursor_1->val;
                int cursor_2_val = cursor_2 == nullptr ? 0 : cursor_2->val;
                int new_val = cursor_1_val + cursor_2_val + carry;

                if ( new_val >= 10 ) {
                    carry = 1;
                    new_val -= 10;
                } else {
                    carry = 0;
                }

                if ( ret == nullptr ) {
                    ret = new ListNode(new_val);
                } else {
                    ListNode* push_cursor = ret;
                    while ( push_cursor->next != nullptr ) {
                        push_cursor = push_cursor->next;
                    }
                    push_cursor->next = new ListNode(new_val);
                }

                if ( cursor_1 != nullptr ) cursor_1 = cursor_1->next;
                if ( cursor_2 != nullptr ) cursor_2 = cursor_2->next;
            }

            if ( carry != 0 ) {
                if ( ret == nullptr ) {
                    ret = new ListNode(carry);
                } else {
                    ListNode* push_cursor = ret;
                    while ( push_cursor->next != nullptr ) {
                        push_cursor = push_cursor->next;
                    }
                    push_cursor->next = new ListNode(carry);
                }
            } 

            return ret;
        }
};