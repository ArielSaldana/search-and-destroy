use std::println;
use std::time::Instant;

mod destroy;
mod search;

fn main() {
    let now = Instant::now();

    search::search();
    destroy::destroy();

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
