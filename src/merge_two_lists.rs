#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn solve(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = None;
    let mut current = &mut dummy;

    while list1.is_some() && list2.is_some() {
        if list1.as_ref().unwrap().val < list2.as_ref().unwrap().val {
            std::mem::swap(current, &mut list1);
            std::mem::swap(&mut current.as_mut().unwrap().next, &mut list1)
        } else {
            std::mem::swap(current, &mut list2);
            std::mem::swap(&mut current.as_mut().unwrap().next, &mut list2)
        }
        current = &mut current.as_mut().unwrap().next;
    }

    if list1.is_some() {
        std::mem::swap(current, &mut list1);
    }
    if list2.is_some() {
        std::mem::swap(current, &mut list2);
    }

    dummy
}

pub fn run() {
    let list1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));
    let list2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));

    let res = solve(list1, list2);
    let mut current = res.as_ref();
    while let Some(c) = current {
        print!("{} ", c.val);
        current = current.as_ref().unwrap().next.as_ref();
    }
    println!();
}
