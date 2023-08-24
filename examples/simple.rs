use puppeteer::{AppEnvironment, InvokeWebView, OsFamily, OsType, Puppeteer};

fn main() {
    {
        // Detect App OS
        if cfg!(target_os = "linux") {
            assert_eq!(OsType::Linux, OsType::which_os().into());
        } else if cfg!(target_os = "windows") {
            assert_eq!(OsType::Windows, OsType::which_os().into());
        } else if cfg!(target_os = "macos") {
            assert_eq!(OsType::MacOs, OsType::which_os().into());
        } else if cfg!(target_os = "silly_os") {
            assert_eq!(OsType::UnrecognizedOs, OsType::which_os().into());
        }
    }

    {
        // Detect app environment by initializing the `AppEnvironment` struct with `init()` method
        let init_app_env = AppEnvironment::init();

        if cfg!(target_os = "linux") {
            assert_eq!(init_app_env.os, OsType::which_os().into());
        } else if cfg!(target_os = "windows") {
            assert_eq!(init_app_env.os, OsType::which_os().into());
        } else if cfg!(target_os = "macos") {
            assert_eq!(init_app_env.os, OsType::which_os().into());
        } else {
        }

        if cfg!(target_arch = "unix") {
            assert_eq!(init_app_env.family, OsFamily::which_os_family().into());
        } else if cfg!(target_family = "windows") {
            assert_eq!(init_app_env.family, OsFamily::which_os_family().into());
        }
    }
}
