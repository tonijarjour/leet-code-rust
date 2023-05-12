use std::mem;

#[derive(Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

pub fn solve(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut prev, mut curr) = (None, head);

        while let Some(mut n) = curr {
            curr = n.next;
            n.next = prev;
            prev = Some(n);
        }

        prev
}
