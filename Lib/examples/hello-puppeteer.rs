use html_to_string_macro::html;
use once_cell::sync::Lazy;
use puppeteer::{
    async_trait::{self},
    smol::lock::Mutex,
    tracing::{self, Level},
    ActiveAppEnv, ContextMenu, ModifyView, Puppeteer, PuppeteerApp, Shell, DEFAULT_WINDOW_ACTIONS,
    DEFAULT_WINDOW_ACTIONS_SCRIPT, DEFAULT_WINDOW_ACTIONS_STYLE,
};
use std::{borrow::Cow, collections::HashMap};
use tracing_subscriber::FmtSubscriber;

static MEM_STORE: Lazy<Mutex<HashMap<&str, String>>> = Lazy::new(|| {
    let storage = HashMap::new();
    Mutex::new(storage)
});

fn main() {
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    smol::block_on(async {
        PuppeteerApp::<AppTest>::init("Puppeteer Test App")
            .with_fonts_dir(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/examples/assets/fonts"
            ))
            .await
            .unwrap()
            .start()
            .await
            .unwrap()
    })
}

#[derive(Debug)]
pub enum AppTest {
    Root,
    CloseWindow,
    RecvUserEmail(String),
    SubmitEmail,
}

impl AsRef<str> for AppTest {
    fn as_ref(&self) -> &str {
        match self {
            Self::Root => "root",
            Self::CloseWindow => "close_window",
            Self::RecvUserEmail(_) => "recv_user_email",
            Self::SubmitEmail => "submit_email",
        }
    }
}

const PUPPETEER_LOGO: &str = include_str!("../../Documentation/Puppeteer-Logo.svg");
const PUPPETEER_ICON: &str = include_str!("../../Documentation/Puppeteer-Logo-Icon.svg");

const CONTEXT_MENU: &str = r#"
<div id = "context-menu-identifier" style = "display: none;">
<ul class = "menuItems">
<li class = "items">Menu Item-1 </li>
<li class = "items">Menu Item-2 </li>
<li class = "items">Menu Item-3 </li>
<li class = "items">Menu Item-4 </li>
<li class = "items">Menu Item-5 </li>
<li class = "items">Menu Item-6 </li>
</ul>
</div>
"#;

const CONTEXT_MENU_STYLE: &str = r#"
#context-menu-identifier {
    position: absolute;
    background-color: #84abb5;
    color: white;
    height: 150px;
    width: 100px;
    text-align: center;
 }
 .menuItems {
    list-style: none;
    font-size: 12px;
    padding: 0;
    margin: 0;
 }
 .menuItems .items { padding: 5px; border-bottom: 1px solid #e6d4b6;}
 .menuItems .items:last-child { border: none;}
 .menuItems .items a {text-decoration: none; color: white;}
"#;

const EMAIL_HANDLER: &str = r#"
<script>
function email_ops() {
    var inputElement = document.getElementById("user_email");

    inputElement.addEventListener("keyup", function () {
        var inputValue = inputElement.value;

        window.ipc.postMessage(`event:user_mail>${inputValue}`)
    });

}
</script>
"#;

#[async_trait::async_trait]
impl Puppeteer for AppTest {
    fn shell() -> Shell {
        let context_menu_script = ContextMenu::new()
            .add_id("context-menu-identifier")
            .build_script();

        Shell::new()
            // The order in which styles are added matters since CSS is cascading
            .add_style(include_str!("assets/frow.min.css"))
            .add_style("body {background-color: #1a1a1a; color: #FFFFFF;}")
            .add_style("#logo-icon svg{width: 30px}")
            .add_style(".splash-icon>svg{width: 50vw}")
            .add_style(CONTEXT_MENU_STYLE)
            .add_style(DEFAULT_WINDOW_ACTIONS_STYLE)
            .add_scripts([
                DEFAULT_WINDOW_ACTIONS_SCRIPT.into(),
                context_menu_script,
                EMAIL_HANDLER.into(),
            ])
    }

    fn splashscreen() -> ModifyView {
        let splash_html = html!(
            <div class="frow row-center ">
                <div class="splash-icon frow row-center p-20">{ PUPPETEER_LOGO }</div>
            </div>
        );

        ModifyView::ReplaceApp(Cow::Owned(splash_html))
    }

    async fn init(app_env: &ActiveAppEnv) -> ModifyView {
        dbg!(app_env);

        smol::Timer::after(std::time::Duration::from_secs(3)).await;

        let title_bar = html!(
            {CONTEXT_MENU}
            <div class="frow direction-row">
                <div id="logo-icon" class="drag-region frow row-start p-5 col-xs-1-4"> { PUPPETEER_ICON }</div>
                <div class="drag-region frow row-end col-xs-3-4"> { DEFAULT_WINDOW_ACTIONS }</div>
            </div>
            <div class="frow"><h1 style="font-family: 'rockville_solid','sans-serif'">"HELLO from PUPPETEER"</h1></div>
            <div class="frow"><h3 style="font-family: 'centauri','sans-serif'">"Nice Font :)"</h3></div>
            <div class="frow direction-column row-center">
                <input class="frow col-md-1-2 mt-40" type="email" id="user_email" name="name" required placeholder="Enter Your Email Address" onkeydown="email_ops()"/>

                <button onclick="window.ipc.postMessage('event:submit_mail>')">"SUBMIT"</button>
            </div>
        );

        ModifyView::ReplaceApp(Cow::Owned(title_bar))
    }

    fn parse(message: &str) -> Self {
        if message.starts_with("PuppeteerError»") {
            // Handle this error into `Self`
            panic!("Encountered error: {}", message)
        }

        if message.starts_with("event:user_mail>") {
            let user_email = message.to_string().replace("event:user_mail>", "");

            Self::RecvUserEmail(user_email)
        } else if message.starts_with("event:submit_mail>") {
            Self::SubmitEmail
        } else {
            todo!()
        }
    }

    async fn event_handler(&mut self, app_env: ActiveAppEnv) -> ModifyView {
        match self {
            Self::RecvUserEmail(data) => {
                MEM_STORE.lock().await.insert("user_email", data.clone());

                ModifyView::Skip
            }
            Self::Root => {
                println!("ACTIVE_ENV: {:?}", app_env);

                ModifyView::ReplaceApp("EVENT RECV".into())
            }
            Self::SubmitEmail => {
                println!(
                    "THE USER EMAIL IS: {:?}",
                    MEM_STORE.lock().await.get("user_email")
                );

                ModifyView::ReplaceApp("USER EMAIL SUBMITTED".into())
            }
            _ => ModifyView::Skip,
        }
    }

    async fn error_handler(_error: impl std::error::Error + Send) -> ModifyView {
        ModifyView::ReplaceApp("ERROR RECV".into())
    }
}
