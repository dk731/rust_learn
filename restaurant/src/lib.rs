use back_of_house::Breakfast;

mod front_of_house;

mod back_of_house;

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");

    println!("I want different toast: {}", meal.toast);
}
