fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Condition-based number
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    // named loop breaking when count equals 2
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        
        count += 1;
    }
    println!("End count = {}", count);

    // Loop with result at stop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    //Basic while loop
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // Index based while loop (slow and can cause panics at runtime)
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // More efficient for loop
    for element in a {
        println!("the value is: {}", element);
    }

    // For loop with rev, to reverse the range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    println!("5 Fahrenheit is {} degrees celsius", fahr_to_cels(5.0));

    for number in 1..10 {
        println!("The {}th fibonacci number is: {}", number, fib(number));
    }
    twelve_days_of_christmas(12);
}


fn fahr_to_cels(f: f32) -> f32 {
    (f-32.0) * 0.5556 
}

fn fib(n: u32) -> u32 {
    match n {
        1 => 0,
        2 => 1,
        _ => {
            fib(n-1) + fib(n-2)
        },
    }
}

fn twelve_days_of_christmas(n: usize) -> () {
    let days = ["A song and a Christmas tree",
                            "Two candy canes",
                                "Three boughs of holly",
                                "Four colored lights",
                                "A shining star",
                                "Little silver bells",
                                "Candles a-glowing",
                                "Gold and silver tinsel",
                                "A guardian angel",
                                "Some mistletoe",
                                "Gifts for one and all",
                                "All their good wishes"];
    let numbers = ["First","Second","Third","Fourth","Fifth","Sixth","Seventh","Eigth","Ninth","Tenth","Eleventh","Twelfth"];

    for number in 0..n {
        println!("On the {} day of christmas \nMy good friends brought to me", numbers[number]);
        for verse in (0..=(number)).rev() {
            println!("{}",days[verse]);
        }
        println!("\n");
    }
    
}
