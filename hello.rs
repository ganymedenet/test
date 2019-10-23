use std::time::{Instant};

fn main(){
 
let now = Instant::now();

let mut n = 1; 
let mut x:i128 = 5;

while n < 99 {

x = x * n;
n += 1;
} 
  
println!("The value of x is: {}\n", x);
println!("{}", now.elapsed().as_secs());

}


