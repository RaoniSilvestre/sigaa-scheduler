use crate::schedule::SigaaTime;

use super::auxiliary::*;
use super::{Disciplina, DisciplinaErrors};
use std::collections::BTreeSet;
use std::fmt::{self, Display};

impl Disciplina {
    pub fn new(nome: &str, sigaa_time: BTreeSet<SigaaTime>) -> Disciplina {
        Disciplina { sigaa_time, abreviacao: generate_abreviação(nome), nome: nome.to_string() }
    }

    pub fn new_stringify(nome: &str, new_time: &str) -> Result<Disciplina, DisciplinaErrors> {
        if !is_formatted(new_time) {
            return Err(DisciplinaErrors::TimeNotFormatted);
        }

        let new_times: BTreeSet<SigaaTime> = valid_string_to_vec_sigaa_times(new_time);

        Ok(Disciplina { sigaa_time: new_times, abreviacao: generate_abreviação(nome), nome: nome.to_string() })
    }

    pub fn add_time(&mut self, new_time: SigaaTime) -> Result<(), DisciplinaErrors> {
        match self.sigaa_time.insert(new_time) {
            true => Ok(()),
            false => Err(DisciplinaErrors::TimeAlreadyInserted),
        }
    }
}

impl Display for Disciplina {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {} - {}", self.nome, self.abreviacao, generate_horario_display(self))
    }
}

impl fmt::Debug for Disciplina {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -", self.abreviacao)?;
        for sigaa_time in self.sigaa_time.iter() {
            write!(f, " {}", sigaa_time)?
        }

        Ok(())
    }
}
