use std::io;

fn get_int_input(msg: &str) -> isize {
    println!("{}", msg);    // Format argument must be string literal? interesting

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().parse().expect("Failed to parse number")
}

fn get_nth_fib(n: isize) -> isize {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        let mut last_last: isize = 0;
        let mut last: isize = 1;
        for _ in 2..=n {
            let new_num: isize = last_last + last;
            last_last = last;
            last = new_num;
        };
        last
    }
}

fn main() {
    println!("This program generates the nth Fibonacci number");

    let n = get_int_input("Enter n (0-indexed)");

    let nth_fib = get_nth_fib(n);

    let suffix: &str = match n {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th"
    };

    println!("The {n}{suffix} Fibonacci number is {nth_fib}")

}
