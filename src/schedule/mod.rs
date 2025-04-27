mod class;
mod schedule;
mod schedule_unity;
mod sigaa_time_format;
use serde::{Deserialize, Serialize};

pub use class::*;
pub use sigaa_time_format::*;

/// Representa uma unidade de horário em um cronograma.
///
/// Uma `ScheduleUnity` contém um horário específico (`SigaaTime`) e uma disciplina opcional
/// associada a esse horário.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduleUnity {
    /// O horário específico para esta unidade.
    pub horario: SigaaTime,
    /// A disciplina associada a este horário, se houver.
    pub disciplina: Option<Disciplina>,
}

/// Representa um cronograma composto por uma matriz de unidades de horário.
///
/// O cronograma é uma coleção bidimensional de `ScheduleUnity`, onde cada `Vec` representa uma linha
/// de horários.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Schedule(Vec<Vec<ScheduleUnity>>);

/// Erros que podem ocorrer ao trabalhar com cronogramas.
///
/// Estes erros cobrem conflitos entre disciplinas, horários não encontrados e erros relacionados ao `SigaaTime`.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ScheduleError {
    /// Disciplina conflitante com outra disciplina.
    ConflictingDisciplines(Disciplina, Disciplina),
    /// Horário não encontrado no cronograma.
    TimeNotFound(SigaaTime),
    /// Erros associados ao `SigaaTime`.
    SigaaTimeError(SigaaTimeErrors),
    /// Disciplina não encontrada para remoção
    DisciplineNotFoundToRemove,
}

/// Resultado de busca de disciplina.
///
/// Indica se a disciplina foi encontrada ou não.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum DisciplineWasFound {
    /// Disciplina encontrada com sucesso.
    DisciplineFound(Disciplina),
    /// Disciplina não encontrada.
    DisciplineNotFound,
}
