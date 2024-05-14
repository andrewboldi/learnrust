use std::io;

fn main() {
    let mut n = String::new();

    println!("Enter the n'th fibonacci number you'd like to find: ");
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to parse input!");

    let mut n: u128 = n.trim().parse().expect("Please enter a valid integer!!!");

    let mut prev = 0;
    let mut curr = 1;

    while n > 0 {
        let tmp = curr;
        curr += prev;
        prev = tmp;

        n -= 1;
    }

    println!("{curr}");
}
