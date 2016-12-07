

fn main() {
     println!("Hello, hipstercode!");
    let mut x = 5;
{
    let y = &mut x;
    *y += 1;
}
println!("{}", x);

let z = do_it(5);
println!("{}", z);


}

fn do_it(i: i32) -> i32 {
    let x = i * i;
    return x;
}