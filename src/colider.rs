
use std::rc::Rc;
use std::cell::RefCell;

type Link = Option<Rc<RefCell<Node>>>;

type Dummy = Rc<RefCell<Node>>;
//type ObjPointer = Box<GameObject>;
type ObjPointer = Box<i32>;

struct Morton {
	
}

impl Morton {

}

#[derive(Clone)]
struct Node {
	obj:ObjPointer,
	prev:Link,
	next:Link,
	cellLength:Rc<RefCell<u64>>
}
impl Node {
	fn new(obj: ObjPointer,length :Rc<RefCell<u64>>) -> Rc<RefCell<Node>> {
		Rc::new(RefCell::new(Node{
			obj:obj,
			prev:None,
			next:None,
			cellLength:length,
		}))
	}
	pub fn remove(&mut self) {
		match &mut self.prev {
			Some(prev) => {
				prev.borrow_mut().next = if let Some(next) = &self.next {
					Some(Rc::clone(&next))
				} else {
					None
				}
			},
			None => {},
		}
		match &mut self.next {
			Some(next) => {
				next.borrow_mut().prev = if let Some(prev) = &self.prev {

					Some(Rc::clone(&prev))
				} else {
					None
				}
			},
			None => {},
		}

		self.prev.take();
		self.next.take();

		*self.cellLength.borrow_mut() -= 1;

	}
}

pub struct CCell {
	first:Dummy,
	last:Dummy,
	pub length:Rc<RefCell<u64>>,
}
impl CCell {	
	pub fn new() -> CCell {
		let DummyObj = Box::new(0);
		let DummyObj_2 = Box::new(0);
		let DummyLen = Rc::new(RefCell::new(0));
		let first = Node::new(DummyObj,Rc::clone(&DummyLen));
		let last = Node::new(DummyObj_2,Rc::clone(&DummyLen));
		first.borrow_mut().next = Some(Rc::clone(&last));
		last.borrow_mut().prev = Some(Rc::clone(&first));
		CCell{
			first:first,
			last:last,
			length:Rc::new(RefCell::new(0)),
		}
	}
	pub fn push(&mut self,obj:ObjPointer) {
		let new = Node::new(obj,Rc::clone(&self.length));
		new.borrow_mut().prev = Some(Rc::clone(&self.first) );

		match &mut self.first.borrow_mut().next {
			Some(first_next) => {
				new.borrow_mut().next = Some(Rc::clone(&first_next) );
				match &mut first_next.borrow_mut().prev {
					Some(first_next_prev) => {
						*first_next_prev = Rc::clone(&new);
					},
					None => {},
				}
				*first_next = Rc::clone(&new);
			},
			None => {},
		};
		*self.length.borrow_mut() += 1;
	}
	pub fn iter(&self) -> ListIteraror {
        ListIteraror::new(self.first.borrow_mut().next.clone())
    }
    pub fn ProIter(&self) -> ProcessingIteraror {
        ProcessingIteraror::new(self.first.borrow_mut().next.clone())
    }
}
pub struct ListIteraror {
    current: Link,
}
 
impl ListIteraror {
    fn new(start_at: Link) -> ListIteraror {
        ListIteraror {
            current: start_at,
        }
    }
}
impl Iterator for ListIteraror {
    type Item = Box<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some(current) => {
                let current = current.borrow();
                result = Some(current.obj.clone());
                current.next.clone()
            },
            None => None,
        };
        result
    }
}

pub struct ProcessingIteraror {
    current: Link,
}
impl ProcessingIteraror {
    fn new(start_at: Link) -> ProcessingIteraror {
        ProcessingIteraror {
            current: start_at,
        }
    }
}
impl  Iterator for ProcessingIteraror {
	type Item = Box<i32>;
    fn next(&mut self)  -> Option<Self::Item> {
    	let mut result = None;
        let _next = match &mut self.current {
        	Some(current) => {

        		let current = current.borrow_mut();
            	result = Some(current.obj.clone());
        		current.next.clone()
        	},
        	None => None,
        };

        match &mut self.current {
        	Some(current) => {
        		let mut current = current.borrow_mut();
        		if *current.obj < 0 {
        			current.remove();
        		}
        	},
        	None => {},
        };
        self.current = _next;
        result
    }
}