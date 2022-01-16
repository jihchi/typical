#![allow(
    clippy::cast_possible_truncation,
    clippy::identity_op,
    clippy::let_and_return,
    clippy::let_unit_value,
    clippy::match_same_arms,
    clippy::match_single_binding,
    clippy::module_inception,
    clippy::module_name_repetitions,
    clippy::no_effect,
    clippy::no_effect_underscore_binding,
    clippy::redundant_closure_for_method_calls,
    clippy::shadow_unrelated,
    clippy::similar_names,
    clippy::too_many_lines,
    clippy::type_complexity,
    clippy::unit_arg,
    clippy::unnecessary_wraps,
    clippy::unused_self,
    clippy::used_underscore_binding,
    clippy::useless_conversion,
    dead_code,
    unreachable_code,
    unreachable_patterns,
    unused_parens,
    unused_variables
)]

include!(concat!(env!("OUT_DIR"), "/types.rs"));
