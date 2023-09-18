fn main() {
    let mut counter = 0;
    
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }

    };
    println!("Value of result is {result}");
    label_loops();
}

fn label_loops() {
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
    
}
