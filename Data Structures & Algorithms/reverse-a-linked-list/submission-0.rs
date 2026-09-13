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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_list = None;
        let mut current = head;

        while let Some(mut node) = current.take() {
            current = node.next.take();
            if let Some(new_node) = new_list.take() {
                node.next = Some(new_node);
            }
            new_list = Some(node);
        }

        new_list
    }
}
