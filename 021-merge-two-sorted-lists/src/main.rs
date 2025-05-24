// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub struct Solution;

impl Solution {
    // Recursive solution to merge two sorted lists
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => {
                println!("Both lists are empty, returning None");
                None
            },
            (Some(l), None) => {
                println!("List 2 is empty, returning remaining list 1 with value: {}", l.val);
                Some(l)
            },
            (None, Some(r)) => {
                println!("List 1 is empty, returning remaining list 2 with value: {}", r.val);
                Some(r)
            },
            (Some(mut l), Some(mut r)) => {
                if l.val <= r.val {
                    println!("Comparing {} <= {}, choosing {} from list 1", l.val, r.val, l.val);
                    l.next = Self::merge_two_lists(l.next, Some(r));
                    Some(l)
                } else {
                    println!("Comparing {} > {}, choosing {} from list 2", l.val, r.val, r.val);
                    r.next = Self::merge_two_lists(Some(l), r.next);
                    Some(r)
                }
            }
        }
    }
}

// Helper function to create a linked list from a vector
fn from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
    vec.into_iter().rev().fold(None, |next, val| {
        Some(Box::new(ListNode { val, next }))
    })
}

// Helper function to convert a linked list to a vector
fn to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut current = &list;
    while let Some(node) = current {
        result.push(node.val);
        current = &node.next;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let list1 = from_vec(vec![1, 2, 4]);
        let list2 = from_vec(vec![1, 3, 4]);
        let merged = Solution::merge_two_lists(list1, list2);
        assert_eq!(to_vec(merged), vec![1, 1, 2, 3, 4, 4]);
    }

    #[test]
    fn test_empty_lists() {
        assert_eq!(Solution::merge_two_lists(None, None), None);
    }

    #[test]
    fn test_one_empty_list() {
        let list1 = from_vec(vec![1, 2, 3]);
        let merged = Solution::merge_two_lists(list1, None);
        assert_eq!(to_vec(merged), vec![1, 2, 3]);
    }

    #[test]
    fn test_another_empty_list() {
        let list2 = from_vec(vec![1, 2, 3]);
        let merged = Solution::merge_two_lists(None, list2);
        assert_eq!(to_vec(merged), vec![1, 2, 3]);
    }
}

// Demonstration in main
fn main() {
    let list1 = from_vec(vec![1, 2, 4]);
    let list2 = from_vec(vec![1, 3, 4]);
    let merged = Solution::merge_two_lists(list1, list2);
    println!("Merged list: {:?}", to_vec(merged));
}