/// Count the number of items inside a macro
#[macro_export]
macro_rules! items_counter {
    ($name:ident) => { 1 };
    ($first:ident, $($rest:ident),*) => {
        1 + count_items!($($rest),*)
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
