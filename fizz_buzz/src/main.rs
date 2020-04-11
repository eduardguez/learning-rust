use std::io;

fn main() {
  loop {
    let n = read_number();
    match n.parse::<u32>() {
      Ok(n) => {
        fizzbuzz_to(n);
        break;
      }
      Err(..) => continue,
    };
  }
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
  if rhs == 0 {
    return false;
  } 
  lhs % rhs == 0
}

fn fizzbuzz(n: u32) {
  if is_divisible_by(n, 15) {
    println!("fizzbuzz");
  } else if is_divisible_by(n, 3) {
    println!("fizz");
  } else if is_divisible_by(n, 5) {
    println!("buzz");
  } else {
    println!("{:?}", n);
  }
}

fn fizzbuzz_to(n: u32) {
  for n in 1..n + 1 {
    fizzbuzz(n)
  }
}

fn read_number() -> String {
  println!("Please input your limit: ");
  let mut any_string = String::new();
  io::stdin().read_line(&mut any_string).expect("Failed to read line");
  any_string.trim().to_string()
}
