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
