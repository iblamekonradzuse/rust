fn main (){
    let x: u32 = 5u32;

    let y: u32 = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expressions will be assignet to 'y'
        x_cube + x_squared + x
    };

    let z: u32 = {
        // the semicolon supresses this expression and '{}' is assaigned to 'z'
        2 * x
    };
    println!("x is {:?}", x);
    println!("y is {:?}", y );
    println!("z is {:?}",z);
}