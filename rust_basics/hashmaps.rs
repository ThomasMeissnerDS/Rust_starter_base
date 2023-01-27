use::std::collections::HashMap;

fn main() {
    let mut bookreviews = HashMap::new();
    bookreviews.insert("Thomas DS".to_string(),
                       "The book".to_string());

    if bookreviews.contains_key("Thomas DS") {
        println!("That was expected");
    }

    let lookup = ["Ludmilla List", "Henry McDinor", "Thomas DS"];
    for &author in &lookup {
        match bookreviews.get(author) {
            Some(review) => println!("{author}: {review}"),
            None => println!("{author} is unreviewed.")
        }
    }

    println!("{:?}", bookreviews)
}