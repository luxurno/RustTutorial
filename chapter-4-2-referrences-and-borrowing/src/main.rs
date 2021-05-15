fn main() {
    let mut s1 = String::from("Hello");

    let len = calculate_length(&s1);

    println!("The length of {}, is {}", s1, len);

    change(&mut s1);

    let r1 = &s1; // no problem
    let r2 = &s1; // no problem 
    
    println!("{}, {}", r1, r2);
    // r1, r2 are not longer used, so you might now use muttable s1
    let r3 = &mut s1; // BIG problem, data races

    println!("{}", r3);

    let reference_do_nothing = no_dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn no_dangle() -> String {
    let s = String::from("Hello");

    s
}
