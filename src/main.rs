mod box_smart_pointers;
mod lifetimes;
mod move_closure;
mod ownership_borrow;
mod pointer_ref;
mod rc_smart_pointer;
mod ref_cycles;
mod refcell_sm;
mod structs;
mod vectors;

fn main() {
    ref_cycles::run();
}
