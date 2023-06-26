use puppeteer::Puppeteer;

fn main() {
    Puppeteer::new("Simple App")
        .unwrap()
        .set_init(init)
        //.set_title_bar_type(puppeteer::TitleBarType::Native)
        .run()
        .unwrap();
}

#[derive(Debug)]
enum CustomEvent {
    CloseWindow,
    NextPage,
    AdjustWindow,
    Onload,
}

impl From<String> for CustomEvent {
    fn from(value: String) -> Self {
        match value.as_str() {
            "close" => CustomEvent::CloseWindow,
            "next_page" => CustomEvent::NextPage,
            "maximize" => CustomEvent::AdjustWindow,
            "onload" => CustomEvent::Onload,
            _ => CustomEvent::CloseWindow,
        }
    }
}

pub fn init() -> bool {
    println!("RUNNING INIT");

    std::thread::sleep(std::time::Duration::from_secs(4));

    true
}
