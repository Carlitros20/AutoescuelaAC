use crate::error_codigo_postal::ErrorCodigoPostal;
use std::collections::{HashMap, HashSet};
use std::time::Duration;

const TIEMPO_A_UN_CONTIGUO: Duration = Duration::from_secs(10 * 60);

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

    #[test]
    fn en_el_mismo_codigo_postal_no_hay_que_desplazarse() {
        let codigos = codigos_18007_18009_y_18009_18001_contiguos();

        assert_eq!(
            codigos.tiempo_de_trayecto("18009", "18009"),
            Ok(Some(Duration::ZERO)),
            "en el mismo código postal el tiempo de trayecto debería ser 0"
        );
    }

    #[test]
    fn a_un_codigo_postal_contiguo_se_tarda_10_minutos() {
        let codigos = codigos_18007_18009_y_18009_18001_contiguos();

        assert_eq!(
            codigos.tiempo_de_trayecto("18007", "18009"),
            Ok(Some(Duration::from_secs(10 * 60))),
            "a un código postal contiguo se debería tardar 10 minutos"
        );
    }

    #[test]
    fn a_un_codigo_postal_no_contiguo_no_se_llega() {
        let codigos = codigos_18007_18009_y_18009_18001_contiguos();

        assert_eq!(
            codigos.tiempo_de_trayecto("18007", "18001"),
            Ok(None),
            "a un código postal que no es contiguo no se debería llegar"
        );
    }

    #[test]
    fn el_tiempo_de_trayecto_a_un_codigo_postal_desconocido_da_error() {
        let codigos = codigos_18007_18009_y_18009_18001_contiguos();

        assert_eq!(
            codigos.tiempo_de_trayecto("18007", "29001"),
            Err(ErrorCodigoPostal::Desconocido),
            "sin saber qué códigos tiene al lado 29001 no se puede calcular el tiempo"
        );
    }
}
