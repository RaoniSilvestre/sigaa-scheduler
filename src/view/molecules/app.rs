use super::super::atoms::github_link::GithubLink;
use super::super::atoms::total_hour::TotalHourComponent;
use super::discipline_list::DisciplineListComponent;
use super::form::*;
use super::schedule::ScheduleComponent;
use class::Disciplina;
use gloo::console::log;
use gloo_storage::{LocalStorage, Storage};
use schedule::Schedule;
use std::ops::Deref;
use yew::*;

#[function_component]
pub fn App() -> Html {
    let schedule = use_state(|| Schedule::new());

    let total_hours = use_state(|| 0);

    let schedule_clone = schedule.clone();
    {
        use_effect_with((), move |_| {
            let disciplinas: Vec<Disciplina> = get_saved_disciplines();
            log!("Loading saved disciplines...");

            let mut new_schedule = schedule_clone.deref().clone();
            for disciplina in disciplinas {
                log!(&format!("{:?}", &disciplina));
                new_schedule.insert(disciplina).unwrap();
            }

            schedule_clone.set(new_schedule);
        });
    }
    let on_form_submit = {
        let schedule = schedule.clone();
        Callback::from(move |form_data: Data| {
            let updated_schedule = updated_schedule_with_form_data(schedule.deref().clone(), &form_data);
            schedule.set(updated_schedule);
        })
    };

    let remove_discipline_from_schedule = {
        let schedule = schedule.clone();
        Callback::from(move |disciplina: Disciplina| {
            let mut updated_schedule = schedule.deref().clone();
            updated_schedule
                .remove(disciplina.clone())
                .unwrap();
            schedule.set(updated_schedule);

            let mut disciplinas = get_saved_disciplines();
            disciplinas.retain(|d| d != &disciplina);
            LocalStorage::set("disciplinas", &disciplinas).expect("Failed to update LocalStorage");
        })
    };
    let discipline_list = get_saved_disciplines();

    {
        let schedule = schedule.clone();
        let total_hours = total_hours.clone();
        use_effect_with(schedule.clone(), move |_| {
            let new_total_hours = calculate_total_hours();
            total_hours.set(new_total_hours);
            || ()
        });
    }
    html! {
    <div class="flex flex-row justify-around items-start bg-white p-5 rounded-lg shadow-md w-4/5 max-w-[1200px]">
        <div class="relative flex flex-col space-y-4" >
            <FormComponent on_submit={on_form_submit}/>
            <TotalHourComponent total_hours={*total_hours}/>
            <GithubLink />
            </div>
        <div class="w-full">
            <ScheduleComponent schedule={(*schedule).clone()} />
            <DisciplineListComponent discipline_list={discipline_list} on_remove={remove_discipline_from_schedule}/>
        </div>
    </div>
    }
}

fn updated_schedule_with_form_data(mut sched: Schedule, form_data: &Data) -> Schedule {
    log!(format!("FormData values: \n    Nome da disciplina: {}\n    Horário da disciplina: {}", form_data.nome.clone(), form_data.horario.clone()));

    let new_disciplina = Disciplina::new_stringify(&form_data.nome, &form_data.horario);

    match new_disciplina {
        Ok(disciplina) => match sched.insert(disciplina.clone()) {
            Ok(_) => add_disciplina_to_local_storage(disciplina),
            Err(error) => gloo::dialogs::alert(&format!("{:?}", error)),
        },
        Err(error) => gloo::dialogs::alert(&format!("{:?}", error)),
    }

    sched
}

fn add_disciplina_to_local_storage(disciplina: Disciplina) {
    let mut disciplinas: Vec<Disciplina> = get_saved_disciplines();
    disciplinas.push(disciplina);
    LocalStorage::set::<Vec<Disciplina>>("disciplinas", disciplinas).unwrap();
}

pub fn get_saved_disciplines() -> Vec<Disciplina> {
    match LocalStorage::get("disciplinas") {
        Ok(val) => val,
        Err(_) => {
            LocalStorage::set::<Vec<Disciplina>>("disciplinas", vec![]).unwrap();
            LocalStorage::get("disciplinas").unwrap()
        }
    }
}

fn calculate_total_hours() -> usize {
    let disciplinas = get_saved_disciplines();
    let mut output = 0;
    for dis in &disciplinas {
        output += dis.sigaa_time.len() * 30;
    }
    output
}
