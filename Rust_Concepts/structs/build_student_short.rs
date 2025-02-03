struct Student {

    username: String,
    reg_no: String,
    course: String,
    numerical_identifier: u64,
}

fn main()
{

    let _username: String = String::from("Churchil");
    let _reg_no: String = String::from("TIE124");
    let _course: String = String::from("Telecommunication and Information Engineering");
    let _id: u64 = 12345566;

    let stud1 = build_student(_username, _reg_no, _course, _id);
    

}

fn build_student(username: String, reg_no: String, course: String, numerical_identifier: u64) -> Student {
    Student {
        username,
        reg_no,
        course,
        numerical_identifier,
    }
}
