struct Person
{
    name: String,
    age: u8,
    email: String,
}

//creates and returns a person instance
fn create_person(name: &str, age: u8, email: &str) -> Person {
    Person
    {
        name: name.to_string(),
        age,
        email: email.to_string(),
    }
}

//prints the person instance
fn display_details(person: &Person)
{
    println!("Name: {}\nAge: {}\nEmail: {}", person.name, person.age, person.email); 
}


fn main()
{
    let name: &str = "Churchil";
    let age: u8 = 10;
    let email: &str = "churchilokechduor@gmail.com";
    let person: Person = create_person(name, age, email);
    display_details(&person);

}
