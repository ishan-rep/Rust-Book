use std::io;

fn arrays(){
    println!("Enter the index of which value you want: ");

    let a: [u32;10] = [1,2,3,4,5,6,7,8,9,10];

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line!");
    
    let index: usize = index
                .trim()
                .parse()
                .expect("Couldn't parse it into int");
    
    let element = a[index];
    println!("The value of Array a at index {index} is {element}");
}
