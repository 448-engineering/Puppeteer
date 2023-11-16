/// Count the number of items inside a macro
#[macro_export]
macro_rules! items_counter {
    ($($name:expr),* $(,)?) => {
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

/// Include asset bytes at compile time from an absolute path
#[macro_export]
macro_rules! assets {
    ($(($name:expr, $path:expr)),* $(,)?) => {{
        const ITEMS_LEN: usize = [$(stringify!($name),)*].len();

        use puppeteer::StaticAsset;

        let mut outcome = [StaticAsset{name: "", bytes: &[0u8]}; ITEMS_LEN];
        let mut count = 0usize;

        $({
            const BYTES: &[u8] = include_bytes!($path);
            const ASSET: StaticAsset = StaticAsset{
                name: $name,
                bytes: BYTES
            };
            outcome[count] = ASSET;
            count += 1;
        })*;

        outcome
    }};
}

/// Include asset bytes at compile time but load the path from the manifest directory
#[macro_export]
macro_rules! assets_from_manifest_dir {
    ($(($name:expr, $path:expr)),* $(,)?) => {{
        const ITEMS_LEN: usize = [$(stringify!($name),)*].len();

        use puppeteer::StaticAsset;

        let mut outcome = [StaticAsset{name: "", bytes: &[0u8]}; ITEMS_LEN];
        let mut count = 0usize;

        $({
            const BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path));
            const ASSET: StaticAsset = StaticAsset{
                name: $name,
                bytes: BYTES
            };
            outcome[count] = ASSET;
            count += 1;
        })*;

        outcome
    }};
}

/// Include asset in UTF-8 format at compile time
#[macro_export]
macro_rules! static_str {
    ($path:expr) => {{
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path))
    }};
}

/// Load static strings
#[macro_export]
macro_rules! load_strs {
    ($($path:expr),* $(,)?) => {{
        const ITEMS_LEN: usize = [$(stringify!($path),)*].len();

        use puppeteer::StaticAsset;

        let mut outcome = [""; ITEMS_LEN];
        let mut count = 0usize;

        $({
            outcome[count] = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path));
            count += 1;
        })*;

        outcome
    }};
}

/// Load strings within a Clone-on-Write with a `'static` lifetime styles
#[macro_export]
macro_rules! load_cow {
    ($($path:expr),* $(,)?) => {{
        const ITEMS_LEN: usize = [$(stringify!($path),)*].len();

        use puppeteer::{StaticAsset, StaticCowStr};

        let mut outcome = [StaticCowStr::Borrowed(""); ITEMS_LEN];
        let mut count = 0usize;

        $({
            outcome[count] = StaticCowStr::Borrowed(
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path))
            );
            count += 1;
        })*;

        outcome
    }};
}
