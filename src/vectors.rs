fn vectors() {
    // vectors contain variables of the same type
    // they do not have a fixed length
    let mut my_vector: Vec<i32> = vec![0, 1, 2, 3, 4, 5];

    println!("The value of idx 0 is {} and the idx one is {}.", my_vector[0], my_vector[1]);
    println!("We  can also print the whole vector using {:?}.", my_vector);

    my_vector[4] = 100; // also here we can easily update values

    // initialize with all zeros
    let all_zero: Vec<i32> = vec![0; 25];

    let mut str_vec: Vec<&str> = vec!["A", "B", "C"];
    // all same strings
    let all_momo: Vec<&str> = ["momo"; 13];

    //subsets
    let my_subset: &&[i32] = &&my_vector[0..3];
    println!("We  can also print the vector slice {:?}.", my_subset);

    // we can also safely try to access a certain index
    idx_check: Option<&str> = str_vec.get(index=25); // returns none or some
    println!("This should be none: {:?}.", idx_check);

    // we can also add elements to a vectror
    my_vector.push(1000);

    // we can also remove values from specified index
    my_vector.remove(0);

    // check if certain value exists in vector
    println!("Does 500 exist? - {}", my_vector.contains(&500)); // & is used because function requires a reference to a number!

}

fn main() {
    vectors()
}