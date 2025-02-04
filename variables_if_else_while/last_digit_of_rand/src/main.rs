use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let rand_no = rng.gen_range(-100..100);

    let last_num = rand_no % 10;

    if last_num > 5
    {
        println!("The last digit of {rand_no} is {last_num} and is greater than 5");
    } else if last_num < 5 && last_num > 0
    {
        println!("The last digit of {rand_no} is {last_num} and is less than 5");
    } else
    {
        println!("The last digit of {rand_no} is {last_num} and is less than or equal to zero");
    }


}
