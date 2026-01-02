fn main() {
    println!("Hello, world!");
    another_func();
    second_func(56);
    print_labeled_measurement(64.3, 'p');

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is {y}");

    let x = five();
    println!("The value of x is {x}");

    let z = inc(12);
    println!("The value of z is {z}");
}

fn another_func() {
    println!("Another function");
}

fn second_func(x: i32) {
    println!("The value of x is {x}");
}

fn print_labeled_measurement(value: f32, unit_label: char) {
    println!("The measurment is {value}{unit_label}");
}

fn five() -> u16 {
    5
}

fn inc(x: i32) -> i32 {
    x + 1
}
