use yew::prelude::*;
use yew_linkify::Linkify;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{"Yew Linkify Example"}</h1>
            <Linkify text={"Contact me at test@example.com or visit https://example.com!".to_string()} />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
