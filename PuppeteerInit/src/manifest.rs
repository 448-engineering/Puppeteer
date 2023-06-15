use crate::{InjectorResult, Logger};
use camino::{Utf8Path, Utf8PathBuf};
use smol::{fs::File, io::AsyncReadExt};
use std::borrow::Cow;

pub async fn read_manifest_file<'a>() -> InjectorResult<Cow<'a, str>> {
    let mut manifest_file = File::open("./Cargo.toml").await?;
    let mut contents = String::new();
    manifest_file.read_to_string(&mut contents).await?;

    let parsed = cargo_toml::Manifest::from_str(&contents).unwrap();

    let mut package_name = String::new();

    if let Some(lib) = parsed.lib.as_ref() {
        if let Some(lib_name) = lib.name.as_ref() {
            package_name.push_str(&lib_name);
        } else {
            package_name.push_str(&parsed.package().name);
        }
    } else {
        package_name.push_str(&parsed.package().name);
    }

    let package_name = package_name.replace("-", "_");

    Ok(Cow::Owned(package_name))
}

pub async fn get_path() -> InjectorResult<Utf8PathBuf> {
    let args = crate::Args::parse();

    let package_name = read_manifest_file().await?;

    let mut path = Utf8PathBuf::new();
    path.push(args.path());
    path.push(package_name.to_string());
    path.set_extension("wasm");

    Ok(path)
}

pub async fn read_wasm<'a>(path: &Utf8Path) -> InjectorResult<Vec<u8>> {
    let logger = Logger::new(path.as_str()).with_label(" BINARY> ");
    println!("{}{}{}", logger.symbol, logger.label.unwrap(), logger.body);

    let mut file = File::open(path).await?;

    let mut contents = Vec::new();
    file.read_to_end(&mut contents).await?;

    Ok(contents)
}
