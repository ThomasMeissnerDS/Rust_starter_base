#[derive(Debug)]
pub struct LinkedList <T> {
    data: T,
    next: Option<Box<LinkedList<T>>>, // Box here means: if there is nothing it only takes the size of a data pointer
    // if we write this as next: Option<LinkedList<T>> then it will not compile as every linked list
    // would require to reserve the same amount of memory on the heap again
}

impl <T:std::ops::AddAssign> LinkedList<T> { // enables us to do plus-equals operations as we are dealing with type T
    pub fn add_up(&mut self, n:T) {
    self.data += n
}

}

fn string_find_f(s: &str) -> &str { // String is nothing, but str in a box: Box<str> // we receive and return pointer of a str
    for (n, x) in s.char_indices() {
        if x == 'f' {
            return &s[n..]
        }
    } return s
}

fn choose_str(n: i32) -> &'static str { // static means it will exist as long as the program does (otherwise it does not know how long the str lives
    match n {
        0 => "hello",
        1 => "goodbye",
        _ => "other"
    }
}

fn main() {
    let mut ll = LinkedList {
        data: 3,
        next: Some(Box::new(LinkedList{data: 2, next: None}))
    };
    if let Some(ref mut v) = ll.next {
        v.add_up(10)
    }
    println!("{:?}", ll);

    let mut v: Vec<String> = Vec::new(); // this is just a list..can change size when needed
    v.push("New entry".to_string());
    v.push("New entry 2".to_string());
    println!("Vector is {:?} with length {} and capacity of {}", v, v.len(), v.capacity()); // capacity is 4 here (can hold 4 objects)

    let mut v: Vec<String> = Vec::with_capacity(100); // this is just a list..can change size when needed
    v.push("New entry".to_string());
    v.push("New entry 2".to_string());
    println!("Vector is {:?} with length {} and capacity of {}", v, v.len(), v.capacity()); // capacity is 100 here (as defined)

    for i in 0..105 {
        v.push(i.to_string())
    }
    println!("Vector with length {} and capacity of {}", v.len(), v.capacity()); // capacity is 200 here now (vecs usually store 2 times of what they need)
    // when we exceeded 100 it had to occupy and even bigger place in memory, copy all values into there and then add the new element

    let s = " hello "; // s sits on the stack here
    let p = s.trim();// here p is a substring of s that points further in and has less length allocated
    // p has pointer and length and that makes it a slice instead of a vec

    let _s = " hello ".to_string(); // here s is allocated on the heap
    println!("p == {}", p);

    let fstr = "I love Rust";
    let ffstr = string_find_f(fstr);
    println!("ffstr == {}", ffstr);
    println!("choose_str == {}", choose_str(1));

}