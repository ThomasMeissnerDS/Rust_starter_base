pub struct Person {
    name: String,
    age: u32,
    children: u32,
    favourite_color: Color
}

#[derive(Debug)]
pub enum Color { /*enum restricts the choices someone can make*/
    Red(String),
    Green,
    Blue
}

impl Person {
    pub fn print(self) -> String {
        format!("Name: {},Age:  {}, Children: {}, Favourite color: {:?}",
                self.name, self.age, self.children, self.favourite_color)
    }
}

fn main() {
    let p = Person {
        name: "Matt".to_string(),
        age: 32,
        children: 2,
        favourite_color: Color::Green
    };

    let c = Color::Red("reeeeeed".to_string());
    match c {
        Color::Red(s) => println!("The color is {s} of object red..."),
        Color::Green => println!("The color is green..."),
        Color::Blue => println!("The color is blue...")
    }

    println!("Hello, from {}!", p.print());
}