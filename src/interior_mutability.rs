use std::{
    cell::{Cell, RefCell},
    sync::{Arc, Mutex, RwLock},
    thread,
};

#[derive(Debug)]
struct NodeCell<'a> {
    val: Cell<i32>,
    adjacent: Vec<&'a NodeCell<'a>>,
}

fn add_one_cell(node: &NodeCell) {
    let curr_val = node.val.get();
    node.val.set(curr_val + 1);
    for adj in node.adjacent.iter() {
        add_one_cell(&adj);
    }
}

/**
 * Cell<T> is useful when you want to mutate data even when there are immutable references to that data
 */
pub fn cell() {
    let a = NodeCell {
        val: Cell::new(1),
        adjacent: vec![],
    };

    let b = NodeCell {
        val: Cell::new(2),
        adjacent: vec![&a],
    };

    let c = NodeCell {
        val: Cell::new(3),
        adjacent: vec![&a],
    };

    add_one_cell(&b);

    dbg!(&a);
    dbg!(&b);
    dbg!(&c);
}

#[derive(Debug)]
struct NodeRefCell<'a> {
    val: RefCell<String>,
    adjacent: Vec<&'a NodeRefCell<'a>>,
}
fn add_one_refcell(node: &NodeRefCell) {
    let mut curr_val = node.val.borrow_mut();
    curr_val.push('!');
    for adj in node.adjacent.iter() {
        add_one_refcell(&adj);
    }
}

/**
 * RefCell<T> is useful when you want to mutate data even when there are immutable references to that data
 * Cell<T> vs RefCell<T> - Cell<T> is for Copy types, RefCell<T> is for non-Copy types
 */
pub fn refcell() {
    let a = NodeRefCell {
        val: RefCell::new(String::from("aaa")),
        adjacent: vec![],
    };

    let b = NodeRefCell {
        val: RefCell::new(String::from("bbb")),
        adjacent: vec![&a],
    };

    let c = NodeRefCell {
        val: RefCell::new(String::from("ccc")),
        adjacent: vec![&a],
    };

    add_one_refcell(&b);

    dbg!(&a);
    dbg!(&b);
    dbg!(&c);
}

#[derive(Debug)]
struct NodeRwLock {
    val: RwLock<String>,
    adjacent: Vec<Arc<NodeRwLock>>,
}
fn add_one_rw_lock(node: &NodeRwLock) {
    {
        /*
         * We want the release of the lock to happen as soon as possible
         * because of this we use a new scope
         */
        let mut curr_val = node.val.write().unwrap();
        curr_val.push('!');
    }
    for adj in node.adjacent.iter() {
        add_one_rw_lock(&adj);
    }
}

/**
 * RWLock<T> for concurrent access
 * Only one thread can write at a time
 * Arc<T> guarantees that the data will not be deallocated as long as there is at least one thread still referencing it
 * RwLock<T> will block the thread if it cannot acquire the lock, but RefCell<T> will panic if someone else has already borrowed the data
 */
pub fn rw_lock() {
    let a = Arc::new(NodeRwLock {
        val: RwLock::new(String::from("aaa")),
        adjacent: vec![],
    });

    let b = Arc::new(NodeRwLock {
        val: RwLock::new(String::from("bbb")),
        adjacent: vec![a.clone()],
    });

    let c = Arc::new(NodeRwLock {
        val: RwLock::new(String::from("ccc")),
        adjacent: vec![a.clone()],
    });

    let t1_b = b.clone();
    let t1 = thread::spawn(move || {
        add_one_rw_lock(&t1_b);
    });

    let t2_c = c.clone();
    let t2 = thread::spawn(move || {
        add_one_rw_lock(&t2_c);
    });

    t1.join();
    t2.join();

    dbg!(&*a);
    dbg!(&*b);
    // dbg!(&c);
}

#[derive(Debug)]
struct NodeMutex {
    val: Mutex<String>,
    adjacent: Vec<Arc<NodeMutex>>,
}
fn add_one_mutex(node: &NodeMutex) {
    {
        /*
         * We want the release of the lock to happen as soon as possible
         * because of this we use a new scope
         */
        let mut curr_val = node.val.lock().unwrap();
        curr_val.push('!');
    }
    for adj in node.adjacent.iter() {
        add_one_mutex(&adj);
    }
}

/**
 * Mutex<T> for concurrent access
 * This a general lock, there is not relock and write lock
 * If the the value is already locked, the thread will block until the lock is released
 */
pub fn mutex() {
    let a = Arc::new(NodeMutex {
        val: Mutex::new(String::from("aaa")),
        adjacent: vec![],
    });

    let b = Arc::new(NodeMutex {
        val: Mutex::new(String::from("bbb")),
        adjacent: vec![a.clone()],
    });

    let c = Arc::new(NodeMutex {
        val: Mutex::new(String::from("ccc")),
        adjacent: vec![a.clone()],
    });

    let t1_b = b.clone();
    let t1 = thread::spawn(move || {
        add_one_mutex(&t1_b);
    });

    let t2_c = c.clone();
    let t2 = thread::spawn(move || {
        add_one_mutex(&t2_c);
    });

    t1.join();
    t2.join();

    dbg!(&*a);
    dbg!(&*b);
    // dbg!(&c);
}
