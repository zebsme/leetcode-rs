// @leet start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

// Solution 1
// Use DoubleEndedIterator
impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut current = head;
        let mut result = Vec::new();
        while let Some(node) = current {
            result.push(node.val);
            current = node.next;
        }
        let mut iter = result.into_iter();
        loop {
            match (iter.next(), iter.next_back()) {
                (Some(a), Some(b)) => {
                    if a == b {
                        continue;
                    } else {
                        return false;
                    }
                }
                (None, None) => return true,
                _ => return true,
            }
        }
    }
}

// Solution 2
// impl Solution {
//     // 876
//     fn middle_node(head: &Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//         let mut fast = head;
//         let mut slow = head;
//         while fast.is_some() && fast.as_ref()?.next.is_some() {
//             slow = &slow.as_ref()?.next;
//             fast = &fast.as_ref()?.next.as_ref()?.next;
//         }
//         // 把 slow 从 &Option<Box<ListNode>> 强转成 &mut Option<Box<ListNode>>
//         #[allow(mutable_transmutes)]
//         let slow: &mut Option<Box<ListNode>> = unsafe { std::mem::transmute(slow) };
//         slow.take() // 避免 clone()
//     }
//
//     // 206
//     fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//         let mut pre = None;
//         let mut cur = head;
//         while let Some(mut node) = cur {
//             let nxt = node.next;
//             node.next = pre;
//             pre = Some(node);
//             cur = nxt;
//         }
//         pre
//     }
//
//     pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
//         let mid = Self::middle_node(&head);
//         let mut head2 = Self::reverse_list(mid);
//         while head.is_some() {
//             if head.as_ref().unwrap().val != head2.as_ref().unwrap().val {
//                 return false;
//             }
//             head = head.unwrap().next;
//             head2 = head2.unwrap().next;
//         }
//         true
//     }
// }

// @leet end
