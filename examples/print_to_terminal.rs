// cargo run --example print_to_terminal
// cargo run --example print_to_terminal --release
// cargo run --example print_to_terminal           2> /dev/null
// cargo run --example print_to_terminal --release 2> /dev/null

use std::time::Instant;
use stats::{Stats, MB};

fn main() {
    let len = 1 * MB;

    let timer = Instant::now();
    for _ in 0..len { eprint!("\r"); }
    println!("{}", Stats::new(r#"for ... { eprint!("\r"); }         "#, timer.elapsed(), len));

    let timer = Instant::now();
    eprint!("{}", (0..len).map(|_| "\r").collect::<String>());
    println!("{}", Stats::new(r#"eprint!((0..len).map().collect()); "#, timer.elapsed(), len));

    let timer = Instant::now();
    eprint!("{}", "\r".repeat(len));
    println!("{}", Stats::new(r#"eprint!("\r".repeat(len));         "#, timer.elapsed(), len));
}
