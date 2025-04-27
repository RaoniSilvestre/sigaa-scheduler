use schedule::*;
use stf::*;
use yew::prelude::*;

use super::super::atoms::schedule_unity::UnityComponent;

#[derive(Properties, PartialEq)]
pub struct ScheduleProps {
    pub schedule: Schedule,
}

#[function_component]
pub fn ScheduleComponent(props: &ScheduleProps) -> Html {
    let x: Vec<Vec<Html>> = (0..8)
        .map(|x| create_row(x, &props.schedule))
        .collect();

    html! {
        <div class="container mx-auto p-4 ">
            <table class="w-full border-collapse border border-gray-300 ">
            {
                for x.into_iter().map(|row| {
                    html! {
                        <div class="table-row">
                            { for row.into_iter() }
                        </div>
                    }
                })
            }
            </table>
        </div>
    }
}

pub fn to_stf((x, y): (usize, usize)) -> SigaaTime {
    let turno: Turno = x.try_into().unwrap();
    let dia: Dia = y.try_into().unwrap();

    SigaaTime::new(dia, turno)
}

pub fn to_schedule_unity((siga_time, schedule): (SigaaTime, &Schedule)) -> &ScheduleUnity {
    schedule.get(&siga_time)
}

pub fn to_html(unity: &ScheduleUnity) -> Html {
    html! { <UnityComponent props={unity.clone()}/> }
}

pub fn create_row(row: usize, schedule: &Schedule) -> Vec<Html> {
    (0..6)
        .map(|col| (row, col))
        .map(to_stf)
        .map(|x| (x, schedule))
        .map(to_schedule_unity)
        .map(to_html)
        .collect::<Vec<Html>>()
}
