pub mod adder;
pub mod comparator;
pub mod complement;
pub mod depth;
pub mod multiplier;
pub mod parity;

pub use adder::{
    full_adder, generate_propagate, half_adder, half_adder_depth, lookahead_add,
    lookahead_add_depth, ripple_add, ripple_add_depth,
};
pub use comparator::{flag_to_bool, ge};
pub use complement::{ones_complement, ones_complement_digit, radix_complement};
pub use depth::{max_level, GateNet, Levels, Sig, Values};
pub use multiplier::{digit_mul, digit_mul_v1, digit_mul_v2, multiply, scale_by_digit};
pub use parity::is_even_digit;
