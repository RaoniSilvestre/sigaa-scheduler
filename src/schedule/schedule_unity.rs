use super::ScheduleUnity;
use class::Disciplina;
use stf::SigaaTime;

impl ScheduleUnity {
    /// Cria uma nova instância de `ScheduleUnity`.
    ///
    /// # Parâmetros
    ///
    /// * `horario` - O horário associado a esta unidade.
    /// * `disciplina` - A disciplina associada a esta unidade (pode ser `None`).
    ///
    /// # Retorno
    ///
    /// Retorna uma nova instância de `ScheduleUnity`.
    pub fn new(horario: SigaaTime, disciplina: Option<Disciplina>) -> ScheduleUnity {
        ScheduleUnity {
            horario,
            disciplina,
        }
    }

    pub fn update(&mut self, disciplina: Option<Disciplina>) {
        self.disciplina = disciplina
    }
}
