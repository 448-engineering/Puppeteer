use puppeteer::{EventHandler, ModifyView, Puppeteer};

fn main() {
    Puppeteer::new("Simple App")
        .unwrap()
        .run::<UserEvents>()
        .unwrap();

    // .add_style("splash_animation", SPLASH_ANIMATION_CSS);
    //.set_title_bar_type(puppeteer::TitleBarType::Native)

    //app.set_title_bar_type(puppeteer::TitleBarType::Native)
    //app.shell.set_theme(Theme::Light);

    /*let shell = Shell::default()
    .set_scripts(include_str!("./simple.html"))
    .set_style(include_str!("./simple.css"));*/

    //app.set_shell(shell);
}

#[derive(Debug)]
enum UserEvents {
    NewPage,
    Unsupported,
}

impl From<String> for UserEvents {
    fn from(value: String) -> Self {
        match value.as_str() {
            "success_route" => UserEvents::NewPage,
            _ => UserEvents::Unsupported,
        }
    }
}

#[async_trait::async_trait]
impl EventHandler for UserEvents {
    async fn init_func() -> ModifyView {
        println!("RUNNING INIT");

        std::thread::sleep(std::time::Duration::from_secs(2));

        let data = r#"
    <div>AFTER SPLASH</div>
    <button onclick="window.ipc.postMessage('success_route')">ROUTE TO NEW PAGE</button>
    "#;

        ModifyView::replace_view(data.into())
    }

    async fn view_model(&self) -> ModifyView {
        println!("ROUTING");

        std::thread::sleep(std::time::Duration::from_secs(5));

        let data = "<div>SUCCESS ROUTING</div>";

        ModifyView::replace_app(data.into())
    }
}

pub const PUPPETEER_ANIMATION: &str = r#"
<div class="lds-ellipsis">
    <div></div>
    <div></div>
    <div></div>
    <div></div>
</div>
"#;

pub const SPLASH_ANIMATION_CSS: &str = r#"
#splashscreen {
    width: 100%;
}
.lds-ellipsis {
    display: inline-block;
    position: relative;
    width: 80px;
    height: 80px;
  }
  .lds-ellipsis div {
    position: absolute;
    top: 33px;
    width: 13px;
    height: 13px;
    border-radius: 50%;
    background: #fff;
    animation-timing-function: cubic-bezier(0, 1, 1, 0);
  }
  .lds-ellipsis div:nth-child(1) {
    left: 8px;
    animation: lds-ellipsis1 0.6s infinite;
  }
  .lds-ellipsis div:nth-child(2) {
    left: 8px;
    animation: lds-ellipsis2 0.6s infinite;
  }
  .lds-ellipsis div:nth-child(3) {
    left: 32px;
    animation: lds-ellipsis2 0.6s infinite;
  }
  .lds-ellipsis div:nth-child(4) {
    left: 56px;
    animation: lds-ellipsis3 0.6s infinite;
  }
  @keyframes lds-ellipsis1 {
    0% {
      transform: scale(0);
    }
    100% {
      transform: scale(1);
    }
  }
  @keyframes lds-ellipsis3 {
    0% {
      transform: scale(1);
    }
    100% {
      transform: scale(0);
    }
  }
  @keyframes lds-ellipsis2 {
    0% {
      transform: translate(0, 0);
    }
    100% {
      transform: translate(24px, 0);
    }
  }
  
"#;
