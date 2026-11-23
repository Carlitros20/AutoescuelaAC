use crate::error_codigo_postal::ErrorCodigoPostal;
use std::collections::{HashMap, HashSet};

pub struct CodigosPostales {
    contiguos: HashMap<String, HashSet<String>>,
}

impl CodigosPostales {
    pub fn new() -> Self {
        CodigosPostales {
            contiguos: HashMap::new(),
        }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn codigos_18007_18009_y_18009_18001_contiguos() -> CodigosPostales {
        let mut codigos = CodigosPostales::new();
        codigos.agregar_contiguos("18007", "18009");
        codigos.agregar_contiguos("18009", "18001");
        codigos
    }

    #[test]
    fn dos_codigos_agregados_como_contiguos_lo_son_en_los_dos_sentidos() {
        let codigos = codigos_18007_18009_y_18009_18001_contiguos();

        assert_eq!(
            codigos.son_contiguos("18007", "18009"),
            Ok(true),
            "18007 debería ser contiguo a 18009"
        );
        assert_eq!(
            codigos.son_contiguos("18009", "18007"),
            Ok(true),
            "18009 debería ser contiguo a 18007"
        );
    }

    #[test]
    fn dos_codigos_conocidos_que_no_se_tocan_no_son_contiguos() {
        let codigos = codigos_18007_18009_y_18009_18001_contiguos();

        assert_eq!(
            codigos.son_contiguos("18007", "18001"),
            Ok(false),
            "18007 y 18001 no se han agregado como contiguos"
        );
    }

    #[test]
    fn un_codigo_desconocido_da_error() {
        let codigos = codigos_18007_18009_y_18009_18001_contiguos();

        assert_eq!(
            codigos.son_contiguos("18007", "29001"),
            Err(ErrorCodigoPostal::Desconocido),
            "29001 no está en los datos, así que no se puede saber si es contiguo"
        );
        assert_eq!(
            codigos.son_contiguos("29001", "18007"),
            Err(ErrorCodigoPostal::Desconocido),
            "29001 no está en los datos, así que no se puede saber si es contiguo"
        );
    }
}
