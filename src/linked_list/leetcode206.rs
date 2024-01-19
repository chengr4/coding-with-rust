use super::ListNode;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut curr = head;

    while let Some(mut curr_node) = curr {
        curr = curr_node.next;
        curr_node.next = prev;
        prev = Some(curr_node);
    }

    prev
}


// test
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reverse_list() {
        let mut head = Some(Box::new(ListNode::new(1)));
        let mut node2 = Some(Box::new(ListNode::new(2)));
        let mut node3 = Some(Box::new(ListNode::new(3)));
        let mut node4 = Some(Box::new(ListNode::new(4)));
        let node5 = Some(Box::new(ListNode::new(5)));
        node4.as_mut().unwrap().next = node5;
        node3.as_mut().unwrap().next = node4;
        node2.as_mut().unwrap().next = node3;
        head.as_mut().unwrap().next = node2;
        let res = reverse_list(head);
        assert_eq!(res.unwrap().val, 5);
    }
}