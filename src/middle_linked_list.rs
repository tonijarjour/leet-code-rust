#[derive(Debug, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

pub fn solve(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut mid, mut end) = (&head, &head);

        while let Some(ListNode { next: Some(node), .. }) = end.as_deref() {
            mid = &mid.as_ref().unwrap().next;
            end = &node.next;
        }

        mid.clone()
}
