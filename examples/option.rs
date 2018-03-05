extern crate panic_search;

fn main() {
    panic_search::enable();

    None::<i32>.unwrap();
}
