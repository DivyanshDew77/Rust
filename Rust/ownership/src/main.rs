// fn main() {
//     // let mut s = String::from("hello");

//     // s.push_str(",world!");
    
//     // println!("{s}");

//     // let s1 = String::from("hello");
//     // let s2 = s1;

//     // println!("{s1}, world!");

//     // let mut s = String::from("hello");
//     // s= String::from("Namaste");

//     // println!("{s}, world!");

//     let s1 = String::from("Hello");
//     let s2 = s1.clone();

//     println!("s1 = {s1}, s2 = {s2}, {s2}");
// }
// fn main() {
//     let mut name = "Lo mai aa gya";

//     println!("{name}");
// }
fn main() {
    // let x = "hello";
    // takes_ownership(x);
    // println!("{}", x);

    // let x = gives_ownership();

    // println!("{}", x);

    // let s1 = String::from("hello");
    // let len = calculate_length(&s1);
    // println!("The length of '{}' is {}", s1, len);

    let mut s = String::from("hello");

    let r1 =  &mut s;
    println!("r1 is {r1}");
    s.push_str(" ,world!");
    println!("r1 is {r1}");

    let r2 = &mut s;
    println!("r1 is {r1}");
    println!("r2 is {r2}");


}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn gives_ownership() -> String {
    let s = String::from("hello");

    s
}

fn calculate_length(s: String) -> usize {
    let length = s.len();
    length
}