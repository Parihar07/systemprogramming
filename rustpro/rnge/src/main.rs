fn main() {
    println!("Hello, learning range thing in rust programming!");

    let month_days = 1..30;

    println!("{month_days:?}");

    for i in month_days {
        println!("{i}");
    }

    let month_days = 1..=30;
    for i in month_days {
        println!("{i}");
    }

    let A = [1, 2, 3, 4, 56, 7, 3, 8];
    for i in A {
        println!("{i}");
    }

    let ch: std::ops::Range<char> = 'b'..'z';

    for i in ch {
        println!("{i}");
    }
}
