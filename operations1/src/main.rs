use std::ops::{Add,Mul};

// #[derive(Debug, Copy, Clone, PartialEq)]
#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, other: i32) -> Point {
        Point {
            x: self.x * other,
            y: self.y * other,
        }
    }
}


fn main() {
    let p1 = Point {x: 1, y: 5};
    let x = 0;
    let y = 0;
    let p2 = Point {x, y};
    let p3 = Point {..p2};
    let p3 = Point {..p1};
    println!("Hello {p1:?}");
    println!("{p2:?} {p3:?}");
    
    println!("{:?}", p2 + p3);
    println!("{:?}", p1 * 10);
    
    // println!("This makes a bug. p1 is borrowed above {p1:?}");
    let p1 = Point {x: 1, y: 5};
    let p2 = p1;
    println!("Hello {p2:?}");
    //println!("This makes a bug. p1 is borrowed above {p1:?}");
    
    let p1 = Box::new(Point {x: 1, y: 5});
    let p2 = p1;
    println!("Hello {p2:?}");
    // println!("This makes a bug. p1 is borrowed above {p1:?}");
}
