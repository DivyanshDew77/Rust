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