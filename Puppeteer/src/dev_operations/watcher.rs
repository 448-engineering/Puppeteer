use camino::{Utf8Path, Utf8PathBuf};
use notify::{event::AccessMode, Event, EventKind, RecursiveMode, Result as NotifyResult, Watcher};
use smol::channel::bounded;

pub(crate) async fn watcher() {
    let read_manifest = crate::read_manifest_file().await.unwrap();
    let mut wasm32_dir_path = Utf8PathBuf::new();
    wasm32_dir_path.push(crate::WASM32_DIR);
    wasm32_dir_path.push(read_manifest.to_string().as_str());
    wasm32_dir_path.set_extension("wasm");

    let (sender, receiver) = bounded::<()>(100);

    let mut watcher = notify::recommended_watcher(move |res: NotifyResult<Event>| match res {
        Ok(event) => match event.kind {
            EventKind::Access(access_event) => {
                if access_event == notify::event::AccessKind::Close(AccessMode::Write) {
                    let modified_paths = if event.paths.len() > 1 {
                        event
                            .paths
                            .iter()
                            .map(|path| path.to_string_lossy() + "\n" + super::SPACING)
                            .collect::<String>()
                    } else {
                        event.paths[0].to_string_lossy().to_string()
                    };

                    let logger = super::Logger::new(&modified_paths).with_label(" PATHS> ");
                    println!("{}{}{}", logger.symbol, logger.label.unwrap(), logger.body,);
                    //Unwrap here never fails

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

    let paths = vec!["./src", "./Cargo.toml"];

    paths.iter().for_each(|path| {
        watcher
            .watch(Utf8Path::new(path).as_std_path(), RecursiveMode::Recursive)
            .unwrap();
    });

    super::exec(&wasm32_dir_path, receiver).await.unwrap();
}
