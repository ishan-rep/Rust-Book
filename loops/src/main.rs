fn main() {
    // basic_loop();
    // label_loops();
    fibonacci();
}

fn _basic_loop() {
    let mut counter = 0;
    
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }

    };
    println!("Value of result is {result}");
}

fn fibonacci() {
    let mut curr = 1;
    let mut next = 1;
    let mut temp : i32;
    let mut num = 1;
    while num < 20 {
        println!("{num} Fibonacci number is : {curr}");
        temp = curr + next;
        curr = next;
        next = temp;
        num += 1;
    }
}

fn _label_loops() {
    let mut count = 0;
    // Can add labels like this and refer these while calling break / continue
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
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
    // conditional loops!
    while count != 3 {
        println!("End count = {count}");
        count += 1
    }

    let a : [i32; 5] = [1,2,3,4,5];
    for element in a {
        println!("The value is : {element}");
    }
    
}
