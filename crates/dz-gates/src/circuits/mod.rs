pub mod adder;
pub mod comparator;
pub mod complement;
pub mod depth;
pub mod multiplier;
pub mod parity;
pub mod trace;

pub use adder::{
    full_adder, full_adder_net, generate_propagate, generate_propagate_balanced, half_adder,
    half_adder_balanced, half_adder_balanced_depth, half_adder_depth, half_adder_net,
    lookahead_add, lookahead_add_balanced, lookahead_add_balanced_depth, lookahead_add_depth,
    ripple_add, ripple_add_depth, ripple_add_net, ripple_add_net_traced,
};
pub use comparator::{flag_to_bool, ge, ge_net};
pub use complement::{
    ones_complement, ones_complement_digit, ones_complement_digit_net, ones_complement_net,
    radix_complement, radix_complement_net,
};
pub use depth::{max_level, depth_tracking, record_level, DepthScope, GateNet, Levels, Sig, Values};
pub use multiplier::{
    digit_mul, digit_mul_net, digit_mul_v1, digit_mul_v1_net, digit_mul_v2, digit_mul_v2_net,
    multiply, multiply_net, multiply_net_traced, scale_by_digit, scale_by_digit_net, MulTraceSink,
    NoMulTrace,
};
pub use parity::{is_even_digit, is_even_digit_net};
pub use trace::{multiply_trace, ripple_add_trace, MulTrace, PartialRow, StepRecord};
