fn main() {
  // immutable
  let x = 5;
  println!("The value of x is: {}", x);

  // mutable
  let mut an_integer = 1u32;
  println!("An integer: {:?}", an_integer);
  an_integer = 6;
  println!("Another integer: {:?}", an_integer);

  // shadowing
  let spaces = "   ";
  let spaces = spaces.len();
  println!("The number of spaces is: {}", spaces);
}
