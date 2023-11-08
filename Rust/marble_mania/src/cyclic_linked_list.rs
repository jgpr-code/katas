use std::cell::RefCell;
use std::rc::Rc;

pub trait CyclicLinkedList<T: Default> {
    fn insert(&mut self, elem: T);
    fn move_cursor(&mut self, amount: i32);
    fn remove(&mut self) -> T;
}

pub struct VectorList<T: Default> {
    elements: Vec<T>,
    cursor: i32,
}

impl<T: Default> VectorList<T> {
    pub fn new() -> Self {
        VectorList {
            elements: Vec::new(),
            cursor: -1,
        }
    }
}

impl<T: Default> CyclicLinkedList<T> for VectorList<T> {
    fn insert(&mut self, elem: T) {
        let index = (self.cursor + 1) as usize;
        self.elements.insert(index, elem);
        self.cursor += 1;
    }
    fn move_cursor(&mut self, amount: i32) {
        let len = self.elements.len() as i32;
        if len <= 1 {
            return;
        }
        self.cursor += amount;
        while self.cursor >= len {
            self.cursor -= len;
        }
        while self.cursor < 0 {
            self.cursor += len;
        }
    }
    fn remove(&mut self) -> T {
        let len = self.elements.len();
        if len == 0 {
            return T::default();
        }
        let removed = self.elements.remove(self.cursor as usize);
        if self.elements.len() == 0 {
            self.cursor = -1;
        }
        removed
    }
}

struct Node<T: Default> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}

pub struct LinkedList<T: Default> {
    length: usize,
    head: Option<Rc<RefCell<Node<T>>>>,
    cursor: Option<Rc<RefCell<Node<T>>>>,
    last: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Default> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            length: 0,
            head: None,
            cursor: None,
            last: None,
        }
    }
}

impl<T: Default> CyclicLinkedList<T> for LinkedList<T> {
    fn insert(&mut self, elem: T) {
        if let Some(head) = self.head {
        } else {
        }
    }

    fn move_cursor(&mut self, amount: i32) {
        todo!()
    }

    fn remove(&mut self) -> T {
        todo!()
    }
}

// struct LinkedList {
//     head: Rc<RefCell<Node>>,
//     cursor: Rc<RefCell<Node>>,
// }

// impl LinkedList {
//     fn new() -> Self {
//         let head = Rc::new(RefCell::new(Node {
//             value: 0,
//             next: None,
//             prev: None,
//         }));
//         LinkedList {
//             head: Rc::clone(&head),
//             cursor: Rc::clone(&head),
//         }
//     }
//     fn insert(&mut self, value: usize) {
//         let node = Rc::new(RefCell::new(Node {
//             value,
//             next: None,
//             prev: Some(Rc::clone(&self.cursor)),
//         }));
//         let curs = self.cursor.borrow_mut();
//         // curs.
//     }
// }
// what should the adt be able to do?
// move cursor while cycling around
// insert element after cursor
// remove element at cursor

// ADT
// member:
// - last_inserted_pos
// - current_player
// next_pos_to_insert() -> pos
// remove_special() -> pos
