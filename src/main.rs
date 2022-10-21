use rand::Rng;
use std::char;
use std::env;

fn print_usage() {
  println!("Invalid call!\nUsage: ./rust-password-generator <password_length>");
}

fn gen_password(length: u32) -> String {
  let mut password = String::new();

  for _ in 1..=length {
    password.push(
      char::from_u32(
        rand::thread_rng().gen_range(33..=126)
      ).unwrap()
    )
  }

  return password;
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() <= 1 {
    print_usage();
    return;
  }

  let length: u32 = match args[1].parse() {
    Ok(length) => length,
    Err(_) => {
      print_usage();
      return;
    }
  };

  let password = gen_password(length);

  println!("The generated password is: {password}")
}
