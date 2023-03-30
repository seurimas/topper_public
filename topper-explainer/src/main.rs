use yew::prelude::*;

#[derive(Default)]
pub struct ExplainerModel;

#[derive(Properties, PartialEq, Default)]
pub struct ExplainerProps;

pub enum ExplainerMessage {
    Noop,
}

impl Component for ExplainerModel {
    type Message = ExplainerMessage;
    type Properties = ExplainerProps;

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!(<div>{ "Just a test" }</div>)
    }

    fn create(ctx: &Context<Self>) -> Self {
        ExplainerModel::default()
    }
}

fn main() {
    yew::Renderer::<ExplainerModel>::new().render();
}
