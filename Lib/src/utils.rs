/// Concatenate several static strings at compile time
#[macro_export]
macro_rules! concat_strs {
    ( $( $str:expr ),* ) => {
        {
            const ARRAY_LEN: usize = count_args![$($str),*];
            let mut outcome = arrayvec::ArrayString::<ARRAY_LEN>::new();

            $( {
                outcome.push_str($str)
            })*

            outcome
        }
    };
}

/// Count the number of values inside a declarative macro
#[macro_export]
macro_rules! count_args {
    ($($arg:expr),*) => {{
        count_args!(@count $($arg),*)
    }};
    (@count $($arg:expr),*) => {
        <[()]>::len(&[$(count_args![@single $arg]),*])
    };
    (@single $_arg:expr) => { () };
}
