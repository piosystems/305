//Error handling
//See my slides for references
pub fn run(){

    //panic!("Problem. You called panic");

    //Illustrate Some
    let mut v = vec!["a", "b", "c"];

    //pop an element from the vector
    let x = v.pop();

    //Use match
    match x {
        Some(value) => println!("Popped {}", value),
        None => println!("Your vector is empty"),
    }
    //compare above to:
    let mut v2: Vec<&str> = Vec::new();

    //let mut y2 = v2.pop().unwrap(); //will panic without message because it returns an unhandled None
    //let mut y2 = v2.pop().expect("Do not call pop on an empty Vector"); //will panic and send a message

    //Exercise: How can you ensure that your program does not panic when you call a function that returns an Option?
    let y2 = match v2.pop() {
        Some(val) => val,
        None => "Empty vector",
    };
    println!("{}", y2);

    //let's use ? for Option
    let mut v3 = vec![1,2,3];

    let mut plus_one = || -> Option<i32> { Some(v3.pop()? + 1) };

    println!("Plus one: {}", plus_one().unwrap());
}