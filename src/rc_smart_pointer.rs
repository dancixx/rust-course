enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::rc::Rc;

pub fn run() {
    // Rc<T> and RefCell<T>
    // Rc<T> is for multiple ownership
    // RefCell<T> is for interior mutability
    // RefCell<T> is only for single-threaded scenarios
    // RefCell<T> is useful for cases when you know the borrowing rules will be followed at runtime, but can't be checked at compile time
    // RefCell<T> is useful when you're sure your code follows the borrowing rules but the compiler is unable to understand and guarantee that
    // RefCell<T> is useful when you want to mutate data even when there are immutable references to that data
    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    // This is not a deep copy, only the reference count is incremented
    let b = List::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = List::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
