enum List {
    Cons(i32, Rc<List>),
    Nil,
}

struct MyBox<T>(T);

use ::std::ops::Deref;

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

// Drop trait
struct CustomSmartPointer {
    data: String,
}

use ::std::ops::Drop;
use std::rc::Rc;

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

pub fn run() {
    // let mut b = Box::new(5);
    // println!("b = {}", b);

    // *b = 7;
    // println!("b = {}", b);

    // Box is a smart pointer
    // use for heap allocation
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // Deref trait
    // let x = 5;
    // let y = Box::new(x);

    // assert_eq!(5, x);
    // assert_eq!(5, *y);

    // MyBox
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *(y.deref()));

    // Deref coeBoxion
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    hello(&(*m)[..]);
    // DerefMut trait

    // Drop trait
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    drop(c);
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
