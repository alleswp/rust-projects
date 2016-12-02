fn main() {
    print_number(5);
    let x = add_one(5);
    println!("X plus one is: {:?}", x);
    
    //function pointers: both work typed & inferred
    let f: fn(i32) -> i32 = add_one;
    //let f = add_one;

    let addedone = f(10);
    println!("addedone plus one is: {:?}", addedone);
}

fn print_number(x: i32) {
    println!("x is {:?}", x);
}

fn add_one(x: i32) -> i32 {
    x + 1
}


