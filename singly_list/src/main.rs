use std::mem;

// #[derive(Debug)]
struct Cell<T> {
    data: T,
    next: Option<Box<Cell<T>>>,
}

pub struct SinglyList<T> {
    first: Option<Box<Cell<T>>>,
    length: u32,
}

impl<T> SinglyList<T> {
    pub fn new() -> Self{
        SinglyList{
            first: None,
            length: 0,
        }
    }

    pub fn len(&self) -> u32 {
        self.length
    }

    pub fn push(&mut self, data: T) {
        let old_first = self.first.take();
        let cell = Cell {
            data,
            next: old_first,
        };
        self.first = Some(Box::new(cell));
        self.length += 1;
    }

    // pub fn get(&self, index: u32) -> Option<&T>{
    //     if index < self.length{
    //         let current_elem = &self.first;
    //         match &self.first{
    //             Some(elem) => Some(&elem.data),
    //             None => None,
    //         }
    //     }else{

    //     }
    // }

    // pub fn insert_at(&mut self, data: T, index: u32){

    // }
}

fn main() {
    let mut list = SinglyList::new();
    list.push(42);
    list.push(66);

    // match list.get(0){
    //     Some(elem) => println!("element: {}", elem),
    //     None => println!("No element."),
    // };

    println!("{}", list.len());
}
