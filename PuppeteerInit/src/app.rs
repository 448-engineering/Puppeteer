use crate::{events::UserEvent, screens::splash_screen, Logger, SHELL_DEBUG_RUN_COMMAND};
use once_cell::sync::OnceCell;
use puppeteer_types::{
    blake3,
    wry::{
        application::{
            event::{Event, StartCause, WindowEvent},
            event_loop::{ControlFlow, EventLoop, EventLoopProxy},
            window::WindowBuilder,
        },
        webview::WebViewBuilder,
    },
    Puppeteer,
};
use smol::{
    io::{AsyncBufReadExt, BufReader},
    process::{Command, Stdio},
    stream::StreamExt,
};
use std::borrow::Cow;
use yansi::Paint;

#[cfg(debug_assertions)]
pub(crate) static CURRENT_HASH: OnceCell<blake3::Hash> = OnceCell::new();
pub(crate) static EVENT_PROXY: OnceCell<EventLoopProxy<UserEvent>> = OnceCell::new();

pub async fn runner() {
    let event_loop = EventLoop::<UserEvent>::with_user_event();
    let window = WindowBuilder::new()
        .with_title("WebAssembly App")
        .build(&event_loop)
        .unwrap();

    let webview = WebViewBuilder::new(window)
        .unwrap()
        .with_html("<body>'FOO'</body>".to_owned())
        .unwrap()
        .build()
        .unwrap();

    let proxy = event_loop.create_proxy();

    EVENT_PROXY.get_or_init(|| proxy);

    let mut current_hash = blake3::hash(&[]);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                println!("Application started");
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("Closing window");
                *control_flow = ControlFlow::Exit;
            }
            Event::UserEvent(ui_data) => {
                let blake3hash = blake3::hash(&ui_data);

                if current_hash != blake3hash {
                    println!("RECEIVED» {}", blake3hash);
                    let eval = wasm_loader(&ui_data);
                    //let eval = r#"console.log("YEAH");"#;
                    webview.evaluate_script(&eval).unwrap();

                    current_hash = blake3hash;
                } else {
                    println!("IGNORED» {}", blake3hash);
                }
            }
            _ => (),
        }
    });
}

fn wasm_loader<'a>(wasm_bytes: &[u8]) -> Cow<'a, str> {
    let part_1 = r#"
            function injector(){
                let wasm = new Uint8Array(
    
    "#;

    let part2 = "
            );
            WebAssembly.instantiate(wasm).then(wasm_object => {
                //console.log(wasm_object.instance.exports.greet());

                const memory = wasm_object.instance.exports.memory;
                const greet = wasm_object.instance.exports.greet;
                const deallocateString = wasm_object.instance.exports.deallocate_string;

                const messagePtr = greet();
                const messageView = new Uint8Array(memory.buffer);
                let i = messagePtr;
                let message = '';
                while (messageView[i] !== 0) {
                    message += String.fromCharCode(messageView[i]);
                    i++;
                }

                deallocateString(messagePtr);
                console.log(message);
                document.body.innerHTML = message;
            });
            };

            injector();
    ";

    let full = format!("{}{:?}{}", part_1, wasm_bytes, part2);

    Cow::Owned(full)
}

pub(crate) async fn run_shell() {
    let exec_command = SHELL_DEBUG_RUN_COMMAND
        .iter()
        .map(|command| command.to_string() + " ")
        .collect::<String>();

    let logger = Logger::new(&exec_command)
        .symbol("SHELL>")
        .with_label(" COMPILING...");
    println!("{}{}{}", logger.symbol, logger.label.unwrap(), logger.body);

    let mut build_wasm = Command::new(SHELL_DEBUG_RUN_COMMAND[0])
        .args(&SHELL_DEBUG_RUN_COMMAND[1..])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let stdout = build_wasm.stdout.take().unwrap();
    let reader = BufReader::new(stdout);

    let mut lines = reader.lines();
    while let Some(line) = lines.next().await {
        println!("{}", line.unwrap()); //TODO
    }

    let status = build_wasm.output().await;

    let output = match status {
        Ok(value) => value,
        Err(error) => {
            println!(
                "{}\n{}",
                Paint::red("ENCOUNTERED ERROR:"),
                Paint::red(error.to_string())
            );

            std::process::exit(1);
        }
    };

    println!("{}", String::from_utf8(output.stdout).unwrap());
}

//#[cfg(not(debug_assertions))]

//#[cfg(debug_assertions)]

pub fn shell() {
    let window = WindowBuilder::new()
        .with_decorations(false)
        .with_title("Puppeteer");
    Puppeteer::<UserEvent>::new(Box::new(splash_screen()))
        .unwrap()
        .set_window(window)
        .unwrap()
        .run(crate::custom_event_handler)
        .unwrap();

    /*************************** */
    let mut buffer = Vec::new();

    let event_loop = EventLoop::<UserEvent>::with_user_event();
    let window = WindowBuilder::new()
        .with_title("WebAssembly App")
        .build(&event_loop)
        .unwrap();

    let webview = WebViewBuilder::new(window)
        .unwrap()
        .with_html("<body>'FOO'</body>".to_owned())
        .unwrap()
        .build()
        .unwrap();

    let proxy = event_loop.create_proxy();

    EVENT_PROXY.get_or_init(|| proxy);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                println!("Application started");
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("Closing window");
                *control_flow = ControlFlow::Exit;
            }
            Event::UserEvent(ui_data) => {
                let blake3hash = blake3::hash(&ui_data);

                if CURRENT_HASH.get() != Some(&blake3hash) {
                    println!("RECEIVED» {}", blake3hash);
                    let eval = wasm_loader(&ui_data);
                    //let eval = r#"console.log("YEAH");"#;
                    webview.evaluate_script(&eval).unwrap();

                    CURRENT_HASH.get_or_init(|| blake3hash);
                } else {
                    println!("IGNORED» {}", blake3hash);
                }
            }
            _ => (),
        }
    });
}
