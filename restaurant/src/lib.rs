#![allow(dead_code)] //Using this to prevent warnings about dead code:
//EG all of it since nothing is actually used
#![allow(unused_variables)] //Using this to prevent warnings about unused variables
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}

        pub fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    //impl for Breakfast is important as, since it has a private field, it would be impossible to use it
    //with out a function to set all the fields. So since we can set toast our self and there is a way
    //for the function to set the fruit, it's usable! Can't see the fruit though, it's just there
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    //Enums are different than structs in that if the enum is public, all the variants are public
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {}
}

//items are private by default, you need to add pub to allow something to use it
//Just because a parent is public, does not mean its child is public

//Use using (ha) absolute path
use crate::front_of_house::hosting;

//Relative path goes:
//self::front_of_house::hosting;

//For clarity purposes, when using functions, it's better to use the parent and reference it that way
//For structs and enums, it's okay to go a step further. The point is to ensure scope is obvious!
//The one exception is that if it would cause there to be name conflicts, import/use the parent to reference it
//OR
//use the as keyword to rename it and prevent a naming conflict
//EG:
//use std::fmt::Result;
//use std::io::Result as IoResult;

pub fn eat_at_restaurant() {
    //absolute path
    //crate::front_of_house::hosting::add_to_waitlist();
    //relative path
    //front_of_house::hosting::add_to_waitlist();

    //Utilizing use instead of relative/absolute path
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    //Ordering a breakfast with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    //Chaning our mind about the bread
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast, please", meal.toast);
    //However we can't do this for the fruit since it is private

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
