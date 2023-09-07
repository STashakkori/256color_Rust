// 256color_Rust
// By: $t@$h, QVLx Labs

use rand::Rng;
use std::process::Command;
use std::env;

fn main() {
  let test = String::from("  ");
  println!();
  
  for i in 232..256 {
    let colored_test = ansi_system_string(test.clone(),i);
    print!("{}", colored_test);
  }

  println!();
  println!();

  for i in 0..8 {
    let colored_test = ansi_system_string(test.clone(),i);
    print!("{}", colored_test);
  }  
  println!();
  
  for i in 8..16 {
    let colored_test = ansi_system_string(test.clone(),i);
    print!("{}", colored_test);
  }  
  println!();
  //println!();

  for i in 0..6 {
    println!();
    for j in 0..6 {
      for k in 0..6 {
        let colored_test = ansi_rgb_string(test.clone(),i,j,k);
          print!("{}", colored_test);
      }
    }
  }
}

fn ansi_system_string(s: String, color: u32) -> String {
  let mut ansi_code = String::from("\x1b[48;5;");
  ansi_code.push_str(&color.to_string()[..]);
  ansi_code.push_str("m");
  ansi_code.push_str(&s);
  ansi_code.push_str("\x1b[0m");
  return ansi_code; 
}

fn ansi_rgb_string(s: String, r: u32, g: u32, b: u32) -> String {
  if r > 6 || g > 6 || b > 6 { return s; }
  let mut ansi_code = String::from("\x1b[48;5;");
  let color = 16 + (r * 36) + (g * 6) + b;
  ansi_code.push_str(&color.to_string()[..]);
  ansi_code.push_str("m");
  ansi_code.push_str(&s);
  ansi_code.push_str("\x1b[0m");
  return ansi_code; 
}

fn print_colorful(s: String) {
  let ansi_code = String::from("\x1b[38;5;");
  let mut colored_string = String::new();
  let raw = s.chars();
  for (_i,character) in raw.enumerate() {
    let r = rand::thread_rng().gen_range(1..6);
    let g = rand::thread_rng().gen_range(1..6);
    let b = rand::thread_rng().gen_range(1..6);
    let non_system = 16 + (r * 36) + (g * 6) + b;
		let system_color = rand::thread_rng().gen_range(1..9);
		let shuffler = rand::thread_rng().gen_range(0..7);
    let mut code = non_system;
    if shuffler > 5 { code = system_color; }
    colored_string.push_str(&ansi_code);
		colored_string.push_str(&code.to_string()[..]);
		colored_string.push_str("m");
		colored_string.push_str(&character.to_string());
		colored_string.push_str("\x1b[0m"); 
  }
  println!("{}", colored_string);
}
