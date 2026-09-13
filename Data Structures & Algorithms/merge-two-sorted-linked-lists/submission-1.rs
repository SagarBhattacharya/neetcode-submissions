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

        while let (Some(node1), Some(node2)) = (&list1, &list2) {
            if node1.val <= node2.val {
                tail.next = list1.take();
                tail = tail.next.as_mut().unwrap();
                list1 = tail.next.take();
            } else {
                tail.next = list2.take();
                tail = tail.next.as_mut().unwrap();
                list2 = tail.next.take();
            }
            
        }

        tail.next = list1.or(list2);
        dummy.next
    }
}
