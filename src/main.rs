
fn main(){
    println!("{}", transfer_to_meow(1));
}

fn transfer_to_meow(x: i32) -> i32 {
    println!("You are meowed {}", x);
    let y = x + 114514;
    y
}

