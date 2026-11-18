use crate::clase::Clase;
use crate::error_margen::ErrorMargen;
use crate::ubicacion_coche::UbicacionCoche;
use std::collections::BTreeMap;
use std::time::{Duration, SystemTime};

pub struct Agenda {
    clases: BTreeMap<SystemTime, Clase>,
    ubicacion_coche: Option<UbicacionCoche>,
}

impl Agenda {
    pub fn new() -> Self {
        Agenda {
            clases: BTreeMap::new(),
            ubicacion_coche: None,
        }
    }

    pub fn agregar_clase(&mut self, inicio: SystemTime, clase: Clase) {
        self.clases.insert(inicio, clase);
    }

    pub fn registrar_aviso(
        &mut self,
        inicio: SystemTime,
        aviso: SystemTime,
    ) -> Result<(), ErrorMargen> {
        let clase = self
            .clases
            .get_mut(&inicio)
            .ok_or(ErrorMargen::ClaseInexistente)?;
        clase.registrar_aviso(aviso);
        Ok(())
    }

    pub fn margen(&self, inicio: SystemTime) -> Result<Duration, ErrorMargen> {
        self.clases
            .get(&inicio)
            .ok_or(ErrorMargen::ClaseInexistente)?
            .margen(inicio)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dni::Dni;
    use std::time::UNIX_EPOCH;

    fn hora(horas: u64, minutos: u64) -> SystemTime {
        UNIX_EPOCH + Duration::from_secs(horas * 3600 + minutos * 60)
    }

    fn agenda_con_clase_a_las_17() -> Agenda {
        let mut agenda = Agenda::new();
        agenda.agregar_clase(hora(17, 0), Clase::new(Dni::new(12345678, 'Z')));
        agenda
    }

    fn margen_con_aviso_a_las(aviso: SystemTime) -> Result<Duration, ErrorMargen> {
        let mut agenda = agenda_con_clase_a_las_17();
        agenda.registrar_aviso(hora(17, 0), aviso).unwrap();
        agenda.margen(hora(17, 0))
    }

    #[test]
    fn aviso_a_las_1610_para_clase_a_las_17_deja_50_minutos() {
        assert_eq!(
            margen_con_aviso_a_las(hora(16, 10)),
            Ok(Duration::from_secs(50 * 60)),
            "un aviso a las 16:10 para una clase a las 17:00 debería dejar 50 minutos de margen"
        );
    }

    #[test]
    fn clase_sin_aviso_no_tiene_margen() {
        let agenda = agenda_con_clase_a_las_17();

        assert_eq!(
            agenda.margen(hora(17, 0)),
            Err(ErrorMargen::SinAviso),
            "una clase sin aviso de cancelación no debería tener margen"
        );
    }

    #[test]
    fn aviso_a_la_hora_de_inicio_no_deja_margen() {
        assert_eq!(
            margen_con_aviso_a_las(hora(17, 0)),
            Err(ErrorMargen::AvisoTardio),
            "un aviso a la misma hora en que empieza la clase no debería dejar margen"
        );
    }

    #[test]
    fn aviso_posterior_al_inicio_no_deja_margen() {
        assert_eq!(
            margen_con_aviso_a_las(hora(17, 5)),
            Err(ErrorMargen::AvisoTardio),
            "un aviso posterior al inicio de la clase no debería dejar margen"
        );
    }

    #[test]
    fn hora_sin_clase_no_tiene_margen() {
        let agenda = agenda_con_clase_a_las_17();

        assert_eq!(
            agenda.margen(hora(18, 0)),
            Err(ErrorMargen::ClaseInexistente),
            "una hora sin clase en la agenda no debería tener margen"
        );
    }

    #[test]
    fn no_se_puede_registrar_un_aviso_a_una_hora_sin_clase() {
        let mut agenda = agenda_con_clase_a_las_17();

        assert_eq!(
            agenda.registrar_aviso(hora(18, 0), hora(16, 10)),
            Err(ErrorMargen::ClaseInexistente),
            "no debería poder registrarse un aviso a una hora sin clase"
        );
    }
}
