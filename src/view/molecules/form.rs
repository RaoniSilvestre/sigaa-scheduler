use super::super::atoms::text_input::TextInput;
use std::ops::Deref;
use yew::{function_component, html, use_state, Callback, Html, Properties, SubmitEvent};

#[derive(Properties, PartialEq)]
pub struct FormProps {
    pub on_submit: Callback<Data>,
}

#[derive(Default, Clone)]
pub struct Data {
    pub nome: String,
    pub horario: String,
}

#[function_component]
pub fn FormComponent(props: &FormProps) -> Html {
    let state = use_state(|| Data::default());

    // Isso aqui mantém o nome atualizado no struct Data
    let cloned_state = state.clone();
    let nome_changed = Callback::from(move |nome| {
        let mut data = cloned_state.deref().clone();
        data.nome = nome;
        cloned_state.set(data);
    });

    // Isso aqui mantém o horário atualizado no struct Data
    let cloned_state = state.clone();
    let disciplina_changed = Callback::from(move |horario: String| {
        let mut data = cloned_state.deref().clone();
        data.horario = horario.to_uppercase();
        cloned_state.set(data);
    });

    // Isso aqui processa o form quando o usuário faz submit
    let form_onsubmit = props.on_submit.clone();
    let cloned_state = state.clone();
    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        let data = cloned_state.deref().clone();
        form_onsubmit.emit(data);
    });

    html! {
        <form onsubmit={onsubmit} class="flex flex-col mr-5">
            <TextInput name="Nome da disciplina" handle_onchange={nome_changed} />
            <TextInput name="Horário da disciplina" handle_onchange={disciplina_changed}/>
          <button type="submit" class="p-2 rounded bg-blue-500 text-white cursor-pointer transition duration-300 ease-in-out mb-2 hover:bg-blue-700">{"Adicionar"}</button>
        </form>
    }
}
