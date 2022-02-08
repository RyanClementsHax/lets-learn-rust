fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    // this errors because the value is borrowed after a move
    // println!("{}, world!", s1);
    println!("{}, world!", s2);

    let s1 = String::from("hello");
    let s2 = s1.clone();

    // this is fine cuz we cloned s1 instead of moving it
    println!("s1 = {}, s2 = {}", s1, s2);

    main2();
    main3();
    main4();
    main5();
    main6();
    main7();
    main8();
    main9();
}

fn main2() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    // this breaks because takes_ownership disposed of the variable
    // println!("{}", s);

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn main3() {
    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
    println!("{}{}", s1, s3);
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn main4() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn main5() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    // this breaks because the reference isn't mutable
    // some_string.push_str(", world");
    println!("{}", some_string);
}

fn main6() {
    let mut s = String::from("hello");

    change2(&mut s);
}

fn change2(some_string: &mut String) {
    some_string.push_str(", world");
    println!("{}", some_string);
}

fn main7() {
    let mut s = String::from("hello");
    println!("s before: {}", s);
    s = String::from("world");
    println!("s after: {}", s);

    let r1 = &s; // no problem
    let r2 = &s; // no problem
                 // let r3 = &mut s; // BIG PROBLEM

    println!("{}: {}, and {}", s, r1, r2);
}

fn main8() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

fn main9() {
    // let reference_to_nothing = dangle();
    let reference_to_something = no_dangle();
    println!("{}", reference_to_something);
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
