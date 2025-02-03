struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_account: u64,
}

fn main()
{
    let mut user = User {
        active: true,
        username: String::from("Churchil"),
        email: String::from("churchilokechoduor@gmail.com"),
        sign_in_account: 1,
    };

    let username = user.username;
    println!("{username}");
}
