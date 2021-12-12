use std::mem;

#[derive(Debug)]
struct Cell<T> {
    data: T,
    next: Option<Box<Cell<T>>>,
}

pub struct SinglyList<T> {
    first: Option<Box<Cell<T>>>,
}

impl<T> SinglyList<T> {
    pub fn new() -> Self{
        SinglyList{
            first: None,
        }
    }

    pub fn insert_start(&mut self, data: T) {
        let old_first = mem::replace(&mut self.first, None);
        let cell = Cell {
            data,
            next: old_first,
        };
        self.first = Some(Box::new(cell));
    }
}

fn main() {
    // let mut list = new_list(42);
    let mut list = SinglyList::new();
    list.insert_start(79);
    list.insert_start(89);
    print!("{:?}",list.first);
}
