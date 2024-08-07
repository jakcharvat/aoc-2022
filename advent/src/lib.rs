pub mod bounds;
pub mod coord;
pub mod diagonal_iterable;
pub mod parsers;
pub mod side_effect;

#[macro_export]
macro_rules! vec2d {
    ( $el:expr; $v:expr ) => {
        vec![vec![$el; $v[0].len()]; $v.len()]
    };
}
