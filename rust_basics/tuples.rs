fn tuples() {
    // tuples are often used for functions with several outputs
    let my_tuple: (&str, i32) = ("My name", 5000_000);

    println!("The value of idx 0 is {} and the other one is {}.", my_tuple.0, my_tuple.1);
    println!("This can also be printed as {:?}", my_tuple);

    // deconstruct into multiple scalars individually
    let (name, salary) = my_tuple;

    // we can also nest tuples
    let nested_tuple = (4.0, 3, ("My", "Pet"));
    let element: &str = nested_tuple.2.0; // accessing the element of a nested tuple

    // we can also create empty tuples
    let empty_tuple = ();
}

fn main() {
    tuples()
}