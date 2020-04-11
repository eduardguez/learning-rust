fn main() {
    // This example first creates a tuple and binds it to the variable tup. 
    // It then uses a pattern with let to take tup and turn it into three 
    // separate variables, x, y, and z.
    let tup = (500, 6.4);
    let (x, y) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    // This example creates a tuple, x, and then makes new variables for each 
    // element by using their respective indices.
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let six_point_four = x.1;
    println!("The value of six_point_four is: {}", six_point_four);
}
