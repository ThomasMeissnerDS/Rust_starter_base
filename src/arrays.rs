fn arrays() {
    // arrays contain variables of the same type
    // at compile time the length must be known
    let mut my_array: [i32; 5] = [1, 2, 3, 4, 5];

    println!("The value of idx 0 is {} and the idx one is {}.", my_array[0], my_array[1]);
    println!("We  can also print the whole array using {:?}.", my_array);


    // we can also uppdate individual elements of the array
    my_array[0] = 0;

    // we can also create an array with same elements
    let all_ones = [1; 10];
    let mut str_array: [&str; 3] = ["A", "B", "C"];
    let all_same_str = ["Same str"; 5];

    // there are also array slices which do not copy, but reference elements of array
    let array_slice = &str_array[0..2]; // the & indicates the reference to the other array
    println!("We  can also print the whole array using {:?}.", array_slice);
    // we can also include the last index
    let array_slice = &str_array[0..=2]; // the & indicates the reference to the other array
    println!("We  can also print the whole array using {:?}.", array_slice);
    // Note: Slices cannot be used to update the array!

    // we can also print the array's size in memory
    println!("The array uses {} bytes in memory.", std::mem::size_of_val(&str_array));

    // we can also safely try to access a certain index
    idx_check = str_array.get(index=25); // returns none or some
    println!("This should be none: {:?}.", idx_check);
}

fn main() {
    tuples()
}