mod math_operations;
use math_operations::math_operations::basic;

fn main() {
   let x: i32 = 10;
   let y: i32 = 5;

   let sum: i32 = basic::add(x, y);
   let difference: i32 = basic::sub(x, y);
   let mul: i32 = basic::mul(x, y);
   let div: i32 = basic::div(x, y).unwrap();
   let modulus: i32 = basic::modulus(x, y);
   
   println!("{x} + {y} = {sum}");
   println!("{x} - {y} = {difference}");
   println!("{x} * {y} = {mul}");
   println!("{x} / {y} = {div}");
   println!("{x} % {y} = {modulus}");
}
