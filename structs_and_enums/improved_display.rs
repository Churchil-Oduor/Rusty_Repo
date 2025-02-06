struct Person
{
    name: String,
    age: u8,
    email: String,
}


impl Person
{
    //creates and returns a person instance
    fn new(name: &str, age: u8, email: &str) -> Person {
        Person
        {
            name: name.to_string(),
            age,
            email: email.to_string(),
        }
    }

    //displays the Person's details
    fn display_details(&self)
    {
        println!("Name: {}\nAge: {}\nEmail: {}", self.name, self.age, self.email);
    }

}



fn main()
{
    
    let person: Person = Person::new("Churchil", 10, "churchilokech@gmail.com");
    person.display_details();
    person.display_details();

}
