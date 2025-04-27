use class::Disciplina;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DisciplineButtonProps {
    pub disciplina: Disciplina,
    pub onremove: Callback<Disciplina>,
}

#[function_component(DisciplineButton)]
pub fn discipline_button(props: &DisciplineButtonProps) -> Html {
    let disciplina = props.disciplina.clone();
    let on_remove = props.onremove.clone();

    let hours = disciplina.sigaa_time.len() * 30;

    let text = format!("{} ({}) - {}H", disciplina.nome, disciplina.abreviacao, hours);

    let on_click_remove = Callback::from(move |_| on_remove.emit(disciplina.clone()));
    html! {
    <button class="w-full p-2 rounded cursor-pointer transition duration-300 ease-in-out mb-2 hover:bg-blue-700 border " onclick={on_click_remove}>
        { text }
    </button>
    }
}
