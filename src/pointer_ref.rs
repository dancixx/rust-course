// Reference Pointers - Point to a resource in memory

pub fn run() {
    // Primitive Array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("Values: {:?}", (arr1, arr2));

    // With non-primitives, if you assign another variable to a piece of data, the first variable will no longer hold that value. You'll need to use a reference (&) to point to the resource.

    // Vector
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;

    println!("Values: {:?}", (&vec1, vec2));

    // Reference Pointers
    let v = vec![1, 2, 3, 4, 5];
    let w = v.clone();

    let v_ptr = v.as_ptr();
    let w_ptr = w.as_ptr();

    println!("v_ptr: {:p}", v_ptr);
    println!("w_ptr: {:p}", w_ptr);

    let v_ref = &*v;
    let w_ref = &*w;

    println!("v_ref: {:p}", v_ref);
    println!("w_ref: {:p}", w_ref);

    let v = vec![1, 2, 3];
    let v_rc = std::rc::Rc::new(v); // create an Rc<Vec<i32>> pointing to v
    let w_rc = v_rc.clone(); // clone the Rc<Vec<i32>>

    println!("v = {:p}", v_rc);
    println!("w = {:p}", w_rc);
}
