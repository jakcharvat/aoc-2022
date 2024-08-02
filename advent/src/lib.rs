pub mod coord;

#[macro_export]
macro_rules! vec2d {
    ( $el:expr; $v:expr ) => {
        vec![vec![$el; $v[0].len()]; $v.len()]
    };
}
