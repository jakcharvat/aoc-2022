pub mod bounds;
pub mod coord;
pub mod parsers;

#[macro_export]
macro_rules! vec2d {
    ( $el:expr; $v:expr ) => {
        vec![vec![$el; $v[0].len()]; $v.len()]
    };
}
