fn main() {
    println!("Hello, world!");

    another_function();
    other_function(5,6);
}


fn another_function(){
    println!("Another Function.");
}

fn other_function(x: i32,y: i32){
    println!("x= {},y= {}",x,y);
}
