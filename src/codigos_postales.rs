use crate::error_carga_csv::ErrorCargaCsv;
use crate::error_codigo_postal::ErrorCodigoPostal;
use crate::filas_csv::filas_csv;
use std::collections::{HashMap, HashSet};
use std::time::Duration;

const TIEMPO_A_UN_CONTIGUO: Duration = Duration::from_secs(10 * 60);

pub(crate) fn esta_bien_escrito(codigo_postal: &str) -> bool {
    codigo_postal.len() == 5 && codigo_postal.bytes().all(|cifra| cifra.is_ascii_digit())
}

pub struct CodigosPostales {
    contiguos: HashMap<String, HashSet<String>>,
}

impl CodigosPostales {
    pub fn new() -> Self {
        CodigosPostales {
            contiguos: HashMap::new(),
        }
    }

    pub fn desde_csv(csv: &str) -> Result<Self, ErrorCargaCsv> {
        let mut codigos = CodigosPostales::new();
        for (linea, [uno, otro]) in filas_csv(csv)? {
            for codigo in [uno, otro] {
                if !esta_bien_escrito(codigo) {
                    return Err(ErrorCargaCsv::CodigoPostalMalEscrito(linea));
                }
            }
            codigos.agregar_contiguos(uno, otro);
        }
        Ok(codigos)
    }

    pub fn agregar_contiguos(&mut self, uno: &str, otro: &str) {
        self.contiguos
            .entry(uno.to_string())
            .or_default()
            .insert(otro.to_string());
        self.contiguos
            .entry(otro.to_string())
            .or_default()
            .insert(uno.to_string());
    }

    pub fn son_contiguos(&self, uno: &str, otro: &str) -> Result<bool, ErrorCodigoPostal> {
        let contiguos = self
            .contiguos
            .get(uno)
            .ok_or(ErrorCodigoPostal::Desconocido)?;
        if !self.contiguos.contains_key(otro) {
            return Err(ErrorCodigoPostal::Desconocido);
        }
        Ok(contiguos.contains(otro))
    }

    pub fn tiempo_de_trayecto(
        &self,
        origen: &str,
        destino: &str,
    ) -> Result<Option<Duration>, ErrorCodigoPostal> {
        if self.son_contiguos(origen, destino)? {
            Ok(Some(TIEMPO_A_UN_CONTIGUO))
        } else if origen == destino {
            Ok(Some(Duration::ZERO))
        } else {
            Ok(None)
        }
    }
}
