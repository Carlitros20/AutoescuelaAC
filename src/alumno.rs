use crate::codigos_postales::CodigosPostales;
use crate::error_codigo_postal::ErrorCodigoPostal;
use std::time::Duration;

#[derive(Debug, PartialEq)]
pub struct Alumno {
    nombre: String,
    codigo_postal: String,
}

impl Alumno {
    pub(crate) fn new(nombre: String, codigo_postal: String) -> Self {
        Alumno {
            nombre,
            codigo_postal,
        }
    }

    pub(crate) fn es_alcanzable(
        &self,
        codigos: &CodigosPostales,
        origen: &str,
        margen: Duration,
    ) -> Result<bool, ErrorCodigoPostal> {
        let tiempo = codigos.tiempo_de_trayecto(origen, &self.codigo_postal)?;
        Ok(tiempo.is_some_and(|tiempo| tiempo <= margen))
    }
}
