use std::ops::Deref;
use std::option::Option::Some;

#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }
    let mut prev = None;
    let mut curr = head;
    while curr.is_some() {
        let mut node = curr.take().unwrap();
        curr = node.next;
        node.next = prev;
        prev = Some(node);
    }
    prev
}

fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }

    let mut count = 0;
    let mut root = &head;
    while root.is_some() {
        count += 1;
        root = &root.as_ref().unwrap().next
    }

    let mut root = &mut head;
    let mut idx = 0;
    while idx < count {
        idx += 1;
        root = &mut root.as_mut().unwrap().next;
    }

    return root.take();
}

fn find_node(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut root = head;
    while let Some(node) = root {
        if node.val == val {
            return Option::Some(node);
        }
        root = node.next;
    }
    return None;
}

fn delete_node(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode::new(0)));
    let mut next = dummy.as_mut();
    while let Some(mut inner) = head {
        head = inner.next.take();
        if inner.val != val {
            next.as_mut().unwrap().next = Some(inner);
            next = next.unwrap().next.as_mut();
        }
    }
    dummy.unwrap().next
}

fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    let mut list = Vec::new();

    let mut root = head;

    while let Some(node) = root {
        list.push(node.val);
        root = node.next;
    }

    let mut idx = 0;
    let length = list.len();
    while idx < length {
        if list[idx] != list[length - 1 - idx] {
            return false;
        }
    }
    return true;
}

fn add_two_numbers(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut prev = Some(Box::new(ListNode::new(0)));
    let mut ret_list = prev.as_mut();
    let mut up = false;

    while l1.is_some() || l2.is_some() {
        let mut val = 0;

        if let Some(l1_node) = l1 {
            val += l1_node.val;
            l1 = l1_node.next;
        }
        if let Some(l2_node) = l2 {
            val += l2_node.val;
            l2 = l2_node.next;
        }

        if up {
            up = false;
            val += 1;
        }
        if val > 9 {
            up = true;
        }

        let cur_node = ret_list.unwrap();
        cur_node.next = Some(Box::new(ListNode::new(val)));
        ret_list = cur_node.next.as_mut();
    }

    return prev.unwrap().next;
}

fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }

    let mut node = head.unwrap();
    let new_list = sort_list(node.next);
    node.next = new_list;

    let mut for_ret = ListNode { val: -2147483648, next: Some(node) };
    let mut cur = &mut for_ret;

    loop {
        if cur.next.is_none() {
            break;
        }
        let mut next_node = cur.next.unwrap();

        let next2_opt = next_node.next;
        if next2_opt.is_none() {
            break;
        }
        let mut next2_node = next2_opt.unwrap();

        if next_node.val < next2_node.val {
            break;
        }

        let tmp = next2_node.next;
        next_node.next = tmp;
        next2_node.next = Some(next_node);
        cur.next = Some(next2_node);

        cur = &mut next_node;
    }


    return for_ret.next;
}

fn main() {}
