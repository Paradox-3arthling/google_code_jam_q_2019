use std::io;

fn main() {
    println!("Insert value");
   let val: u32 = integer_input();
   println!("value input {}", val);
   println!("value {}", u32::max_value());
   for i in 1..val + 1 {
       println!("value {}", i);
   }
}
fn integer_input() -> u32 {
    let mut val: String = String::new();
    io::stdin().read_line(&mut val)
        .expect("Failed to read input!");
    let val: u32 = val.trim().parse()
        .expect("Expected integer!");
    val
}

