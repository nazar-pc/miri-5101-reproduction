#![expect(incomplete_features, reason = "generic_const_*")]
#![feature(generic_const_args, min_generic_const_args)]

pub fn no_op<T>(value: T) -> T {
    value
}
