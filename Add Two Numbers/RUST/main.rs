impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1, l2);
        let mut head = None;
        let mut current_node = &mut head;
        let mut carry = 0;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let mut val1 = 0;
            let mut val2 = 0;

            // Extract values from the current nodes if they exist
            if let Some(node1) = l1 {
                val1 = node1.val;
                l1 = node1.next;
            }
            if let Some(node2) = l2 {
                val2 = node2.val;
                l2 = node2.next;
            }

            // Calculate sum and carry
            let total_sum = val1 + val2 + carry;
            carry = total_sum / 10;
            let current_digit = total_sum % 10;

            // Create new node with the current digit and link it
            *current_node = Some(Box::new(ListNode::new(current_digit)));
            current_node = &mut current_node.as_mut().unwrap().next;
        }
        head
    }
}