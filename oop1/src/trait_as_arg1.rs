trait Lengthable {
    
    fn get_length(&self) -> i32;
}

// #[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Lengthable for Point {

    fn get_length(&self) -> i32 {
        self.x + self.y
    }
}

impl Lengthable for Point3D {

    fn get_length(&self) -> i32 {
        self.x + self.y + self.z
    }
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32
}

fn measure_static1(l1: impl Lengthable, l2: impl Lengthable) -> i32 {
    let len = l1.get_length() + l2.get_length();
    println!("given total length: {}", len);
    len
}

fn measure_static2<L1, L2>(l1: L1, l2: L2) -> i32
    where L1: Lengthable, L2: Lengthable {
    let len = l1.get_length() + l2.get_length();
    println!("given total length: {}", len);
    len
}

fn measure_dyn(l1: &dyn Lengthable, l2: &dyn Lengthable) -> i32 {
    let len = l1.get_length() + l2.get_length();
    println!("given total length: {}", len);
    len
}

fn short_calls() {
    let x = 1;
    let y = 2;
    let z = 3;
    measure_static1(Point{x, y}, Point3D{x, y, z});
    measure_static2(Point{x, y}, Point3D{x, y, z});
    measure_dyn(&Point{x, y}, &Point3D{x, y, z});
}

pub fn main_trait_as_arg1() {
    println!("Hello, world!");
    short_calls();
}
