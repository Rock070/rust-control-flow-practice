mod homework {
    pub mod hm1;
    pub mod hm2;
}

use homework::hm1;
use homework::hm2;

fn main() {
    hm1::temperature_transfer();
    hm2::fibonacci();
}
