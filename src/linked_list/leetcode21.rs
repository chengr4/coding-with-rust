use super::ListNode;

pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  let mut dummy = ListNode::new(0);
  let mut curr_head = &mut dummy;

  while list1.is_some() && list2.is_some() {
    if list1.as_ref().unwrap().val < list2.as_ref().unwrap().val {
      curr_head.next = list1.clone();
      list1 = list1.unwrap().next;
    } else {
      curr_head.next = list2.clone();
      list2 = list2.unwrap().next;
    }

    curr_head = curr_head.next.as_mut().unwrap();
  }

  if list1.is_some() {
    curr_head.next = list1;
  }
  if list2.is_some() {
    curr_head.next = list2;
  }

  dummy.next
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_merge_two_lists() {
    assert_eq!(
      merge_two_lists(
        Some(Box::new(ListNode {
          val: 1,
          next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
              val: 4,
              next: None
            }))
          }))
        })),
        Some(Box::new(ListNode {
          val: 1,
          next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode {
              val: 4,
              next: None
            }))
          }))
        }))
      ),
      Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
          val: 1,
          next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
              val: 3,
              next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                  val: 4,
                  next: None
                }))
              }))
            }))
          }))
        }))
      }))
    );
  }
}