/**
 * 445. "Add Two Numbers II"
 *
 * Difficulty: Medium
 * Tags: Linked List, Math, Stack
 * Runtime: Beats 47%
 */
class Solution {
    public:
        ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
            ListNode* l1f = reversed(l1);
            ListNode* l2f = reversed(l2);

            ListNode* ret = nullptr;

            int carry = 0;
            while ( l1f != nullptr || l2f != nullptr ) {
                if ( l1f != nullptr && l2f != nullptr ) {
                    int new_total = l1f->val + l2f->val + carry;
                    if ( new_total > 9 ) {
                        new_total -= 10;
                        carry = 1;
                    } else {
                        carry = 0;
                    }
                    ListNode* new_node = new ListNode(new_total);
                    new_node->next = ret;
                    ret = new_node;

                    l1f = l1f->next;
                    l2f = l2f->next;
                    continue;
                }

                int new_total = (l1f == nullptr ? l2f->val : l1f->val) + carry;
                if ( new_total > 9 ) {
                    new_total -= 10;
                    carry = 1;
                } else {
                    carry = 0;
                }
                ListNode* new_node = new ListNode(new_total);
                new_node->next = ret;
                ret = new_node;
                l1f = l1f == nullptr ? nullptr : l1f->next;
                l2f = l2f == nullptr ? nullptr : l2f->next;
            }
            if ( carry == 1 ) {
                ListNode* new_node = new ListNode(carry);
                new_node->next = ret;
                ret = new_node;
            }

            return ret;
        }
    private:
        ListNode* reversed ( ListNode* l ) {
            if ( l == nullptr ) return l;
            ListNode* ret = new ListNode(l->val);

            ListNode* cursor = l->next;
            while ( cursor != nullptr ) {
                ListNode* new_node = new ListNode(cursor->val);
                new_node->next = ret;
                ret = new_node;

                cursor = cursor->next;
            }

            return ret;
        }
};