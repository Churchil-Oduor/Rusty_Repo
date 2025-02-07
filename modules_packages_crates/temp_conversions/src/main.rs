mod utils;

fn main() {
    let temp_deg: i32 = 45;
    let farenh: f32 = utils::conversions::degree_farenh(temp_deg);

    println!("{temp_deg} degrees is {farenh} FarenHeits");

}
