// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: *mut ListNode,
// }
//
// impl ListNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         ListNode { next: std::ptr::null_mut(), val }
//     }
// }

impl Solution {
    pub fn has_cycle(head: *mut ListNode) -> bool {
        if head.is_null() { return false; }
        unsafe {
            let mut slow = head;
            let mut fast = head;

            loop {
                slow = (*slow).next;
                fast = (*fast).next;
                if fast.is_null() { break; }
                fast = (*fast).next;
                if fast.is_null() { break; }

                if slow == fast {
                    return true;
                }
            }
            false
        }
    }
}
