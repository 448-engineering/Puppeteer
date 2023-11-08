use puppeteer::{path_from_manifest, AssetFileLoader, AssetProperties, StaticAssetProperties};

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

        let frowcss = puppeteer::asset!("frow.min", "examples/assets/frow.min.css");

        dbg!(frowcss.name());
        dbg!(frowcss.format().media_type());

        let _path = path_from_manifest!("examples/assets", "frow.min.css");

        let counter = puppeteer::items_counter!("foos", "two");
        assert_eq!(counter, 2usize);
        let counter = puppeteer::items_counter!(1, 2, 3);
        assert_eq!(counter, 3usize);

        dbg!(puppeteer::concat_paths!("foo", "bar", "baz", "foo.txt"));

        let assets = puppeteer::load_assets!(("frow.min", "assets/frow.min.css"));
        assert_eq!(
            "0b3ae879a79a09c1aa75f82b8f4a2482f08842b511b4b075484996e29cd7c3b0",
            blake3::hash(assets[0].bytes).to_hex().as_str()
        );

        dbg!(puppeteer::manifest_paths!("foo", "bar", "baz", "foo.txt"));
    })
}
