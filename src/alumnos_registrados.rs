use crate::codigos_postales::{CodigosPostales, esta_bien_escrito};
use crate::error_carga_csv::ErrorCargaCsv;
use crate::error_codigo_postal::ErrorCodigoPostal;
use crate::filas_csv::filas_csv;
use std::collections::{HashMap, HashSet};
use std::time::Duration;

pub struct AlumnosRegistrados {
    codigos_postales: HashMap<u32, String>,
}

impl AlumnosRegistrados {
    pub fn new() -> Self {
        AlumnosRegistrados {
            codigos_postales: HashMap::new(),
        }
    }

    pub fn desde_csv(csv: &str) -> Result<Self, ErrorCargaCsv> {
        let mut registrados = AlumnosRegistrados::new();
        for (linea, [numero, _nombre, _dni, codigo_postal]) in filas_csv(csv)? {
            let numero: u32 = numero
                .parse()
                .map_err(|_| ErrorCargaCsv::NumeroMalEscrito(linea))?;
            if !esta_bien_escrito(codigo_postal) {
                return Err(ErrorCargaCsv::CodigoPostalMalEscrito(linea));
            }
            if registrados.codigos_postales.contains_key(&numero) {
                return Err(ErrorCargaCsv::NumeroRepetido(numero));
            }
            registrados
                .codigos_postales
                .insert(numero, codigo_postal.to_string());
        }
        Ok(registrados)
    }

    pub fn esta_registrado(&self, numero: u32) -> bool {
        self.codigos_postales.contains_key(&numero)
    }

    pub fn alcanzables(
        &self,
        codigos: &CodigosPostales,
        origen: &str,
        margen: Duration,
    ) -> Result<HashSet<u32>, ErrorCodigoPostal> {
        let mut alcanzables = HashSet::new();
        for (numero, codigo_postal) in &self.codigos_postales {
            let tiempo = codigos.tiempo_de_trayecto(origen, codigo_postal)?;
            if tiempo.is_some_and(|tiempo| tiempo <= margen) {
                alcanzables.insert(*numero);
            }
        }
        Ok(alcanzables)
    }
}
