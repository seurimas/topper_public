use futures::FutureExt;
use yew::prelude::*;
mod comment;
mod line;
pub mod page;

use crate::{
    bindings::*,
    links::{check_for_link, load_link},
    msg::ExplainerMessage,
};

pub use self::comment::Comment;
pub use self::page::ExplainerPage;
use self::page::ExplainerPageModel;

pub enum ExplainerModel {
    Welcome,
    Loading,
    Loaded(ExplainerPageModel),
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
            Self::Welcome => html!(<div>{ "Welcome! This is just a landing page." }</div>),
            Self::Loading => html!(<div>{ "Loading..." }</div>),
            Self::Loaded(page_model) => page_model.view(ctx),
        }
    }

    fn create(ctx: &Context<Self>) -> Self {
        check_for_link(ctx, ExplainerMessage::Load);
        ExplainerModel::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ExplainerMessage::Load(future) => {
                ctx.link().send_future(future.then(load_link));
                *self = Self::Loading;
                true
            }
            ExplainerMessage::Loaded(loaded) => {
                *self = Self::Loaded(ExplainerPageModel::new(loaded));
                true
            }
            ExplainerMessage::Error(error) => {
                log(&error);
                false
            }
            ExplainerMessage::ExplainerPageMessage(page_message) => match self {
                Self::Loaded(page_model) => page_model.update(ctx, page_message),
                _ => false,
            },
            ExplainerMessage::Noop => false,
        }
    }
}
