use file_format::FileFormat;
use puppeteer::{AssetFileLoader, AssetProperties, StaticAssetProperties};

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

        let counter = puppeteer::items_counter!(
            ("frow.min", "assets/frow.min.css"),
            ("centauri", "assets/fonts/centauri.woff2"),
            ("warteg", "assets/fonts/warteg.woff2"),
        );
        assert_eq!(counter, 3usize);
        let counter = puppeteer::items_counter!(1, 2, 3);
        assert_eq!(counter, 3usize);

        let assets = puppeteer::assets!(
            ("frow.min", "assets/frow.min.css"),
            ("centauri", "assets/fonts/centauri.woff2"),
            ("warteg", "assets/fonts/warteg.woff2"),
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
            "1ee55e0400b17f43438b4ca12a94fa83984323095fa6093e9e97ca6b13d906e6",
            blake3::hash(assets[2].bytes).to_hex().as_str()
        );

        let assets = puppeteer::assets_from_manifest_dir!(
            ("frow.min", "examples/assets/frow.min.css"),
            ("centauri", "examples/assets/fonts/centauri.woff2"),
            ("warteg", "examples/assets/fonts/warteg.woff2"),
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
            "1ee55e0400b17f43438b4ca12a94fa83984323095fa6093e9e97ca6b13d906e6",
            blake3::hash(assets[2].bytes).to_hex().as_str()
        );
    })
}
