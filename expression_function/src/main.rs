fn main() {
    let x = plus_one(5);
    
    println!("The value of x is: {}",x);

}

fn plus_one(x: i32) -> i32 {   // "-> i32" means return value type
    x+1       // This is expression
}
