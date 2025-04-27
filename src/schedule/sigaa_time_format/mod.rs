use serde::{Deserialize, Serialize};

/// Dias da semana utilizados nos horários.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Dia {
    Segunda,
    Terça,
    Quarta,
    Quinta,
    Sexta,
    Sabado,
}

/// Turnos do dia para horários
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Turno {
    /// Turnos diurnos (manhã e tarde). Cada um tem três possibilidades:
    ///
    /// Turnos da manhã: M12, M34, M56.
    /// Turnos da tarde: T12, T34, T56.
    Manhã(HorarioDiurno),
    Tarde(HorarioDiurno),
    /// Turno noturno. Tem duas possibilidades:
    ///
    /// N12, N34
    Noite(HorarioNoturno),
}

/// Horários diurnos
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HorarioDiurno {
    Primeiro,
    Segundo,
    Terceiro,
}

/// Horários noturnos
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HorarioNoturno {
    Primeiro,
    Segundo,
}

/// Representa um horário específico em um dia e turno.
///
/// Exemplo:
/// 2T56 , 4M12, 6N34
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SigaaTime {
    pub dia: Dia,
    pub turno: Turno,
}

/// Erros relacionados ao `SigaaTime`.
///
/// Estes erros cobrem problemas de conversão e formatação para `SigaaTime`.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SigaaTimeErrors {
    /// Erro ao converter um `usize` para um dia.
    InvalidUsizeToDay,
    /// Erro ao converter uma string para um dia.
    InvalidStringToDay,
    /// Erro ao converter um `usize` para um horário.
    InvalidUsizeToHorario,
    /// Erro ao converter uma string para um turno.
    InvalidStringToTurno,
    /// Erro ao converter uma string para um `SigaaTime`.
    InvalidStringToSigaaTime,
    /// Erro ao converter uma string para um horário.
    InvalidStringToHorario,
}

mod dia;
mod sigaa_time;
mod turno;
