
fn reverse( pair: ( i32, bool ) ) -> ( bool, i32 ) {

    //destructuring pair
    let ( integer, boolean ) = pair;

    ( boolean, integer )
}

#[ derive( Debug ) ]
struct Matrix( f32, f32, f32, f32 );

fn main() {
    let matrix = Matrix( 1.1, 1.2, 2.1, 2.2 );
    println!( "{:?}", matrix );

    let tuple = ( 1, "hello", 4.5, true );
    let ( a, b, c, d ) = tuple;
    println!( "{:?}, {:?}, {:?}, {:?}", a, b, c, d );

    let pair = ( 1, true );
    println!( "pair is {:?}", pair );
    println!( "reversed pair is {:?}", reverse( pair ) );
    
}
