fn main() {
    let v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];

    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Reading elements of vectors
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50
    }

    let row = vec![
        SpeadsheetCell::Int(3),
        SpeadsheetCell::Text(String::from("blue")),
        SpeadsheetCell::Float(10.12),
    ];
}

enum SpeadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

