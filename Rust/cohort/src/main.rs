// fn main() {
//     let name = String::from("Harkirat");
//     let (len, name) = get_len(name);

//     println!("The length of the string is {}", len);
//     println!("The length of the string is {}", name);
// }

// fn get_len(s: String) -> (usize, String) {
//     return (s.len(),s);
// }

// Borrowing

// fn main() {
//     let mut s1 = String::from("Harkirat");
//     let ref2 = &mut s1;
//     ref2.push_str(" Singh");
//     let ref3 = &s1;

//     println!("{}, {}",s1, ref2 );


// }

//struct


// struct Rect {
//     height: f32,
//     width: f32
// }

// impl Rect {
//     fn area(&self) -> f32 {
//         return self.width * self.height;
//     }
//     fn print_something(a: u32) {
//         println!("Harkirat");
//     }
//     fn perimeter(&self) -> f32 {
//         return 2.0 * ( self.width + self.height);
//     }
// }

// fn main() {
//     let r = Rect {
//         width: 10.0,
//         height: 10.0
//     };
//     println!("The area is {}", r.area());
//     println!("the perimeter is {}", r.perimeter());
//     Rect::print_something(10);
// }

//enum

// enum Shape {
//     Square(f32),
//     Circle(f32),
//     Rectangle(f32,f32)
// }

// enum 

// fn main() {
//     let shape = Shape::Square(10.0);
//     let shape_circle = Shape::Circle(10.0);
//     let shape_rectangle = Shape::Rectangle(10.0,10.0);

//     println!("The perimeter is {}", perimeter(shape));
// }

// fn perimeter(sh: Shape) -> f32 {
//     match sh {
//         Shape:: Square(side) => 4.0 * side,
//         Shape:: Circle(radius) => 3.14 * radius * radius,
//         Shape:: Rectangle(length,breadth) => 2.0 * (length + breadth),
//     }
// }

// use std::fs;

// enum Result {
//     Ok(String),
//     Err(String)
// }

// fn main() {
//     let contents = fs::read_to_string("a.txt");

//     match contents {
//         Ok(contents) => println!("{}", contents),
//         Err(r) => println!("Error while reading file")
//     }
// }

// use chrono::{Utc, Local};
// use dotenv::dotenv;
// use std::env;

// fn main() {
//     dotenv().ok();
//     let utc = Utc::now();
//     let local_time = Local::now();

//     println!("{}", utc);
//     println!( "{}", local_time);

//     let lo = env::var("REDIS_ADDRESS").unwrap();

//     println!("{}",lo);
// }

//Generics and trait bounds

// fn main() {
//     let s1 = sum(1,2);
//     let s2 = sum(1.0,2.0);
// }

// fn sum(a:u32, b:u32) -> u32 {
//     return a + b;
// }

// trait Shape {
//     fn get_area(&self) -> u32;
// }

// struct Rect {
//     width : u32,
//     height : u32
// }

// impl Shape for Rect{
//     fn get_area(&self) -> u32 {
//         &self.width * &self.height
//     }
// }

// struct Circle {
//     radius : u32
// }

// impl Shape for Circle {
//     fn get_area(&self) -> u32 {
//         &self.radius * &self.radius
//     }
// }

// fn main() {
//     let rectangle = Rect {
//         width: 10,
//         height : 30
//     };

//     let circle = Circle {
//         radius : 10
//     };

//     println!("The area of rectangle is {}", rectangle.get_area());
//     println!("The area of circle is {} ", circle.get_area());
// }

//Macros

// use std::fmt::{write,Display};



// struct User {
//     username : String,
//     age : u32
// }

// impl Display for User {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "This is the user struct with age {}", self.age)
//     }
// }
// #[derive(Debug, Copy, Clone)]
// struct User {
//     is_male : bool,
//     age : u32
// }

// fn main() {
//     let u = User {
//         is_male : true,
//         age : 21
//     };

//     let u2  = u;
//     println!("{:?}, {:?}", u, u2);
// }

fn main() {
}