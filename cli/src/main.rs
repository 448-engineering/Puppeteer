use camino::Utf8Path;
use notify::{event::AccessMode, Event, EventKind, RecursiveMode, Result as NotifyResult, Watcher};
use smol::{
    channel::bounded,
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    process::{Command, Stdio},
};

mod app;
pub(crate) use app::*;
mod args;
pub use args::*;
mod constants;
pub use constants::*;
mod exec_path;
pub(crate) use exec_path::*;
mod manifest;
pub(crate) use manifest::*;
mod errors;
pub(crate) use errors::*;
mod utils;
pub use utils::*;
mod events;
pub(crate) use events::*;
mod screens;
pub(crate) use screens::*;

fn main() {
    smol::block_on(async {
        start_shell();

        let path = get_path().await.unwrap();
        let (sender, receiver) = bounded(100);

        let mut watcher = notify::recommended_watcher(move |res: NotifyResult<Event>| match res {
            Ok(event) => match event.kind {
                EventKind::Access(access_event) => {
                    if access_event == notify::event::AccessKind::Close(AccessMode::Write) {
                        let modified_paths = if event.paths.len() > 1 {
                            event
                                .paths
                                .iter()
                                .map(|path| path.to_string_lossy() + "\n" + SPACING)
                                .collect::<String>()
                        } else {
                            event.paths[0].to_string_lossy().to_string()
                        };

                        let logger = Logger::new(&modified_paths).with_label(" PATHS> ");
                        println!("{}{}{}", logger.symbol, logger.label.unwrap(), logger.body,); //Unwrap here never fails

                        smol::block_on(async { sender.send(()).await.unwrap() })
                    } else {
                        ()
                    }
                }
                _ => (),
            },
            Err(e) => println!("watch error: {:?}", e),
        })
        .unwrap();

        let paths = vec!["./src", "Cargo.toml"];

        paths.iter().for_each(|path| {
            watcher
                .watch(Utf8Path::new(path).as_std_path(), RecursiveMode::Recursive)
                .unwrap();
        });

        exec(&path, receiver).await.unwrap();
    })
}
