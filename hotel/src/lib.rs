mod front_of_house;

use crate::front_of_house::{hosting,turr,ty};

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    turr();
    let r = ty{d:12};
}