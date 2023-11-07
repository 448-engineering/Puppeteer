/// Count the number of items inside a macro
#[macro_export]
macro_rules! items_counter {
    ($($name:expr),*) => {
        {
           [$($name,)*].len()
        }
    }
}

/// Include asset bytes at compile time
#[macro_export]
macro_rules! asset {
    ($name:expr, $path:expr) => {{
        let asset_path = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path));
        puppeteer::StaticAsset {
            name: $name,
            bytes: asset_path,
        }
    }};
}

/// Include concatenate some paths.
/// **NOTE** that if quotes are added to a &str they are considered characters,
/// so use `concat_paths!(foo,bar)` instead of `concat_paths!("foo", "bar")`
/// since this would lead to `"foo"/"bar"` instead of `foo/bar`
#[macro_export]
macro_rules! concat_paths {
    ($($name:expr),*) => {{
        const ARRAY_STRING_LEN: usize = 0 $(+ stringify!($name).len() + "/".len())+ ;

        let mut outcome =  arrayvec::ArrayString::<ARRAY_STRING_LEN>::new();

        $({
            outcome.push_str(stringify!($name));
            outcome.push('/');
        })*;

        outcome.pop();

        outcome.truncate(ARRAY_STRING_LEN - 1);

        outcome

    }};
}

/// Include concatenate some paths.
/// **NOTE** that if quotes are added to a &str they are considered characters,
/// so use `concat_paths!(foo,bar)` instead of `concat_paths!("foo", "bar")`
/// since this would lead to `"foo"/"bar"` instead of `foo/bar`
#[macro_export]
macro_rules! manifest_paths {
    ($($name:expr),*) => {{
        const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
        const SEPERATOR_LEN: usize = "/".len();
        const ARRAY_STRING_LEN: usize = CARGO_MANIFEST_DIR.len() + SEPERATOR_LEN $(+ stringify!($name).len() + SEPERATOR_LEN)+ ;

        let mut outcome =  arrayvec::ArrayString::<ARRAY_STRING_LEN>::new();
        outcome.push_str(CARGO_MANIFEST_DIR);
        outcome.push('/');

        $({
            outcome.push_str(stringify!($name));
            outcome.push('/');
        })*;

        outcome.pop();

        outcome.truncate(ARRAY_STRING_LEN - 1);

        outcome

    }};
}

/// Build path from macros into a static str
#[macro_export]
macro_rules! path_from_manifest {
    ($folder:expr, $file_name:expr) => {{
        concat!(env!("CARGO_MANIFEST_DIR"), "/", $folder, "/", $file_name)
    }};
}
