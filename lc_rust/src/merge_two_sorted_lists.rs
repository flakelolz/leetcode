#![allow(dead_code)]
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution;

impl Solution {
    // No recursion and therefore O(1) space complexity
    // No memory allocations --> Splicing as required by problem description
    // Optimal run-time complexity O(n).
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut temp = &mut list1;
        while list2.is_some() {
            if temp.is_none() || list2.as_ref()?.val < temp.as_ref()?.val {
                std::mem::swap(temp, &mut list2);
            }
            temp = &mut temp.as_mut()?.next;
        }
        list1
    }
}

// TODO: No tests yet. Linked lists in rust are kind of a nightmare, at least for now.

