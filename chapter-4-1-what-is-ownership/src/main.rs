fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{}", s1);

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    takes_ownership(s);

    let x = 5;
    
    makes_copy(x);

    let s1 = gives_ownership();

    let s2 = String::from("Hello");

    let s3 = takes_and_gives_ownership(s2);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello");

    some_string
}

fn takes_and_gives_ownership(a_string: String) -> String {
    a_string
}
