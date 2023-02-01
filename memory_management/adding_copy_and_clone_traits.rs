#[derive(Debug, Clone)]
pub struct Person {
    name: String,
    age: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point{x, y}
    }
}

fn main() {
    let mut x: i32 = 5;
    let y: i32 = x; // x gets copied into y as a deep copy (both on stack)
    x += 5;
    println!("y = {}, x = {}", y, x);

    let mut p = Person {
        name: "Thomas".to_string(),
        age: 50,
    };

    // let p2 = p; This would invalidate p as we borrow p to p2
    let p2 = p.clone(); // needed Clone in derive avove and will do a deep copy now
    p.name.push_str(" the data person"); // now we can change p independently of p2 as they are have separate memory allocations in the heap
    println!("p = {:?}, p2 = {:?}", p, p2);

    let mut pnt = Point::new(3, 4);
    let pn2 = pnt;
    pnt.x += 3;
    println!("p = {:?}, p2 = {:?}", pnt, pn2); // This works because we can use #[derive(Debug, Clone, Copy)] in the class. Adding
    // the copy & clone traits is only possible, because the struct only contains attributes with known size at compile time
}