
mod wallet;
mod connect;

fn main() {
    println!("=== Running All Demos ===\n");

    // Run Fibonacci demo
    println!("1️⃣ Fibonacci Sequence Demo:");
    fibonacci_example();

    // Run Wallet demo
    println!("\n2️⃣ Wallet System Demo:");
    wallet::wallet_example();

    // Run Order Book demo
    println!("\n3️⃣ Order Book System Demo:");
    connect::order_book_demo();

    println!("\n✅ All demos completed successfully!");
}

fn fibonacci_example() {
    let n = 10;
    let mut a = 0;
    let mut b = 1;

    println!("Fibonacci Series up to {} terms:", n);

    for _ in 0..n {
        print!("{} ", a);
        let next = a + b;
        a = b;
        b = next;
    }
    println!();
}
