mod front_of_house;

mod back_of_the_house {
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
    }
}

use crate::front_of_house::hosting::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        // Absolute path
        // crate::front_of_the_house::hosting::add_to_waitlist();

        // Relative path
        // front_of_the_house::hosting::add_to_waitlist();

        // Order a breakfast in the summer with Rye toast
        // let mut meal = back_of_the_house::Breakfast::summer("Rye");
        // meal.toast = String::from("Wheat");
        // println!("I'd like {} toast please", meal.toast);
        super::hosting::add_to_waitlist();
    }
}
