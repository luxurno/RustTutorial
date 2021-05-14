fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other then zero");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is disivible by 2");
    } else {
        println!("number is not disivible by 4, 3, 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    
    println!("The value of number is: {}", number);

    // Loop example
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // while example
    let mut number = 3;

    while number != 0 {
        println!("{}", number);
        number -= 1;


    }

    println!("LIFTOFF");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for element in a.iter() {
        println!("For loop: the value is: {}", element);
    }
}
