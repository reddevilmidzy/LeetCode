// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res : Vec<i32> = Vec::new();

        Self::dfs(root, &mut res, 1);
        res
    }

    pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>, nodes: &mut Vec<i32>, depth: usize) {
        if let Some(node) = root {
            if nodes.len() < depth {
                nodes.push(node.borrow().val);
            }
            nodes[depth - 1] = nodes[depth - 1].max(node.borrow().val);

            Self::dfs(node.borrow().left.clone(), nodes, depth + 1);
            Self::dfs(node.borrow().right.clone(), nodes, depth + 1);
        }
    }
}