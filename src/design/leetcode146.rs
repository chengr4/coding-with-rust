use std::{cell::RefCell, rc::Rc, collections::HashMap};


// RC: 允許一個節點有多個擁有者（HashMap 和 Linked List 鄰居都擁有它）
// RefCell: 允許在不可變引用下修改資料（Linked List 節點的前後指標需要修改, RC 不能直接修改內容）)
type Link = Rc<RefCell<DNode>>;

struct DNode {
    key: i32,
    value: i32,
    prev: Option<Link>,
    next: Option<Link>,
}

impl DNode {
    fn new(key: i32, value: i32) -> Self {
        DNode {
            key,
            value,
            prev: None,
            next: None,
        }
    }
}

struct LRUCache {
    capacity: usize,
    map: HashMap<i32, Link>,
    head_node: Link,
    tail_node: Link,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {

    fn new(capacity: i32) -> Self {
        let head_node = Rc::new(RefCell::new(DNode::new(0, 0)));
        let tail_node = Rc::new(RefCell::new(DNode::new(0, 0)));
        
        head_node.borrow_mut().next = Some(tail_node.clone());
        tail_node.borrow_mut().prev = Some(head_node.clone());

        LRUCache {
            capacity: capacity as usize,
            map: HashMap::with_capacity(capacity as usize),
            head_node,
            tail_node,
        }

    }
    
    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.map.get(&key) {
            let node = node.clone(); // 增加 Rc 計數，避免 borrow conflict
            let value = node.borrow().value;

            self.move_to_head(node);
            return value;

        }
        -1
    }
    
    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.map.get(&key).cloned() {
            node.borrow_mut().value = value;
            self.move_to_head(node);
        } else {
            let new_node = Rc::new(RefCell::new(DNode::new(key, value)));
            self.map.insert(key, new_node.clone());
            self.add_to_head(new_node);

            if self.map.len() > self.capacity {
                if let Some(tail) = self.remove_tail() {
                    self.map.remove(&tail.borrow().key);
                }
            }
        }
    }

    fn move_to_head(&self, node: Link) {
        self.remove_node(node.clone());
        self.add_to_head(node);
    }

    fn remove_node(&self, node: Link) {
        let prev = node.borrow().prev.clone();
        let next = node.borrow().next.clone();

        if let Some(prev_node) = prev.clone() {
            prev_node.borrow_mut().next = next.clone();
        }
        if let Some(next_node) = next {
            next_node.borrow_mut().prev = prev;
        }
    }

    fn add_to_head(&self, node: Link) {
        let old_first = self.head_node.borrow().next.clone();
        
        node.borrow_mut().prev = Some(self.head_node.clone());
        node.borrow_mut().next = old_first.clone();
        
        self.head_node.borrow_mut().next = Some(node.clone());
        
        if let Some(old_first_node) = old_first {
            old_first_node.borrow_mut().prev = Some(node);
        }
    }

    fn remove_tail(&self) -> Option<Link> {
        let tail_prev = self.tail_node.borrow().prev.clone();
        
        if let Some(last_node) = tail_prev {
            if Rc::ptr_eq(&last_node, &self.head_node) {
                return None;
            }
            self.remove_node(last_node.clone());
            return Some(last_node);
        }
        None
    }
}