use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            prev: None,
            next: None,
        }
    }
}

pub struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) -> () {
        let mut node = Box::new(Node::new(obj));
        unsafe {
            // Since we are adding node at the end, next will always be None
            node.next = None;
            node.prev = self.end;
            // Get a pointer to node
            let node_ptr = Some(NonNull::new_unchecked(Box::into_raw(node)));
            match self.end {
                // This is the case of empty list
                None => self.start = node_ptr,
                Some(end_ptr) => (*end_ptr.as_ptr()).next = node_ptr,
            }

            self.end = node_ptr;
        }
        self.length = self.length + 1;
    }

    pub fn remove(&mut self, mut index: i32) -> Option<T> {
        let mut need_item = match self.start {
            Some(item) => item,
            None => return None,
        };

        unsafe {
            for _ in 0..index {
                match (*need_item.as_ptr()).next {
                    Some(item) => need_item = item,
                    None => return None,
                }
            }

            let prev = (*need_item.as_ptr()).prev;
            let next = (*need_item.as_ptr()).next;
            match (prev, next) {
                (Some(prev), Some(next)) => {
                    (*prev.as_ptr()).next = Some(next);
                    (*next.as_ptr()).prev = Some(prev);
                }
                (Some(prev), None) => {
                    (*prev.as_ptr()).next = None;
                    self.end = Some(prev);
                }
                (None, Some(next)) => {
                    (*next.as_ptr()).prev = None;
                    self.start = Some(next);
                }
                (None, None) => {
                    self.start = None;
                    self.end = None;
                }
            }

            self.length -= 1;

            Some(Box::from_raw(need_item.as_ptr()).val)
        }
    }

    pub fn get<'a>(&'a mut self, index: i32) -> Option<&'a T> {
        return self.get_ith_node(self.start, index);
    }

    fn get_ith_node<'a>(&'a mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&'a T> {
        unsafe {
            match node {
                None => None,
                Some(next_ptr) => match index {
                    0 => Some(&(*next_ptr.as_ptr()).val),
                    _ => self.get_ith_node((*next_ptr.as_ptr()).next, index - 1),
                },
            }
        }
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        unsafe {
            match self.start {
                Some(node) => write!(f, "{}", node.as_ref()),
                None => write!(f, ""),
            }
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        unsafe {
            match self.next {
                Some(node) => write!(f, "{}, {}", self.val, node.as_ref()),
                None => write!(f, "{}", self.val),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn get_by_index_in_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        println!("Linked List is {}", list);
        let retrived_item = list.get(1);
        assert!(retrived_item.is_some());
        assert_eq!(2 as i32, *retrived_item.unwrap());
    }

    #[test]
    fn get_by_index_in_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        println!("Linked List is {}", list_str);
        let retrived_item = list_str.get(1);
        assert!(retrived_item.is_some());
        assert_eq!("B", *retrived_item.unwrap());
    }
}
