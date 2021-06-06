#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

fn serve_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
        pub fn show_menu(&self) {
            println!("{} with {}", self.toast, self.seasonal_fruit);
        }
    }
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
    fn cook_order() {}
}

use crate::front_of_house::hosting;
// use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_wait_list();

    // relative path
    front_of_house::hosting::add_to_wait_list();

    // use hosting mod
    hosting::add_to_wait_list();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.show_menu();
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // println!("I cannot know about fruit: {}", meal.seasonal_fruit);
}
