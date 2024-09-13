use super::super::atoms::discipline::DisciplineButton;
use class::Disciplina;
use yew::*;

#[derive(Properties, PartialEq)]
pub struct DisciplineListProps {
    pub discipline_list: Vec<Disciplina>,
    pub on_remove: Callback<Disciplina>, // Callback para remover a disciplina
}

#[function_component]
pub fn DisciplineListComponent(props: &DisciplineListProps) -> Html {
    html! {
        <div>
            <ul>
                {for props.discipline_list.iter().enumerate().map(|(index, disciplina)| {

                    html! {
                        <DisciplineButton key={index} disciplina={disciplina.clone()} onremove={props.on_remove.clone()}/>
                    }
                })}
            </ul>
        </div>
    }
}
