fn main() {
    let mut x = 17;
    println!("x = {}", x);
    let y = my_func();
    println!("y = {}", y);
    let result = calc(10, 5);
    println!("result = {}", result);
}

fn my_func() -> i32 {
    println!("hello from my_func");
    let x = 9;
    x
}

fn calc(x: i32, y: i32) -> i32 {
    let result = x * y;
    result
}