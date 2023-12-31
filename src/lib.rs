use std::fmt::Display;
use std::ptr::null_mut;
struct Node<T: Copy + Display> {
    val: T,
    next: *mut Node<T>,
}

pub struct LinkedList<T: Copy + Display> {
    head: *mut Node<T>,
}
impl<T: Copy + Display> std::default::Default for LinkedList<T> {
    fn default() -> Self {
        Self { head: null_mut() }
    }
}
impl<T: Copy + Display> LinkedList<T> {
    pub fn new(val: T) -> Self {
        let new_node = Node {
            val,
            next: null_mut(),
        };
        Self {
            head: Box::into_raw(Box::new(new_node)),
        }
    }
    pub fn push(&mut self, val: T) {
        let new_node = Node {
            val,
            next: self.head,
        };
        self.head = Box::into_raw(Box::new(new_node));
    }
    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            if self.head.is_null() {
                return None;
            }
            let tmp = (*self.head).val;
            let tmp_head = (*self.head).next;
            let _ = Box::from_raw(self.head);
            self.head = tmp_head;
            Some(tmp)
        }
    }
    pub fn pop_back(&mut self) -> Option<T> {
        let mut current = self.head;
        let mut prev = null_mut();
        unsafe {
            while !(*current).next.is_null() {
                prev = current;
                current = (*current).next;
            }
            if !prev.is_null() {
                (*prev).next = null_mut();
            } else if !self.head.is_null() {
                self.head = null_mut();
            } else {
                return None;
            }
            let tmp = (*current).val;
            let _ = Box::from_raw(current);
            Some(tmp)
        }
    }
    pub fn display(&self) {
        let mut current = self.head;
        if current.is_null() {
            return;
        }
        unsafe {
            while !(*current).next.is_null() {
                print!("{}=>", (*current).val);
                current = (*current).next;
            }
            if !current.is_null() {
                print!("{}\n", (*current).val);
            }
        }
    }
}
