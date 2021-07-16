#[derive(Debug)]
struct Calc {
  num1: i32,
  num2: i32,
}

impl Calc {
  fn new() -> Self {
    Calc { num1: 0, num2: 1 }
  }

  fn add(&self) -> i32 {
    self.num1 + self.num2
  }
}

trait Transform {
  fn sub(&self) -> String;

  fn output_sub(&self) {
    println!("{}", self.sub());
  }
}


impl Transform for Calc {
  fn sub(&self) -> String {
    "Done".to_string()
  }
}


#[allow(dead_code, unused_variables)]
fn main() {
  println!("Hello, world!");

  let _a: i32 = 21;
  let mut _b: f32 = 3.14;

  let _a: ([i32; 5]) = [1, 2, 3, 4, 5];
  _b = 1.6;

  let _v: Vec<i32> = Vec::new();

  let name: &'static str = "Simanjeet Singh";
  let fname: &str = &name[..9];
  let lname: &str = &name[10..];
  let size = get_length(&name);

  println!("{1}, {0} : {2}", fname, lname, size);

  let city = String::from("NY");
  take_ownership(city);

  enum Color {
    Red,
    Green,
    Blue,
  }

  println!("{}", Color::Red as i32);

  let age = 16;

  match age {
    18 => println!("age is 18"),
    20 | 21 | 22 => println!("u're teenager"),
    30..=45 => println!("ok boomer!"),
    n if n < 18 => println!("go home kiddo"),
    _ => println!("who ?"),
  }
  let mut i = 0;
  loop {
    if i == 10 {
      break;
    }
    i += 1;
  }

  for i in (0..11).step_by(2) {
    print!("{} ", i);
  }

  let data = Calc { num1: 1, num2: 5 };
  let default = Calc::new();

  dbg!(&data);
  println!("\nSum : {}", data.add());

  let my_fav = Color(255, 127, 10);
  dbg!(&my_fav);

}

#[derive(Debug)]
struct Color(u16, u16, u16);


fn get_length(s: &str) -> usize {
  s.chars().count()
}

fn take_ownership(s: String) {
  println!("{}", s);
}
