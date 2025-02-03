fn area(dimens: (u32, u32)) -> u32 {
    dimens.0 * dimens.1
}


fn main()
{
    let rect: (u32, u32) = (23, 67);
    println!("Area is {}", area(rect));
}
