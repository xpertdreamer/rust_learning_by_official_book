use rand::Rng;

fn main() {
    let x = rand::rng().random_range(0..10);

    if x < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    if x % 4 == 0 {
        println!("Number is divisible by 4");
    } else if x % 3 == 0 {
        println!("Number is divisible by 3");
    } else if x % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible")
    }

    let condition: bool = true;
    let number: u8 = if condition { 5 } else { 6 };
    println!("The value of number is {number}");

    let mut i = 0;
    loop {
        if i == 10 {
            break;
        }
        println!("again!");
        i += 1;
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remainig = 10;

        loop {
            println!("remainig = {remainig}");
            if remainig == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remainig -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    let mut num = 3;

    while num != 0 {
        println!("num = {num}");
        num -= 1;
    }
    println!("stop while");

    let a = [10, 20, 30, 40, 50];
    for elem in a {
        println!("The value is {elem}");
    }

    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!!!");
}
