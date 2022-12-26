pub fn run() {
    // non-mutable closure
    let mut x = 10;
    let mut closure = |y: i32| x += y;
    closure(5);
    println!("{:?}", x);

    // move closure
    let x: i32 = 10;
    let closure = move |y: i32| -> i32 {
        let mut x = x.clone();
        x += y;
        x
    };
    let x = closure(5);
    println!("{:?}", x);

    // mutable closure
    let mut x: i32 = 10;
    {
        let mut closure = |y: i32| x += y;
        closure(5);
    }
    println!("{:?}", x);

    // mutable move closure
    let mut x: i32 = 10;
    {
        let mut closure = move |y: i32| -> i32 {
            // move keyword creates a snapshot of x
            x += y;
            x
        };
        let y = closure(5);
        x += y;
    }
    println!("{:?}", x);

    // move close take ownership of x
    let x = String::from("hello");
    let closure = move || {
        // move keyword creates a snapshot of x
        println!("{:?}", x);
    };
    closure();
    //println!("{:?}", x);

    //
}
