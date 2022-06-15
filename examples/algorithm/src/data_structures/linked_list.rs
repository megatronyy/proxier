/*
    双向链表
    新知识点：
    1、std::ptr::NonNull
    2、std::marker::PhantomData
*/
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use std::ptr::NonNull;

struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
    perv: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
            perv: None,
        }
    }
}

pub struct LinkedList<T> {
    length: u32,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    market: PhantomData<Box<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            head: None,
            tail: None,
            market: PhantomData,
        }
    }

    pub fn insert_at_head(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = self.head;
        node.perv = None;

        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.head {
            None => self.tail = node_ptr,
            Some(head_ptr) => unsafe { (*head_ptr.as_ptr()).perv = node_ptr },
        }
        self.head = node_ptr;
        self.length += 1;
    }

    pub fn insert_as_tail(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.perv = self.tail;
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.tail {
            None => self.head = node_ptr,
            Some(tail_ptr) => unsafe { (*tail_ptr.as_ptr()).next = node_ptr },
        }
        self.tail = node_ptr;
        self.length += 1;
    }

    pub fn insert_as_ith(&mut self, index: u32, obj: T) {
        if self.length < index {
            panic!("Index out of bounds");
        }

        if index == 0 || self.head == None {
            self.insert_at_head(obj);
            return;
        }

        if self.length == index {
            self.insert_as_tail(obj);
            return;
        }

        if let Some(mut ith_node) = self.head {
            for _ in 0..index {
                unsafe {
                    match ((*ith_node).as_ptr()).next {
                        None => panic!("Index out of bounds"),
                        Some(next_ptr) => ith_node = next_ptr,
                    }
                }
            }

            let mut node = Box::new(Node::new(obj));
            unsafe {
                node.perv = (*ith_node.as_ptr()).perv;
                node.next = Some(ith_node);
                if let Some(p) = (*ith_node.as_ptr()).perv {
                    let node_ptr = Some(NonNull::new_unchecked(Box::into_raw(node)));
                    println!("{:?}", (*p.as_ptr()).next);
                    (*p.as_ptr()).next = node_ptr;
                    self.length += 1;
                }
            }
        }
    }

    fn delete_head(&mut self) -> Option<T> {
        self.head.map(|head_ptr| unsafe {
            let old_head = Box::from_raw(head_ptr.as_ptr());
            match old_head.next {
                Some(mut next_ptr) => next_ptr.as_mut().perv = None,
                None => self.tail = None,
            }
            self.head = old_head.next;
            self.length -= 1;
            old_head.val
        })
    }

    fn delete_tail(&mut self) -> Option<T> {
        self.tail.map(|tail_ptr| unsafe {
            let old_tail = Box::from_raw(tail_ptr.as_ptr());
            match old_tail.prev {
                Some(mut prev) => prev.as_mut().next = None,
                None => self.head = None,
            }
            self.tail = old_tail.prev;
            self.length -= 1;
            old_tail.val
        })
    }

    fn delete_ith(&mut self, index: u32) -> Option<T> {
        if self.length < index {
            panic!("Index out of bounds");
        }

        if index == 0 || self.head == None {
            return self.delete_head();
        }

        if self.length == index {
            return self.delete_tail();
        }

        if let Some(mut ith_node) = self.head {
            for _ in 0..index {
                unsafe {
                    match (*ith_node.as_ptr()).next {
                        None => panic!("Index out of bounds"),
                        Some(next_ptr) => ith_node = next_ptr,
                    }
                }
            }

            unsafe {
                let old_ith = Box::from_raw(ith_node.as_ptr());
                if let Some(mut prev) = old_ith.perv {
                    prev.as_mut().next = old_ith.next;
                }

                if let Some(mut next) = old_ith.next {
                    next.as_mut().perv = old_ith.perv;
                }
                self.length -= 1;
                Some(old_ith.val)
            }
        } else {
            None
        }
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.head, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }),
            },
        }
    }
}

impl Drop for LinkedList<T> {
    fn drop(&mut self) {
        // Pop items until there are none left
        while self.delete_head().is_some() {}
    }
}

impl<T> Display for LinkedList<T>
    where T: Display
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.head {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
    where T: Display
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.next {
            Some(node) => write!(f, "{} {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn insert_at_tail_works() {}
}