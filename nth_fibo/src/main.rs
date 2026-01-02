use std::io;

fn main() {
    println!("Please input the n: ");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Something went wrong))");
    let n: usize = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Something went wrong during conversion"),
    };

    let result: usize = nth_fibonacci(n);
    println!("Result is {result}");
}

fn nth_fibonacci(n: usize) -> usize {
    if n <= 1 {
        return n;
    }

    let mut current = 0;
    let mut prev1 = 1;
    let mut prev2 = 0;

    let mut i = 2;
    while i <= n {
        current = prev1 + prev2;
        prev2 = prev1;
        prev1 = current;

        i += 1;
    }

    current
}
