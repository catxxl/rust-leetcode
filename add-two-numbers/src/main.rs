// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
struct Solution {
}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if l1 == None || l2 == None {
            return None
        } else {
            let mut head = Some(Box::new(ListNode{val: 0, next: None}));
            let mut p = &mut head;
            let mut p1 = &l1;
            let mut p2 = &l2;
            let mut sum = 0;
            loop {
                if !p1.is_none() {
                    sum += p1.as_ref().unwrap().val;
                }
                if !p2.is_none() {
                    sum += p2.as_ref().unwrap().val;
                }
                if !p1.is_none() {
                    p1 = &p1.as_ref().unwrap().next;
                }
                if !p2.is_none() {
                    p2 = &p2.as_ref().unwrap().next;
                }
                match sum {
                    0 => {
                        if p1.is_none() && p2.is_none() {
                            break;
                        } else {
                            if p.is_none() {
                                *p = Some(Box::new(ListNode{val: 0, next: None}));
                            }
                            p = &mut p.as_mut().unwrap().next;
                        }
                    }
                    _ => {
                        if p.is_none() {
                            *p = Some(Box::new(ListNode{val: 0, next: None}));
                        }
                        p.as_mut().unwrap().val = sum % 10;
                        sum = sum / 10;
                        p = &mut p.as_mut().unwrap().next;
                    }
                }
            }
            return head;
        }
    }
}

fn main() {

    println!("Hello, world!");
}
