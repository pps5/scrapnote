#[tokio::main]
async fn main() {
    let (port, _) = scrapnote::start_server(None, scrapnote::Config::default());
    webview_official::WebviewBuilder::new()
        .title("scrapnote")
        .height(600)
        .width(500)
        .resize(webview_official::SizeHint::FIXED)
        .debug(true)
        .url(&format!("http://127.0.1:{}", port))
        .build()
        .run();
}
