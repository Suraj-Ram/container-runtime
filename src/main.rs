mod friendly_id;

fn main() {
    println!("Hello, world!");
    let c_name = friendly_id::generate();
    println!("Starting container {c_name}");
}
