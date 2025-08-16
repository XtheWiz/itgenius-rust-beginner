pub fn lifetime_example() {
  
  
  let str1 = String::from("Programming");
  let str2 = String::from("Rust");

  let result = longest(&str1, &str2);
  println!("The longest string is: {}", result);

  let s1 = String::from("Programming");
  let r;
  let s2 = String::from("Rust");

  {
    r = longest(&s1, &s2);
  }

  println!("{}", r);
}

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x 
  } else {
    y
  }
}