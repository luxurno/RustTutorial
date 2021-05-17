fn main() {
    // Creating a New String
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // method also works directly
    let s = "initial contents".to_string();

    let s = String::from("Initial contents");
    
    // Copied valid UTF-8 strings
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toc");

    // Some error on s1
    // let s = s1 + "-" + &s2 + "-" + &s3;

    let s = format!("{}-{}-{}", s1, s2, s3);

    let s1 = String::from("hello");
    // Error
    // let h = s1[0];

    let hello = "Здравствуйте";
    let answer = &hello[0..4];
    
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
