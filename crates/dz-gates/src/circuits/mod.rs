pub mod adder;
pub mod comparator;
pub mod complement;
pub mod multiplier;
pub mod parity;

pub use adder::{full_adder, half_adder, ripple_add};
pub use comparator::{flag_to_bool, ge};
pub use complement::{ones_complement, ones_complement_digit, radix_complement};
pub use multiplier::{digit_mul, multiply, scale_by_digit};
pub use parity::is_even_digit;
