use std::io;

fn main() {
    // let tuple: (i32, f64, u8) = (500, 6.4, 1);
    // let (x, y, z) = tuple;
    // println!("The value of x is {x}");
    // println!("The value of y is {y}");
    // println!("The value of z is {z}");
    // let x = tuple.0;
    // let y = tuple.1;
    // let z = tuple.2;
    // println!("The value of x is {x}");
    // println!("The value of y is {y}");
    // println!("The value of z is {z}");

    // let a = [1, 2, 3, 4, 5];
    // let months = [
    //     "January",
    //     "February",
    //     "March",
    //     "April",
    //     "May",
    //     "June",
    //     "July",
    //     "August",
    //     "September",
    //     "October",
    //     "November",
    //     "December",
    // ];
    // let b: [i32; 5] = [1, 2, 3, 4, 5];
    // let c = [3; 5]; // 3 3 3 3 3
    // let first = a[0];
    // let second = a[1];

    let a: [u8; 5] = [1, 2, 3, 4, 5];

    println!("Please enter an array index");
    let mut index: String = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element: u8 = a[index];

    println!("The value of the element at {index} is {element}");
}
