/// Specifies the default path to look for the `wasm32-unknown-unknown` binary
pub const WASM32_DIR: &str = "./target/wasm32-unknown-unknown/debug/";
/// Specifies the default buffer capacity
pub const BUFFER_CAPACITY: usize = 64 * 1024;
/// The spacing fot logging information
pub const SPACING: &str = "     ";
/// The default cargo command executed after watched file(s) have been updated
pub const SHELL_DEBUG_RUN_COMMAND: [&str; 2] = ["cargo", "run"];
/// The default cargo command executed after watched file(s) have been updated
pub const DEFAULT_BUILD_COMMAND: [&str; 4] =
    ["cargo", "build", "--target", "wasm32-unknown-unknown"];
