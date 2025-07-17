#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode{val:0,next:head});
        let mut len=0;
        {
            let mut p=dummy.next.as_ref();
            while let Some(node)=p { len+=1; p=node.next.as_ref(); }
        }
        let mut cur=&mut dummy;
        for _ in 0..len-n as usize { cur=cur.next.as_mut().unwrap(); }
        let next=cur.next.as_mut().and_then(|node| node.next.take());
        cur.next=next;
        dummy.next
    }
}

fn from_vec(v: Vec<i32>) -> Option<Box<ListNode>> { v.into_iter().rev().fold(None, |n,val| Some(Box::new(ListNode{val,next:n}))) }
fn to_vec(mut h: Option<Box<ListNode>>) -> Vec<i32> { let mut v=vec![]; while let Some(n)=h {v.push(n.val);h=n.next;} v }

#[cfg(test)]
mod tests{use super::*;#[test]fn test(){let head=from_vec(vec![1,2,3,4,5]);let res=Solution::remove_nth_from_end(head,2);assert_eq!(to_vec(res),vec![1,2,3,5]);}}

fn main(){let res=Solution::remove_nth_from_end(from_vec(vec![1,2,3,4,5]),2);println!("{:?}",to_vec(res));}
