pub fn ownership_example() {
  let s1 = String::from("Helo Rustaceans!");
  println!("s1: {}", s1);

  // move ownership from s1 to s2
  let s2 = s1;
  println!("s2: {}", s2);

  takes_ownership(s2);
  // println!("{}", s2);
}

pub fn takes_ownership(str: String) {
  println!("str: {}", str);
}