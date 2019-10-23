use std::time::{Instant};

fn main(){
 
let now = Instant::now();

let mut n = 1; 
let mut x:i128 = 1;

while n < 999999 {

x = 999 * 999;

n += 1;
} 
  
println!("The value of x is: {}\n", x);
println!("{}", now.elapsed().as_secs());

}
