class Solution:
    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        head = current_node = None
        carry = 0
        
        while l1 or l2:
            val1 = l1.val if l1 else 0
            val2 = l2.val if l2 else 0
            total_sum = val1 + val2 + carry
            carry, current_digit = divmod(total_sum, 10)
            if not current_node:
                head = current_node = ListNode(current_digit)
            else:
                current_node.next = ListNode(current_digit)
                current_node = current_node.next
            l1 = l1.next if l1 else None
            l2 = l2.next if l2 else None
        if carry:
            current_node.next = ListNode(carry)
        return head