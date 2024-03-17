# Stats

The simple utility that formats and prints execution speed

## Examples
``` rust
let char_count = 10 * MB;
let timer = Instant::now();

let mut rnd = rand::thread_rng();
let _ = (0..char_count.to_owned())
    .map(|_| rnd.gen_range(1u8..127) as char)
    .collect::<String>();

println!("{}", Stats::new("Random_chars", timer.elapsed(), char_count));
```

`> cargo run --example random_chars --release` <br>
`'Random_chars': processed 10 MB in 25.72 ms at 388.84 mb/s`
