use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub handle_onchange: Callback<String>,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let handle_onchange = props.handle_onchange.clone();
    let onchange = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        handle_onchange.emit(value);
    });
    html! {
        <div class="w-full">
            <label class="font-bold mb-1">{props.name.clone()}</label> <br />
            <input
                type="text"
                name={props.name.clone()} onchange={onchange}
                class="mb-2 p-2 rounded border border-gray-300"
            />
            <br/>
    </div>

    }
}
