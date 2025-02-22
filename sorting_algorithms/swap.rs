fn main()
{
    let mut arr = [5, 1];
    (arr[0], arr[1]) = (arr[1], arr[0]);

    println!("{:?}", arr);
}
