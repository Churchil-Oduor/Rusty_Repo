fn main()
{
    let x = 10;
    let x = x + 30; // this value of x shadows the value of the first
                    // declaration
    
    {
        let x = x * 2;
        println!("The value of the inner block x is {x}");
    }

    println!("The value of the outer block x is {x}");
}
