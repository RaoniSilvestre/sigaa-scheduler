use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TotalHoursProps {
    pub total_hours: usize,
}

#[function_component(TotalHourComponent)]
pub fn total_hour_component(props: &TotalHoursProps) -> Html {
    let text = format!("Total de horas: {}h", props.total_hours);
    html! {
    <div class="bottom-0 left-0 mr-5">
        <p class="text-xl bg-blue-500 text-white p-3 rounded-lg shadow-lg">{ text }</p>
    </div>
    }
}
