// Variables and Mutability //
// fn main() {
//     let mut x = 57355;

//     println!("the value of x is: {}", x);
//     x = 8;
//     println!("the value of x is: {}", x);

//     let tup = ("Let's Get Rusty!", 100_000);
//     let (channel, sub_count) = tup;

//     println!("the channel is {}", channel);
//     println!("the sub_count is {}", sub_count);

//     let byte = [0; 8];

//     println!("{}", byte[3]);
// }

// FUNCTIONS //

// fn main() {
//     let sum = my_function(11,22);
//     println!("the sum is {}", sum);

// }

// fn my_function(x : u32, y: u32) -> u32 {
//     println!("The value of x is {}", x);
//     println!("the value of y is {}", y);
//     x + y
// }

// CONTROL FLOW //

fn main() {
    let number = 5;

    if number > 10 {
        println!("the first condition was true");
    } else if number < 22 {
        println!("the second condition was true")
    } else {
        println!("All the conditions were false");
    }

    let condition = true;
    let number = if condition {5} else {6};

    println!("the number is {}", number);

    loop {
        println!("kgkh");
        break;
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };
    println!("THe result is {}", result);

    let mut yok = 3;

    while yok != 0 {
        println!("{}!", yok);

        yok -= 1;
    }

    println!("Start!!!!!!");

    let a = [10,20,30,40,50];

    for element in a.iter() {
        println!("The value is {}", element);
    }

    for number in 1..4 {
        println!("{}!", number);
    }
}