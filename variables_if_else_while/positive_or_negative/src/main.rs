use rand::Rng;

fn main() {
   let mut rnd = rand::thread_rng();
   
   let random_number = rnd.gen_range(-100..100);

   if random_number > 0 { 
       println!("{random_number} is positive");
   } else if random_number < 0 {
       println!("{random_number} is negative");
   } else {
       println!("{random_number} is zero");
   }

}
