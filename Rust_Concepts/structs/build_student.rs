struct Student {

    username: String,
    reg_no: String,
    course: String,
    numerical_identifier: u64,
}

fn main()
{

    let username: String = String::from("Churchil");
    let reg_no: String = String::from("TIE124");
    let course: String = String::from("Telecommunication and Information Engineering");
    let id: u64 = 12345566;

    let stud1 = build_student(username, reg_no, course, id);
    

}

fn build_student(username: String, reg_no: String, course: String, num_identity: u64) -> Student {
    Student {
        username: username,
        reg_no: reg_no,
        course: course,
        numerical_identifier: num_identity,
    }
}
