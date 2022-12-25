pub fn run() {
    let mut s = String::from("hello");
    takes_ownership(&mut s); // mutable reference
    println!("{}", s); // error: value borrowed here after move

    let x = 5; // Copy types interger, boolean, float, char
    makes_copy(x);
    println!("{}", x); // 5

    let s1 = gives_ownership();
    println!("{}", s1); // hello

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("{}", s3); // hello
}

fn takes_ownership(some_string: &mut String) {
    some_string.push_str(" world");
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
