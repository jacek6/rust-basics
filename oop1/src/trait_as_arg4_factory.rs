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

struct FactoryAndStore {
    p2d: Point,
    p3d: Point3D,
    already_build: bool,
    use2d: bool
}

impl FactoryAndStore {

    fn new() -> FactoryAndStore {
        FactoryAndStore {p2d: Point {x: -1, y: -1}, p3d: Point3D {x: -1, y: -1, z: -1}, already_build: false, use2d: false}
    }
    
    fn build_or_get(&mut self) -> &dyn Lengthable {
        if !self.already_build {
            if rand::random::<bool>() {
                self.p2d = Point {x: 0, y: 0};
                self.use2d = true;
            } else {
                self.p3d = Point3D {x: 0, y: 0, z: 0};
                self.use2d = false;
            }
            self.already_build = true;
        }
        if self.use2d {
            return &self.p2d;
        } else {
            return &self.p3d;
        }
    }
}

/* FAILS:
struct FactoryAndStore {
    p2d: Option<Point>,
    p3d: Option<Point3D>,
    already_build: bool
}

impl FactoryAndStore {

    fn new() -> FactoryAndStore {
        FactoryAndStore {p2d: None, p3d: None, already_build: false}
    }
    
    fn build_or_get(&mut self) -> &dyn Lengthable {
        if !self.already_build {
            if rand::random::<bool>() {
                self.p2d = Some(Point {x: 0, y: 0});
            } else {
                self.p3d = Some(Point3D {x: 0, y: 0, z: 0});
            }
            self.already_build = true;
        }
        match self.p2d {
            //Some(p) => &self.p2d.unwrap(),
            Some(p) => &p,
            None => &self.p3d.unwrap()
        }
    }
}
*/

struct FactoryAndStore2 {
    p2d: Option<Point>,
    p3d: Option<Point3D>,
    already_build: bool
}

impl FactoryAndStore2 {

    fn new() -> FactoryAndStore2 {
        FactoryAndStore2 {p2d: None, p3d: None, already_build: false}
    }
    
    fn build_or_get(&mut self) -> &dyn Lengthable {
        if !self.already_build {
            if rand::random::<bool>() {
                self.p2d = Some(Point {x: 0, y: 0});
            } else {
                self.p3d = Some(Point3D {x: 0, y: 0, z: 0});
            }
            self.already_build = true;
        }
        if let Some(ref p) = self.p2d {
            return p;
        }
        if let Some(ref p) = self.p3d {
            return p;
        }
        panic!("shold never go here");
    }
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

fn factory_calls() {
    let mut f1 = FactoryAndStore::new();
    let mut f2 = FactoryAndStore::new();
    
    measure_dyn1(f1.build_or_get(), f2.build_or_get());
    // measure_dyn1(f1.build_or_get(), f1.build_or_get()); // FAILS
    
    let p1 = f1.build_or_get();
    let p2 = f2.build_or_get();
    
    measure_dyn1(p1, p2);
}

fn factory_calls2() {
    let mut f1 = FactoryAndStore2::new();
    let mut f2 = FactoryAndStore2::new();
    
    measure_dyn1(f1.build_or_get(), f2.build_or_get());
    // measure_dyn1(f1.build_or_get(), f1.build_or_get()); // FAILS
    
    let p1 = f1.build_or_get();
    let p2 = f2.build_or_get();
    
    measure_dyn1(p1, p2);
}

pub fn main_trait_as_arg4() {
    println!("Hello, world!");
    short_calls();
    rand_calls();
    factory_calls();
    factory_calls2();
}
