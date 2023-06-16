#[cfg(feature = "dev")]
mod dev_operations;
#[cfg(feature = "dev")]
pub use dev_operations::*;

fn main() {
    smol::block_on(async {
        #[cfg(feature = "dev")]
        crate::watcher().await;
        #[cfg(not(feature = "dev"))]
        println!("Hello World");
    })
}
