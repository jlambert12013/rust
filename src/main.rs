#[allow(unused)]

fn main() {
    string_lesson()
}

fn string_lesson() {
    //      Basic Strings
    // let s1: &str = "Hello, World!";
    // let s2: String = String::from("Hello, world!");
    // let s3: String = "Hello, World!".to_string();
    // let s4: String = "Hello, World!".to_owned();
    // let s5: &str = &s4[0..4];
    // println!("{}", s5);

    //      String methods
    // let mut some_string = String::from("foo");
    // println!("{}", some_string);

    // some_string.push_str("bar");
    // println!("{}", some_string);

    // some_string.replace_range(.., "foo bar");
    // println!("{}", some_string);

    //      Concatenate String
    //      Note: we must reference string two & note that string one is moved into string ones spot in memory.
    // let string_one = String::from("Hello, ");
    // let string_two = String::from("world!");
    // let string_three = string_one + &string_two;
    // println!("{}", string_three)

    //      Format Marco
    //      this marco uses string interp
    //      call with format!
    //      can take String Type or String slices
    // let tic = String::from("tic");
    // let tac = String::from("tac");
    // let toe = String::from("toe");

    // let game = format!("The game is called {}-{}-{}.", tic, tac, toe);
    // println!("{game}");

    //      More concatenate strings..
    //      The first returns a String Type while the second return the &str string slice.
    let full_name = ["Jim ", "Lambert"].concat();
    let spouse_name = concat!["Madi ", "Lambert"];
    println!("{}", full_name);
    println!("{}", spouse_name);
}