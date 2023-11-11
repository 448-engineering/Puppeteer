use file_format::FileFormat;
use puppeteer::{path_from_manifest, AssetFileLoader, AssetProperties, StaticAssetProperties};

fn main() {
    smol::block_on(async {
        let resource = AssetFileLoader::new("centauri.woff2")
            .add_dir("examples/assets/fonts")
            .load()
            .await
            .unwrap();

        assert_eq!(FileFormat::WebOpenFontFormat2, resource.format());
        assert_eq!(file_format::Kind::Font, resource.format().kind());
        assert_eq!("font/woff2", resource.format().media_type());
        assert_eq!(Some("WOFF2"), resource.format().short_name());
        assert_eq!("centauri.woff2", &resource.name());

        let resource = AssetFileLoader::new("frow.min.css")
            .add_dir("examples/assets")
            .load()
            .await
            .unwrap();

        assert_eq!(FileFormat::PlainText, resource.format());
        assert_eq!(file_format::Kind::Text, resource.format().kind());
        assert_eq!("text/plain", resource.format().media_type());
        assert_eq!(Some("TXT"), resource.format().short_name());
        assert_eq!("frow.min.css", resource.name());

        let frowcss = puppeteer::asset!("frow.min", "examples/assets/frow.min.css");

        assert_eq!("frow.min", frowcss.name());
        assert_eq!("text/plain", frowcss.format().media_type());

        let _path = path_from_manifest!("examples/assets", "frow.min.css");

        let counter = puppeteer::items_counter!("foos", "two");
        assert_eq!(counter, 2usize);
        let counter = puppeteer::items_counter!(1, 2, 3);
        assert_eq!(counter, 3usize);

        assert_eq!(
            "foo/bar/baz/foo.txt",
            &puppeteer::concat_paths!("foo", "bar", "baz", "foo.txt")
        );

        let assets = puppeteer::load_assets!(
            ("frow.min", "assets/frow.min.css"),
            ("centauri", "assets/fonts/centauri.woff2"),
            ("rockville_solid", "assets/fonts/rockville_solid.woff2"),
        );
        assert_eq!(
            "0b3ae879a79a09c1aa75f82b8f4a2482f08842b511b4b075484996e29cd7c3b0",
            blake3::hash(assets[0].bytes).to_hex().as_str()
        );
        assert_eq!(
            "b67a70f6e41e9dc758a1ab6b24d30865df90446fb155d6fb905c5789b3f43ce3",
            blake3::hash(assets[1].bytes).to_hex().as_str()
        );
        assert_eq!(
            "53a3c3ce4bdb8062c464f624a72a8c7589cc04c612ffbfbf3b07e36e45249104",
            blake3::hash(assets[2].bytes).to_hex().as_str()
        );

        assert_eq!(
            format!("{}/foo/bar/baz/foo.txt", env!("CARGO_MANIFEST_DIR")).as_str(),
            &puppeteer::manifest_paths!("foo", "bar", "baz", "foo.txt")
        );
    })
}
