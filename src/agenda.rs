use crate::alumnos_registrados::AlumnosRegistrados;
use crate::clase::Clase;
use crate::codigos_postales::CodigosPostales;
use crate::error_agregar_clase::ErrorAgregarClase;
use crate::error_busqueda_sustituto::ErrorBusquedaSustituto;
use crate::error_margen::ErrorMargen;
use std::collections::{BTreeMap, HashSet};
use std::time::{Duration, SystemTime};

const DURACION_CLASE: Duration = Duration::from_secs(45 * 60);

pub struct Agenda {
    clases: BTreeMap<SystemTime, Clase>,
    codigo_postal_coche: Option<String>,
    alumnos: AlumnosRegistrados,
    codigos: CodigosPostales,
}

impl Agenda {
    pub fn new(alumnos: AlumnosRegistrados, codigos: CodigosPostales) -> Self {
        Agenda {
            clases: BTreeMap::new(),
            codigo_postal_coche: None,
            alumnos,
            codigos,
        }
    }

    pub fn agregar_clase(
        &mut self,
        inicio: SystemTime,
        numero_alumno: u32,
    ) -> Result<(), ErrorAgregarClase> {
        let clase = Clase::new(numero_alumno);
        if !clase.es_de_un_alumno_registrado(&self.alumnos) {
            return Err(ErrorAgregarClase::AlumnoNoRegistrado);
        }
        let anterior = self.clases.range(..=inicio).next_back();
        if anterior.is_some_and(|(&otra, _)| otra + DURACION_CLASE > inicio) {
            return Err(ErrorAgregarClase::Solapada);
        }
        let siguiente = self.clases.range(inicio..).next();
        if siguiente.is_some_and(|(&otra, _)| inicio + DURACION_CLASE > otra) {
            return Err(ErrorAgregarClase::Solapada);
        }
        self.clases.insert(inicio, clase);
        Ok(())
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

    pub fn registrar_ubicacion_coche(&mut self, codigo_postal: String) {
        self.codigo_postal_coche = Some(codigo_postal);
    }

    pub fn alumnos_alcanzables(
        &self,
        inicio: SystemTime,
    ) -> Result<HashSet<u32>, ErrorBusquedaSustituto> {
        let clase = self
            .clases
            .get(&inicio)
            .ok_or(ErrorMargen::ClaseInexistente)?;
        let margen = clase.margen(inicio)?;
        let codigo_postal_coche = self
            .codigo_postal_coche
            .as_ref()
            .ok_or(ErrorBusquedaSustituto::SinUbicacionCoche)?;
        let mut alcanzables =
            self.alumnos
                .alcanzables(&self.codigos, codigo_postal_coche, margen)?;
        alcanzables.retain(|numero| !clase.es_del_alumno(*numero));
        Ok(alcanzables)
    }
}
