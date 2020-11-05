use crate::topper::{TopperMessage, TopperModule, TopperRequest, TopperResponse};
use std::thread;
use web_view::*;

pub struct WebModule {
    thread: Option<thread::JoinHandle<()>>,
}

impl WebModule {
    pub fn new() -> Self {
        WebModule { thread: None }
    }

    pub fn display(&mut self) {
        let url = "http://localhost:9000";
        println!("Displaying: {}", url);
        self.thread = Some(thread::spawn(move || {
            web_view::builder()
                .title("Topper UI")
                .content(Content::Url(url))
                .size(320, 480)
                .resizable(false)
                .debug(true)
                .user_data(())
                .invoke_handler(|webview, arg| match arg {
                    "test_one" => {
                        println!("Testing");
                        promise_result(webview, arg, "Test complete")
                    }
                    "test_two" => webview.eval(&format!("alert(\"Test\")")),
                    _ => unimplemented!(),
                })
                .build()
                .unwrap()
                .run()
                .unwrap();
        }));
    }
}

impl<'s> TopperModule<'s> for WebModule {
    type Siblings = ();
    fn handle_message(
        &mut self,
        message: &TopperMessage,
        siblings: Self::Siblings,
    ) -> Result<TopperResponse, String> {
        match message {
            TopperMessage::Request(TopperRequest::OpenWeb) => {
                self.display();
                Ok(TopperResponse::silent())
            }
            _ => Ok(TopperResponse::silent()),
        }
    }
}

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}

fn promise_result<T>(
    webview: &mut WebView<T>,
    promise_id: &str,
    s: &str,
) -> Result<(), web_view::Error> {
    let promise_resolver = format!("resolve_promise(\"{}\", \"{}\")", promise_id, s);
    webview.eval(&promise_resolver)
}
