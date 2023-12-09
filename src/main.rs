mod homework {
    pub mod hm1;
    pub mod hm2;
    pub mod hm3;
}

use homework::hm1;
use homework::hm2;
use homework::hm3;

fn main() {
    hm1::temperature_transfer();
    hm2::fibonacci();
    hm3::print_christmas();
}
