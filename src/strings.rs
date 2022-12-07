fn strings() {
    // has fixed size and cannot be mutated
    let fix_string: &str = "An immutable fixed length string";

    println!("Our string is {}.", fix_string);

    let mut other_string: Str = String::from("This is the other string and can grow");
    println!("Our string is {}.", other_string);

    other_string.push("S"); // we can append a single character to this string
    println!("Our string is {}.", other_string);

    other_string.pop(); // we can also remove the last character from a string
    println!("Our string is {}.", other_string);

    other_string.push_str(". - I love strings."); // we can also add multiple characters
    println!("Our string is {}.", other_string);


    // Special functions around string
    println!("Is our string empty? - {}.", other_string.is_empty()); // check if empty
    println!("Is string has {} characters.", other_string.len()); // count characters/length
    println!("Is string has {} bytes allocated in memory.", other_string.capacity()); // memory consumption
    println!("Does our string contain 'in use'? -  {}", other_string.contains("In use")); // search for substring

    other_string.push_str("     "); // adding white spaces here to have something for trimming
    other_string.trim(); // We can trim the string
    other_string.trim().len() // we can also chain methods

    // we can also convert numbers to strings
    let number: i32 = 45;
    let num_string: Str = number.to_string(); // conversion to string
    println!("Is this really a string? - {}", num_string == "45");

    // we can also convert other data types to string
    a_char: char = "a";
    let a_string: Str = a_char.to_string();
    println!("We have a string");

    // We can also just convert a string on the fly
    fly_string: Str = "On the fly".to_string();


}

fn main() {
    strings()
}