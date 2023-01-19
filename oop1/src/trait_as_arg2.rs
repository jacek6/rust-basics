trait Lengthable {
    
    fn get_length(&self) -> i32;
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Drop for Point {
    fn drop(&mut self) {
        println!("Dropping Point: {self:?}");
    }
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

#[derive(Debug)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32
}

impl Drop for Point3D {
    fn drop(&mut self) {
        println!("Dropping Point3D: {self:?}");
    }
}

fn measure_static1(l1: impl Lengthable, l2: impl Lengthable) -> i32 {
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
    let mut counter = 1;
    let mut x = || { counter += 1; counter};
    let y = 2;
    let z = 3;
    measure_static1(Point{x: x(), y}, Point3D{x: x(), y, z});
    measure_dyn(&Point{x: x(), y}, &Point3D{x: x(), y, z});
    println!("last line in short_calls");
}

fn using_box() {
    println!("{:?}", Box::new(Point {x: -2, y: -2}));
    println!("{:?}", Point {x: -1, y: -1});
    
    let p = Box::new(Point {x: 0, y: 0});
    println!("{p:?}");
    let p = Point {x: 0, y: 1};
    println!("{p:?}");
    println!("last line in using_box");
}

fn print_point(p: Point) {
    println!("start print_point");
    println!("{p:?}");
    println!("end print_point");
}

fn using_box_and_unbox() {
    print_point(*Box::new(Point {x: -2, y: -2}));
    
    let b = Box::new(Point {x: 0, y: 0});
    println!("b = {b:?}");
    let p = *b;
    println!("{p:?}");
    println!("last line in using_box_and_unbox");
}

fn using_option() {
    let o = Some(Point {x: 0, y: 0});
    if let Some(p) = o {
        print_point(p);
        println!("go back to if let");
    }
    /* if let Some(p) = o { FAILS
        print_point(p);
        println!("go back to if let");
    } */
    println!("last line in using_option");
}

pub fn main_trait_as_arg2() {
    short_calls();
    using_box();
    using_box_and_unbox();
    using_option();
}
