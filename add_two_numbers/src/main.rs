use std::boxed::Box;
mod two_sum;
use crate::Solution;


#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None
        }
    }
}
pub struct Solution {}
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0); // Create a dummy node
        let mut curr = &mut dummy; // Reference to current node
        let mut carry = 0; // Initialize carry to 0

        let mut l1 = l1;
        let mut l2 = l2;

        while l1.is_some() || l2.is_some() {
            let mut sum = carry;
            if let Some(node) = l1 {
                sum += node.val;
                l1 = node.next;
            }
            if let Some(node) = l2 {
                sum += node.val;
                l2 = node.next;
            }

            carry = sum / 10;
            curr.next = Some(Box::new(ListNode::new(sum % 10)));
            curr = curr.next.as_mut().unwrap();
        }

        if carry > 0 {
            curr.next = Some(Box::new(ListNode::new(carry)));
        }

        return dummy.next;
    }
}

fn main() {
    let mut l1 = Some(Box::new(ListNode::new(2)));
    l1.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));
    l1.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));

    let mut l2 = Some(Box::new(ListNode::new(5)));
    l2.as_mut().unwrap().next = Some(Box::new(ListNode::new(6)));
    l2.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));

    let result = Solution::add_two_numbers(l1, l2);
    println!("{:?}", result);

    two_sum::run();
}