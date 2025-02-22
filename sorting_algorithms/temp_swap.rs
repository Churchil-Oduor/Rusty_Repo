fn main() {
    let mut arr = [10, 9];

    let temp = arr[0];
    arr[0] = arr[1];
    arr[1] = temp;

    println!("{:?}", arr);

}
