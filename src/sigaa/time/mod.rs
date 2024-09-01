#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dia {
    Segunda,
    Terça,
    Quarta,
    Quinta,
    Sexta,
    Sabado,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Turno {
    Manhã,
    Tarde,
    Noite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Horario {
    Primeiro,
    Segundo,
    Terceiro,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SigaaTime {
    pub dia: Dia,
    pub turno: Turno,
    pub horario: Horario,
}

#[derive(Debug)]
pub enum SigaaTimeErrors {
    TriedToCreateN56,
    InvalidUsizeToDay,
    InvalidStringToDay,
    InvalidUsizeToHorario,
    InvalidStringToTurno,
    InvalidStringToSigaaTime,
    InvalidStringToHorario,
}

mod dia;
mod horario;
mod sigaa_time;
mod turno;