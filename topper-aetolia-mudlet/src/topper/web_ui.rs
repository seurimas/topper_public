use std::thread;
use topper_aetolia::timeline::AetTimeSlice;
use topper_core_mudlet::topper::{TopperMessage, TopperModule, TopperRequest, TopperResponse};
use web_view::*;

use crate::sect_parser;

use super::battle_stats::BattleStats;

pub struct WebModule {
    publish_location: Option<String>,
    thread: Option<thread::JoinHandle<()>>,
}

impl WebModule {
    pub fn new(publish_location: Option<String>) -> Self {
        WebModule {
            publish_location,
            thread: None,
        }
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

impl<'s> TopperModule<'s, AetTimeSlice, BattleStats> for WebModule {
    type Siblings = ();
    fn handle_message(
        &mut self,
        message: &TopperMessage<AetTimeSlice>,
        siblings: Self::Siblings,
    ) -> Result<TopperResponse<BattleStats>, String> {
        match message {
            TopperMessage::Request(TopperRequest::ModuleMsg(module, message)) => {
                if module == "web" {
                    println!("Web module received message: {}", message);
                    if message.starts_with("publish_log ") {
                        let log_url = message.trim_start_matches("publish_log ");
                        println!("Retrieving Sect log...");
                        if let Ok(sect_log) = get_sect_log(log_url) {
                            println!("Sect log retrieved {}", sect_log.len());
                            let mut parser = sect_parser::AetoliaSectParser::new();
                            if let Ok(mut explainer_page) = parser.parse_nodes(sect_log) {
                                println!("Explainer page parsed successfully!");
                                explainer_page.filter_out_from_body("Tells");
                                explainer_page.filter_out_from_body("a sprawling venantium cuff");
                                explainer_page.filter_out_from_body("an insignia of the Blades");
                                explainer_page.filter_out_command("tell");
                                explainer_page.filter_out_command("gtells");
                                explainer_page.filter_out_command("gtstells");
                                explainer_page.filter_out_command("clan");
                                explainer_page.filter_out_command("gw");
                                explainer_page.filter_out_command("cw");
                                explainer_page.filter_out_command("who");
                                explainer_page.filter_out_command("rm");
                                explainer_page.filter_out_command("rn");
                                explainer_page.filter_out_command("nstat");
                                explainer_page.filter_out_command("message");
                                explainer_page.filter_out_command("msg");
                                explainer_page.filter_out_command("tell");
                                explainer_page.locked = true;
                                if let Some(publish_location) = &self.publish_location {
                                    let mut file = std::fs::File::create(format!(
                                        "{}/{}.json",
                                        publish_location, explainer_page.id
                                    ))
                                    .unwrap();
                                    serde_json::to_writer_pretty(&file, &explainer_page).unwrap();
                                    let published_file = std::fs::File::open(format!(
                                        "{}/published.json",
                                        publish_location
                                    ))
                                    .unwrap();
                                    let mut published: Vec<String> =
                                        serde_json::from_reader(published_file).unwrap();
                                    if !published.contains(&explainer_page.id) {
                                        published.push(explainer_page.id.clone());
                                        let mut published_file = std::fs::File::create(format!(
                                            "{}/published.json",
                                            publish_location
                                        ))
                                        .unwrap();
                                        serde_json::to_writer_pretty(&published_file, &published);
                                    }
                                } else {
                                    println!("No publish location set");
                                }
                            }
                        }
                    }
                }
                Ok(TopperResponse::silent())
            }
            _ => Ok(TopperResponse::silent()),
        }
    }
}

#[tokio::main]
async fn get_sect_log(log_url: &str) -> Result<String, reqwest::Error> {
    let res = reqwest::get(log_url).await?;
    let body = res.text().await?;
    Ok(body)
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
