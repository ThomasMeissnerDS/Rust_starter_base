#[derive(Debug)]
pub struct LinkedList <T> {
    data: T,
    next: Option<Box<LinkedList<T>>>, // Box here means: if there is nothing it only takes the size of a data pointer
    // if we write this as next: Option<LinkedList<T>> then it will not compile as every linked list
    // would require to reserve the same amount of memory on the heap again
}



fn main() {
    let ll = LinkedList {
        data: 3,
        next: Some(Box::new(LinkedList{data: 2, next: None}))
    };
    println!("{:?}", ll);
}