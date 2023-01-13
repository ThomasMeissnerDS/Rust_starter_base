/*
Write a function which will accept a tuple called point representing the x-axis and y-axis
coordinates of a point. The function will compute the distance of the point from the origin and
will return the computed distance.

The template of the function is given below

fn print_distance(point: (f32, f32)) -> f32 {

// your code here

}

Inside the function, first destructure the tuple into (x,y). This provides a better readability
instead of using point.0 and point.1. Next compute the distance from the original using the formula
 of √(x − 0)2 + (y − 0)2.  you may consider using the following two functions 1).  x.powf(2.0) for
 computing the square of the number x. 2). x.sqrt() which will compute the square root of a number x.


Test the program with the following main program


fn main() {

println!("The distance of the point from the origin is {}", print_distance((5.0,4.0)));

// Note: we need to enclose the inputs to the function in double paranthesis, i.e., print_distance((5.0,4.0)).

// This is becuase a single paranthesis will mean two inputs of 5.0 and 4.0 and since the function has one

// input which is a single tuple therefore the compiler will complain.  }
 */

fn print_distance(point: (f32, f32)) -> f32 {

};

// your code here

fn main() {

    println!("The distance of the point from the origin is {}", print_distance((5.0,4.0)));