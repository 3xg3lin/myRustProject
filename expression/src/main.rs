fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1   // This shows the return value. if you add semicolons, you turn it into a statement.
    };

    println!("The value of y is: {}",y);
}

