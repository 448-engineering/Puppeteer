use super::{read_wasm, Logger, DEFAULT_BUILD_COMMAND};
use blake3;
use camino::Utf8Path;

use smol::{
    channel::Receiver,
    io::{AsyncBufReadExt, BufReader, Result as smolIoResult},
    process::{Command, Stdio},
    stream::StreamExt,
};
use yansi::Paint;

pub async fn exec(path: &Utf8Path, receiver: Receiver<()>) -> smolIoResult<()> {
    let watching = Logger::new(" WATCHING FOR CHANGES.....");
    println!("{}{}{}", watching.header, watching.symbol, watching.body);

    while let Ok(_) = receiver.recv().await {
        let exec_command = DEFAULT_BUILD_COMMAND
            .iter()
            .map(|command| command.to_string() + " ")
            .collect::<String>();

        let logger = Logger::new(&exec_command).with_label(" EXECUTING> ");
        println!("{}{}{}", logger.symbol, logger.label.unwrap(), logger.body);

        let mut build_wasm = Command::new(DEFAULT_BUILD_COMMAND[0])
            .args(&DEFAULT_BUILD_COMMAND[1..])
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let stdout = build_wasm.stdout.take().unwrap();
        let reader = BufReader::new(stdout);

        let mut lines = reader.lines();
        while let Some(line) = lines.next().await {
            println!("{}", line?);
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

        if output.status.success() {
            let wasm_bytes = read_wasm(&path).await?;
            let blake3hash = blake3::hash(&wasm_bytes).to_hex().to_owned();
            let printer = Logger::new(blake3hash.as_str()).symbol("=> CHANGES>");

            println!("{}{}", printer.symbol, printer.body);
        }
    }

    Ok(())
}
