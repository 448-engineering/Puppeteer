use puppeteer::Puppeteer;

fn main() {
    Puppeteer::new("Simple App")
        .unwrap()
        //.set_title_bar_type(puppeteer::TitleBarType::Native)
        .run(init)
        .unwrap();
}

pub fn init() -> bool {
    println!("RUNNING INIT");

    std::thread::sleep(std::time::Duration::from_secs(4));

    true
}
