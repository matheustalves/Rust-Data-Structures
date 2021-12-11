struct User {
    username: String,
    age: u32,
}

struct Cell<T> {
    data: T,
    next: Option<&Box<Cell<T>>>,
}

struct List<T> {
    first: Cell<T>,
    last: Cell<T>,
}

fn new_list<T>(data: T) -> List<T> {
    let cell = Cell { data, next: None };
    List {
        first: cell,
        last: cell,
    }
}

impl<T> List<T> {
    fn insert_start(&mut self, data: T) {
        let cell = Cell {
            data,
            next: Some(Box::new(self.first)),
        };
        self.first = cell;
    }
}

fn main() {
    let user1 = User {
        username: String::from("matheustalves"),
        age: 24,
    };

    let user2 = User {
        username: String::from("anthonykiedis"),
        age: 59,
    };

    let list = new_list(user1);
    list.insert_start(user2);
}
