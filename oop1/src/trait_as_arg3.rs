//use rand::prelude::random;

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

fn rand_p_impl() -> Box<dyn Lengthable> {
    if rand::random::<bool>() {
        return Box::new(Point {x: 0, y: 0});
    } else {
        return Box::new(Point3D {x: 0, y: 0, z: 0});
    }
}

fn measure_static1(l1: impl Lengthable, l2: impl Lengthable) -> i32 {
    let len = l1.get_length() + l2.get_length();
    println!("given total length: {}", len);
    len
}

fn measure_dyn1(l1: &dyn Lengthable, l2: &dyn Lengthable) -> i32 {
    let len = l1.get_length() + l2.get_length();
    println!("given total length: {}", len);
    len
}

fn measure_dyn2(l1: Box<dyn Lengthable>, l2: Box<dyn Lengthable>) -> i32 {
    let len = l1.get_length() + l2.get_length();
    println!("given total length: {}", len);
    len
}

fn short_calls() {
    let x = 1;
    let y = 2;
    let z = 3;
    
    measure_static1(Point{x, y}, Point3D{x, y, z});
    measure_dyn1(&Point{x, y}, &Point3D{x, y, z});
    measure_dyn2(Box::new(Point{x, y}), Box::new(Point3D{x, y, z}));
}

fn rand_calls() {
    let p1 = rand_p_impl();
    let p2 = rand_p_impl();
    // measure_static1(*p1, *p2);  // FAILS
    measure_dyn1(&*p1, &*p2);  // looks weird to me
    measure_dyn2(p1, p2);
}

pub fn main_trait_as_arg3() {
    println!("Hello, world!");
    short_calls();
    rand_calls();
}
