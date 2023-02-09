struct Sedan;
impl LandCapable for Sedan {}

struct SUV;
impl LandCapable for SUV {}

trait LandCapable {
    fn drive(&self) {
        println!("Driving a land capable vehicle");
    }
}

trait WaterCapable {
    fn drive(&self) {
        println!("Driving a water capable vehicle");
    }
}

trait Amphibious: LandCapable + WaterCapable {}

struct Hovercraft;
impl Amphibious for Hovercraft {}
impl LandCapable for Hovercraft {}
impl WaterCapable for Hovercraft {}

fn road_trip_static_dispatch(vehicle: &impl LandCapable) {
    vehicle.drive();
}

// dyn keyword is used to indicate that the type is dynamic
// in case of dynamic dispatch Rust needs a fat pointer, which is basically a pointer to the data and a pointer to the vtable
// vtable is a table of function pointers
fn road_trip_dynamic_dispatch(vehicle: &dyn LandCapable) {
    vehicle.drive();
}

pub fn run() {
    let car = Sedan;
    road_trip_dynamic_dispatch(&car);
    road_trip_static_dispatch(&car);
}
