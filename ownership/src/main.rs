fn main() {
    let s1 = String::from("hello");
    // println!("{}", s1.len()); somehow s in
    let length = pass_reference(&s1);

    println!("\"{}\" string's length is : {}", s1, length);

    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        // let r2 = &mut s; Won't compile because we can't have two mutable reference to same value in a single scope
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

}

fn pass_reference(s: &String) -> usize {
    return s.len();
}

fn _first_ownership_move() {
    let s = String::from("hello");

    _take_ownership(s); // ownership of s is moved to this function!
    // try_to_take_ownership_again(s); -> Compiler will error out.
    let m : i32 = 5;
    _move_m(m);
}

fn _try_to_take_ownership_again(some_string: String){
    println!("{}", some_string);
}

fn _take_ownership(some_string: String){
    println!("{}", some_string);
}

fn _move_m(some_integer: i32){
    println!("{}", some_integer)
}

fn _string_literal() {
    let mut s = String::from("hello");
    // above s is different from string literal below
    let x = "Something, immutable!";
    s.push_str(", world"); // push_str() appends a literal to a String
    println!("{s}");
    println!("{x}");
}
