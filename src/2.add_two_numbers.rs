#![allow(unused)]

use crate::structs::list_node::ListNode;

#[test]
fn test_add_two_numbers() {}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut carry = 0;
    let mut result: Option<Box<ListNode>> = None;
    let mut current = &mut result;
    let mut n1 = l1.as_ref();
    let mut n2 = l2.as_ref();

    while n1.is_some() || n2.is_some() || carry > 0 {
        let mut sum = carry;

        if let Some(v) = n1 {
            sum += v.val;
            n1 = v.next.as_ref();
        }
        if let Some(v) = n2 {
            sum += v.val;
            n2 = v.next.as_ref();
        }

        carry = sum / 10;
        let digit = sum % 10;

        *current = Some(Box::new(ListNode::new(digit)));
        current = &mut current.as_mut().unwrap().next;
    }
    result
}
