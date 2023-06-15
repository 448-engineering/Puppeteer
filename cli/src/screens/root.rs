use puppeteer_types::{Container, DefaultSplashScreen, UiPaint};

pub(crate) fn splash_screen() -> impl UiPaint {
    let mut container = Container::new();

    // Compiled in debug mode where WebAssembly is used for rapid deployment.
    #[cfg(debug_assertions)]
    container.add_child(Box::new(DefaultSplashScreen::dark()));

    // Compiled in release mode
    #[cfg(not(debug_assertions))]
    container.add_child(Box::new(DefaultSplashScreen::dark()));

    container
}
