use puppeteer::{AssetFileLoader, FileProperties};

fn main() {
    smol::block_on(async {
        let resource = AssetFileLoader::new("centauri.woff2")
            .add_dir("examples/assets/fonts")
            .load()
            .await
            .unwrap();

        dbg!(&resource.format());
        dbg!(&resource.format().kind());
        dbg!(&resource.format().media_type());
        dbg!(&resource.format().short_name());
        dbg!(&resource.name());
        dbg!(&resource.base64());

        let resource = AssetFileLoader::new("frow.min.css")
            .add_dir("examples/assets")
            .load()
            .await
            .unwrap();

        dbg!(&resource.format());
        dbg!(&resource.format().kind());
        dbg!(&resource.format().media_type());
        dbg!(&resource.format().short_name());
        dbg!(&resource.name());
        dbg!(&resource.base64());
    })
}
