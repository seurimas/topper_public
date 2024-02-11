use futures::FutureExt;
use wasm_bindgen::{prelude::Closure, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::HtmlInputElement;
use yew::prelude::*;
mod comment;
mod line;
mod page;
mod state;

use crate::{
    bindings::*,
    explainer::ExplainerPage,
    links::{check_for_link, load_file, load_page},
    msg::ExplainerMessage,
    sect_parser::{load_sect_into_iframe, AetoliaSectParser},
};

use self::page::ExplainerPageModel;

#[derive(Debug)]
pub enum ExplainerModel {
    Welcome,
    Loading,
    Parsing(AetoliaSectParser),
    LoadedPage(ExplainerPage),
    Published(Vec<String>),
}

impl Default for ExplainerModel {
    fn default() -> Self {
        Self::Welcome
    }
}

#[derive(Properties, PartialEq, Default)]
pub struct ExplainerProps;

impl Component for ExplainerModel {
    type Message = ExplainerMessage;
    type Properties = ExplainerProps;

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self {
            Self::Welcome => {
                let on_sect_log = ctx.link().batch_callback(|e: Event| {
                    e.target()
                        .and_then(|target| target.dyn_into::<HtmlInputElement>().ok())
                        .and_then(|target| target.files())
                        .and_then(|files| files.get(0))
                        .map(|file| ExplainerMessage::LoadFile(file.text().into()))
                });
                html!(<div key="welcome" class="welcome">
                  <span class="info">
                  {"Welcome to Seurimas' Explainer tool.
                  
                  The purpose of this tool is to provide a means of explaining concepts in the MUD Aetolia, using inline comments. The commented logs can be exported to JSON, shared, and then loaded into this tool to view the comments again.
                  
                  As an added utility, Sect logs can be loaded into this tool. The tool will parse the log and provide inline insights into the state of the fight: afflictions, limb state, and critical balances.
                  
                  When editing a page, you can click on this icon to add a comment: "}
                  <div class="page__add_comment">{"+"}</div>
                  {"
                  While editing or viewing a Sect log, this icon shows up with every prompt to show the battle state: "}
                  <div class="page__view_state">{"?"}</div>
                  <br/>
                  {"This battle state is only supported: for some classes offensively; with simple config options defensively; and will not work after the target leaves the room."}
                  <br/>
                  </span>
                  <label for="sect_log">{"Select an exported JSON file or a downloaded Sect log:"}</label>
                  <input type="file" id="sect_log" name="sect_log" onchange={on_sect_log}/>
                </div>)
            }
            Self::Loading => html!(<div key="loading">{ "Loading..." }</div>),
            Self::Parsing(parser) => html!(<>
                    <a id="export"></a>
                    {load_sect_into_iframe(ctx, &parser.text)}
                </>
            ),
            Self::LoadedPage(page) => {
                log("Rendering page...");
                html!(<ExplainerPageModel
                  page={page.clone()}
                />)
            }
            Self::Published(published) => html!(<div key="welcome" class="welcome">
                <span class="info">
                {"Welcome to Seurimas' Explainer tool's personal, published logs.
                
                The main page for this tool (for use with your own logs) is here: "}
                <a href="?">{"Seurimas' Explainer"}</a>
                {"

                The purpose of this tool is to provide a means of explaining concepts in the MUD Aetolia, using inline comments. The commented logs can be exported to JSON, shared, and then loaded into this tool to view the comments again.
                
                As an added utility, Sect logs can be loaded into this tool. The tool will parse the log and provide inline insights into the state of the fight: afflictions, limb state, and critical balances.

                While viewing a Sect log, this icon shows up with every prompt to show the battle state: "}
                <div class="page__view_state">{"?"}</div>
                <br/>
                {"This battle state is only supported: for some classes offensively; with simple config options defensively; and will not work after the target leaves the room."}
                <br/>
                <br/>
                {"The following logs are available for viewing:"}
                <ul>
                { for published.iter().map(|id| html!(<li><a class="published" href={format!("?my_logs/{id}.json")}>{id}</a></li>)) }
                </ul>
                </span>
            </div>),
            unrendered => html!(<div key="unknown">{ format!("No view: {:?}", unrendered) }</div>),
        }
    }

    fn create(ctx: &Context<Self>) -> Self {
        check_for_link(ctx, ExplainerMessage::LoadPage);
        ExplainerModel::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ExplainerMessage::LoadFile(future) => {
                ctx.link().send_future(future.then(load_file));
                *self = Self::Loading;
                true
            }
            ExplainerMessage::LoadPage(future) => {
                ctx.link().send_future(future.then(load_page));
                *self = Self::Loading;
                true
            }
            ExplainerMessage::LoadedFile(loaded) => {
                match serde_json::from_str::<ExplainerPage>(&loaded) {
                    Ok(page) => {
                        log("Found page from file!");
                        set_title(&page.id);
                        *self = Self::LoadedPage(page);
                    }
                    _ => {
                        log("Assuming non-page file is Sect log!");
                        *self = Self::Parsing(AetoliaSectParser::new(loaded));
                    }
                }
                true
            }
            ExplainerMessage::LoadedPage(loaded) => {
                log(&format!("Loaded {}!", loaded.len()));
                set_title(&loaded.id);
                *self = Self::LoadedPage(loaded);
                true
            }
            ExplainerMessage::LoadedPublished(mut published) => {
                log(&format!("Loaded published {}!", published.len()));
                published.reverse();
                *self = Self::Published(published);
                true
            }
            ExplainerMessage::InitializeSect(iframe) => match self {
                Self::Parsing(parser) => {
                    parser.parse_nodes(&iframe);
                    log("Parsed!");
                    let page = parser.get_page();
                    *self = Self::LoadedPage(page);
                    true
                }
                _ => {
                    log("Bad state for initialization...");
                    false
                }
            },
            ExplainerMessage::Error(error) => {
                log(&error);
                false
            }
            ExplainerMessage::Noop => false,
        }
    }
}
