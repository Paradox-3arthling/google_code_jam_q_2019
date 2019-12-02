use std::io;

fn main() {
   let tries: u32 = integer_input();

   for x in 0..tries {
    let val: u32 = integer_input();
    for i in 1..val - 1 {
        if i % 2 != 0 {
            continue;
        }
        let val_1: String = i.to_string();
        let val_2: u32 = val - i;
        let val_2 = val_2.to_string();
        if val_1.contains("4") || val_2.contains("4") {
            continue;
        }
        let last_val: u32 = i + (val - i);
        if last_val == val {
            println!("Case #{}: {} {}", x + 1, i, val - i); 
            break;  
        }
    }
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

