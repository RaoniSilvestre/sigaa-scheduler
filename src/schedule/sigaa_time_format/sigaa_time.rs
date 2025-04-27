use std::fmt::Display;

use regex::Regex;

use super::{Dia, SigaaTime, SigaaTimeErrors, Turno};

impl SigaaTime {
    /// Cria uma nova instância de `SigaaTime`.
    ///
    /// # Parâmetros
    ///
    /// * `dia` - O dia da semana, do tipo `Dia`.
    /// * `turno` - O turno do dia, do tipo `Turno`.
    ///
    /// # Retorno
    ///
    /// Retorna uma nova instância de `SigaaTime` com o `dia` e `turno` fornecidos.
    ///
    /// # Exemplo
    ///
    /// ```
    /// use stf::{SigaaTime, HorarioDiurno, Turno, Dia};
    /// let sigaa_time = SigaaTime::new(Dia::Segunda, Turno::Manhã(HorarioDiurno::Primeiro));
    /// ```
    pub fn new(dia: Dia, turno: Turno) -> SigaaTime {
        SigaaTime { dia, turno }
    }

    /// Cria uma nova instância de `SigaaTime` a partir de strings.
    ///
    /// As strings são convertidas em seus respectivos tipos `Dia` e `Turno`. Se a conversão falhar,
    /// um erro de tipo `SigaaTimeErrors` será retornado.
    ///
    /// # Parâmetros
    ///
    /// * `dia_str` - Uma string representando o dia da semana.
    /// * `turno_str` - Uma string representando o turno do dia.
    ///
    /// # Retorno
    ///
    /// Retorna `Ok(SigaaTime)` se a conversão for bem-sucedida, ou um `Err(SigaaTimeErrors)` em caso de falha.
    ///
    /// # Exemplo
    ///
    /// ```
    /// use stf::SigaaTime;
    /// match SigaaTime::new_from_strings("2", "M12") {
    ///     Ok(sigaa_time) => println!("Horário criado com sucesso: {:?}", sigaa_time),
    ///     Err(e) => println!("Erro ao criar horário: {:?}", e),
    /// }
    /// ```
    pub fn new_from_strings(dia_str: &str, turno_str: &str) -> Result<SigaaTime, SigaaTimeErrors> {
        let dia: Dia = dia_str.try_into()?;
        let turno: Turno = turno_str.try_into()?;

        Ok(SigaaTime::new(dia, turno))
    }
}

impl TryFrom<&str> for SigaaTime {
    type Error = SigaaTimeErrors;

    fn try_from(value: &str) -> Result<SigaaTime, SigaaTimeErrors> {
        let regex = Regex::new(r"^(\d{1,5})([MTN])(\d{2,6})$").unwrap();

        match regex.captures(value) {
            Some(capturas) => {
                let dias = capturas[1].to_string();
                let turnos = capturas[2].to_string();
                let horarios = capturas[3].to_string();
                let turno_horarios = turnos + &horarios;
                SigaaTime::new_from_strings(&dias, &turno_horarios)
            }
            None => Err(SigaaTimeErrors::InvalidStringToSigaaTime),
        }
    }
}

impl PartialOrd for SigaaTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SigaaTime {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let this_dia_usized: usize = self.dia.into();
        let other_dia_usized: usize = other.dia.into();

        if this_dia_usized != other_dia_usized {
            return this_dia_usized.cmp(&other_dia_usized);
        }

        let this_turno_usized: usize = self.turno.into();
        let other_turno_usized: usize = other.turno.into();

        this_turno_usized.cmp(&other_turno_usized)
    }
}

impl Display for SigaaTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.dia, self.turno)
    }
}
