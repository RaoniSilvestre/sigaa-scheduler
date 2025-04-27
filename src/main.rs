use sigaa_scheduler::app::App;
use yew::prelude::*;

#[function_component]
fn Main() -> Html {
    html! {
        <div class="flex justify-center items-center h-screen m-0 bg-[rgba(0,123,255,0.2)] font-sans">
        <App/>
        </div>
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}
