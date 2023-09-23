mod front_of_house;

pub use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        crate::front_of_house::hosting::add_to_waitlist();
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
