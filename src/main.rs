use rand::Rng;
use std::char;

fn main() {
  let mut password = String::new();

  while password.len() < 32 {
    password.push(
      char::from_u32(
        rand::thread_rng().gen_range(33..=126)
      ).unwrap()
    )
  }

  println!("The generated password is: {password}")
}
