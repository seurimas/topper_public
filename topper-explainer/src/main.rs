use explainer::ExplainerModel;
mod bindings;
mod explainer;
mod links;
mod msg;

fn main() {
    yew::Renderer::<ExplainerModel>::new().render();
}
