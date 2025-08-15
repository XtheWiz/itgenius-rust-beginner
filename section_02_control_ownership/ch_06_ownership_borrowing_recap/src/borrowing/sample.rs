pub fn borrowing_exmaple() {
  let s1 = String::from("Hello Rustaceans !!");

  let s2 = &s1;
  println!("s1: {}", s1);
  println!("s2: {}", s2);

  let len = calculate_length(&s1);
  println!("Length: {}", len);

  let mut r1 = String::from("Rust is Awesome!");

  let r2 = &r1;
  let r3 = &r1;

  println!("r2: {}, r3: {}", r2, r3);

  let r4 = &mut r1;
  r4.push_str(" Programming");
  println!("r4: {}", r4);

  let r5 = &mut r1;
  r5.push_str("is fun !");
  println!("r5: {}", r5);
}

fn calculate_length(s: &String) -> i32 {
  s.len() as i32
}