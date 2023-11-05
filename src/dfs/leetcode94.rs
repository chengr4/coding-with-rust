use crate::tree::TreeNode;
use std::{cell::RefCell, rc::Rc, vec};

type TreeNodePtr = Option<Rc<RefCell<TreeNode>>>;

pub fn inorder_traversal(root: TreeNodePtr) -> Vec<i32> {
    let mut res = vec![];
    dfs(root, &mut res);

    res
}

fn dfs(curr_node: TreeNodePtr, res: &mut Vec<i32>) {
    if let Some(curr) = curr_node {
        dfs(curr.borrow().left.clone(), res);
        res.push(curr.borrow().val);
        dfs(curr.borrow().right.clone(), res);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inorder_traversal() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: None,
            }))),
        })));
        assert_eq!(vec![1, 3, 2], inorder_traversal(root));
    }
}
