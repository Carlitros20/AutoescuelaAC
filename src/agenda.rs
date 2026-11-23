use crate::alumnos_registrados::AlumnosRegistrados;
use crate::clase::Clase;
use crate::codigos_postales::CodigosPostales;
use crate::error_busqueda_sustituto::ErrorBusquedaSustituto;
use crate::error_margen::ErrorMargen;
use crate::ubicacion_coche::UbicacionCoche;
use std::collections::{BTreeMap, HashSet};
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

    pub fn registrar_ubicacion_coche(&mut self, codigo_postal: String, instante: SystemTime) {
        self.ubicacion_coche = Some(UbicacionCoche::new(codigo_postal, instante));
    }

    pub fn alumnos_alcanzables(
        &self,
        inicio: SystemTime,
        alumnos: &AlumnosRegistrados,
        codigos: &CodigosPostales,
    ) -> Result<HashSet<u32>, ErrorBusquedaSustituto> {
        let clase = self
            .clases
            .get(&inicio)
            .ok_or(ErrorMargen::ClaseInexistente)?;
        let margen = clase.margen(inicio)?;
        let ubicacion = self
            .ubicacion_coche
            .as_ref()
            .ok_or(ErrorBusquedaSustituto::SinUbicacionCoche)?;
        let mut alcanzables = ubicacion.alumnos_alcanzables(alumnos, codigos, margen)?;
        alcanzables.retain(|numero| !clase.es_del_alumno(*numero));
        Ok(alcanzables)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::UNIX_EPOCH;

    fn hora(horas: u64, minutos: u64) -> SystemTime {
        UNIX_EPOCH + Duration::from_secs(horas * 3600 + minutos * 60)
    }

    fn agenda_con_clase_a_las_17() -> Agenda {
        let mut agenda = Agenda::new();
        agenda.agregar_clase(hora(17, 0), Clase::new(1));
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

    fn alumnos_en_18007_18009_y_18001() -> AlumnosRegistrados {
        AlumnosRegistrados::desde_csv(
            "numero,nombre,dni,codigo_postal\n1,Lu*** Ga***,45*******,18007\n2,Ma*** Ro***,74*******,18009\n3,An*** Ló***,26*******,18001\n",
        )
        .expect("un CSV bien escrito debería cargarse")
    }

    fn codigos_18007_18009_y_18009_18001_contiguos() -> CodigosPostales {
        let mut codigos = CodigosPostales::new();
        codigos.agregar_contiguos("18007", "18009");
        codigos.agregar_contiguos("18009", "18001");
        codigos
    }

    fn agenda_con_aviso_a_las_y_coche_en(aviso: SystemTime, codigo_postal: &str) -> Agenda {
        let mut agenda = Agenda::new();
        agenda.agregar_clase(hora(17, 0), Clase::new(3));
        agenda.registrar_aviso(hora(17, 0), aviso).unwrap();
        agenda.registrar_ubicacion_coche(codigo_postal.to_string(), aviso);
        agenda
    }

    #[test]
    fn con_aviso_a_las_1610_se_llega_desde_el_coche_al_mismo_codigo_y_a_los_contiguos() {
        let agenda = agenda_con_aviso_a_las_y_coche_en(hora(16, 10), "18007");

        assert_eq!(
            agenda.alumnos_alcanzables(
                hora(17, 0),
                &alumnos_en_18007_18009_y_18001(),
                &codigos_18007_18009_y_18009_18001_contiguos()
            ),
            Ok(HashSet::from([1, 2])),
            "con 50 minutos de margen y el coche en 18007 se llega a 18007 y a su contiguo 18009"
        );
    }

    #[test]
    fn con_aviso_a_las_1655_solo_se_llega_al_codigo_del_coche() {
        let agenda = agenda_con_aviso_a_las_y_coche_en(hora(16, 55), "18007");

        assert_eq!(
            agenda.alumnos_alcanzables(
                hora(17, 0),
                &alumnos_en_18007_18009_y_18001(),
                &codigos_18007_18009_y_18009_18001_contiguos()
            ),
            Ok(HashSet::from([1])),
            "con 5 minutos de margen no da tiempo a ir a un código contiguo"
        );
    }

    #[test]
    fn sin_saber_donde_esta_el_coche_no_se_puede_buscar_sustituto() {
        let mut agenda = agenda_con_clase_a_las_17();
        agenda.registrar_aviso(hora(17, 0), hora(16, 10)).unwrap();

        assert_eq!(
            agenda.alumnos_alcanzables(
                hora(17, 0),
                &alumnos_en_18007_18009_y_18001(),
                &codigos_18007_18009_y_18009_18001_contiguos()
            ),
            Err(ErrorBusquedaSustituto::SinUbicacionCoche),
            "sin la ubicación del coche no se sabe desde dónde se sale"
        );
    }

    #[test]
    fn sin_margen_no_se_puede_buscar_sustituto() {
        let mut agenda = agenda_con_clase_a_las_17();
        agenda.registrar_ubicacion_coche("18007".to_string(), hora(16, 10));

        assert_eq!(
            agenda.alumnos_alcanzables(
                hora(17, 0),
                &alumnos_en_18007_18009_y_18001(),
                &codigos_18007_18009_y_18009_18001_contiguos()
            ),
            Err(ErrorBusquedaSustituto::Margen(ErrorMargen::SinAviso)),
            "una clase sin aviso de cancelación no tiene margen con el que buscar"
        );
    }

    #[test]
    fn el_alumno_que_cancela_no_sale_entre_los_alcanzables() {
        let agenda = agenda_con_aviso_a_las_y_coche_en(hora(16, 10), "18001");

        assert_eq!(
            agenda.alumnos_alcanzables(
                hora(17, 0),
                &alumnos_en_18007_18009_y_18001(),
                &codigos_18007_18009_y_18009_18001_contiguos()
            ),
            Ok(HashSet::from([2])),
            "el alumno 3 está en 18001 como el coche, pero es el que ha cancelado la clase"
        );
    }
}
