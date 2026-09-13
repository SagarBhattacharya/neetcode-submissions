// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>, 
        mut list2: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        while list1.is_some() && list2.is_some() {
            let mut node1 = list1.take().unwrap();
            let mut node2 = list2.take().unwrap();

            if node1.val <= node2.val {
                list1 = node1.next.take();
                tail.next = Some(node1);
                list2 = Some(node2);
            } else {
                list2 = node2.next.take();
                tail.next = Some(node2);
                list1 = Some(node1);
            }

            tail = tail.next.as_mut().unwrap();
        }

        tail.next = if list1.is_some() 
        { list1 } else { list2 };

        dummy.next
    }
}
