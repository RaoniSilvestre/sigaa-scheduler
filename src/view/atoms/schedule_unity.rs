use schedule::ScheduleUnity;
use yew::*;
use yew_autoprops::autoprops;

#[autoprops]
#[function_component]
pub fn UnityComponent(props: &ScheduleUnity) -> Html {
    let default_classes = "table-cell border border-gray-300 p-2 text-center border-blue-500";

    let classes = match props.disciplina {
        Some(_) => format!("{}{}", default_classes, " bg-blue-500"),
        None => default_classes.to_string(),
    };

    html! {
        <div class={classes}>
        {match &props.disciplina {
            Some(disciplina) => disciplina.abreviacao.clone(),
            None => props.horario.to_string()
        }}
        </div>
    }
}
