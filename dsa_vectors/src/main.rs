fn main() {
    // // defining vector
    // let v : Vec<u32> = vec![1,2,3,4,5];

    // // getting an element as type Option
    // match v.get(0) {
    //     None => println!("Vector is empty"),
    //     Some(zero) => println!("First value of vector is: {}", zero)
    // }
    // // get value from reference:
    // println!("First value of vector is: {}", &v[0]);

    // // iterating over vector
    // for i in &v {
    //     println!("{i}");
    // }

    // // Defining mutable vector
    // let mut mutable_vector : Vec<u32> = Vec::new();
    // mutable_vector.push(10);
    // mutable_vector.push(9);
    // mutable_vector.push(8);
    // for i in &mut mutable_vector {
    //     println!("{i}");
    // }

    // add_fifty(&mut mutable_vector);
    // for i in &mut mutable_vector {
    //     println!("{i}");
    // }
    let mut s = String::new();
    s = "to_string_function".to_string();
    println!("{s}");

    s = String::from("String_from_function");
    println!("{s}");
    
    // Pushing values into a string
    s.push_str("_pushing_xyz");

    println!("{s}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // Using format to concatenate strings.
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");
    println!("{s1}");

    let s4 = s1 + "-" + &s2 + "-" + &s3;
    println!("{s4}");
    // can't use s1 here since the ownership got transferred to s4 above.
    println!("{s2}");
    let hello = "Здравствуйте";
    
    // This makes the code panic!
    // let s = &hello[0..3];
    // println!("{s}");

    

}

fn add_fifty(v : &mut Vec<u32>) {
    for i in v {
        // Have to add the derefencing operator here.
        *i += 50;
    }
}
