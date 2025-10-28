pub use states::*;
pub mod states;

fn main() {
    println!("Hello, world!");
    State::new_from_disk().unwrap();
}
