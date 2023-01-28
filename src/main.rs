// mod box_smart_pointers;
mod exercises;
mod interior_mutability;
// mod lifetimes;
// mod move_closure;
// mod ownership_borrow;
// mod pointer_ref;
// mod rc_smart_pointer;
// mod ref_cycles;
// mod refcell_sm;
// mod structs;
// mod vectors;

fn main() {
    // exercises::multithreading::run_sync();
    // exercises::multithreading::run_async_multithread();
    exercises::multithreading::run_local_storage();
    // exercises::multithreading::run_atomic();
    exercises::multithreading::run_tls_atomic();
}
