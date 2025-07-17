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

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        let mut carry = 0;
        let (mut p, mut q) = (l1.as_ref(), l2.as_ref());

        while p.is_some() || q.is_some() || carry != 0 {
            let sum = p.map_or(0, |n| n.val) + q.map_or(0, |n| n.val) + carry;
            carry = sum / 10;
            tail.next = Some(Box::new(ListNode::new(sum % 10)));
            tail = tail.next.as_mut().unwrap();
            p = p.and_then(|n| n.next.as_ref());
            q = q.and_then(|n| n.next.as_ref());
        }
        dummy.next
    }
}

fn from_vec(v: Vec<i32>) -> Option<Box<ListNode>> {
    let mut cur = None;
    for val in v.into_iter().rev() {
        cur = Some(Box::new(ListNode { val, next: cur }));
    }
    cur
}

fn to_vec(l: Option<Box<ListNode>>) -> Vec<i32> {
    let mut res = vec![];
    let mut cur = &l;
    while let Some(node) = cur {
        res.push(node.val);
        cur = &node.next;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        let l1 = from_vec(vec![2,4,3]);
        let l2 = from_vec(vec![5,6,4]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(to_vec(result), vec![7,0,8]);
    }
}

fn main() {
    let l1 = from_vec(vec![2,4,3]);
    let l2 = from_vec(vec![5,6,4]);
    let result = Solution::add_two_numbers(l1, l2);
    println!("{:?}", to_vec(result));
}
