use std::time::Instant;
use stats::{Stats, MB};
use rand::Rng;

fn main() {
    let char_count = 10 * MB;
    let timer = Instant::now();

    let mut rnd = rand::thread_rng();
    let chars = (0..char_count.to_owned())
        .map(|_| rnd.gen_range(1u8..127) as char)
        .collect::<String>();

    println!("{}", Stats::new("Random_chars", timer.elapsed(), char_count));
}
