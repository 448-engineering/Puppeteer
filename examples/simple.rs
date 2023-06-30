use puppeteer::{ModifyView, Puppeteer};

fn main() {
    let mut app = Puppeteer::new("Simple App").unwrap();

    // .add_style("splash_animation", SPLASH_ANIMATION_CSS);
    //.set_title_bar_type(puppeteer::TitleBarType::Native)

    //app.set_title_bar_type(puppeteer::TitleBarType::Native)
    //app.shell.set_theme(Theme::Light);

    /*let shell = Shell::default()
    .set_scripts(include_str!("./simple.html"))
    .set_style(include_str!("./simple.css"));*/

    //app.set_shell(shell);
    app.with_root_page(root_page);
    app.register_event(("success_route", success));
    app.run(init).unwrap();
}

pub fn init() -> bool {
    println!("RUNNING INIT");

    std::thread::sleep(std::time::Duration::from_secs(2));

    true
}

pub fn root_page() -> ModifyView {
    let data = r#"
    <div>AFTER SPLASH</div>
    <button onclick="window.ipc.postMessage('success_route')">ROUTE TO NEW PAGE</button>
    "#;

    ModifyView::replace_app(data.into())
}

pub fn success() -> ModifyView {
    let data = "<div>SUCCESS ROUTING</div>";

    ModifyView::replace_app(data.into())
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
