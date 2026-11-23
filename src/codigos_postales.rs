use crate::error_carga_csv::ErrorCargaCsv;
use crate::error_codigo_postal::ErrorCodigoPostal;
use crate::filas_csv::filas_csv;
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

    pub fn desde_csv(csv: &str) -> Result<Self, ErrorCargaCsv> {
        let mut codigos = CodigosPostales::new();
        for (linea, [uno, otro]) in filas_csv(csv)? {
            for codigo in [uno, otro] {
                if codigo.len() != 5 || !codigo.bytes().all(|c| c.is_ascii_digit()) {
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

    const CABECERA: &str = "codigo_postal,contiguo\n";

    fn cargar(filas: &str) -> Result<CodigosPostales, ErrorCargaCsv> {
        CodigosPostales::desde_csv(&format!("{CABECERA}{filas}"))
    }

    #[test]
    fn cada_pareja_del_csv_queda_registrada_como_contigua() {
        let codigos =
            cargar("18007,18009\n18009,18001\n").expect("un CSV bien escrito debería cargarse");

        assert_eq!(
            codigos.son_contiguos("18001", "18009"),
            Ok(true),
            "18009 y 18001 están en el CSV como contiguos"
        );
        assert_eq!(
            codigos.son_contiguos("18007", "18001"),
            Ok(false),
            "18007 y 18001 no están en el CSV como contiguos"
        );
    }

    #[test]
    fn un_codigo_postal_mal_escrito_en_el_csv_de_contiguos_da_error() {
        assert_eq!(
            cargar("18007,1809\n").err(),
            Some(ErrorCargaCsv::CodigoPostalMalEscrito(2)),
            "1809 tiene cuatro cifras en vez de cinco"
        );
        assert_eq!(
            cargar("18O07,18009\n").err(),
            Some(ErrorCargaCsv::CodigoPostalMalEscrito(2)),
            "18O07 lleva una O en vez de un cero"
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

    #[test]
    fn el_csv_de_contiguos_del_repositorio_se_carga_entero() {
        let codigos = CodigosPostales::desde_csv(include_str!("../datos/codigos_contiguos.csv"))
            .expect("el CSV de contiguos del repositorio debería estar bien escrito");

        assert_eq!(
            codigos.contiguos.len(),
            16,
            "el CSV de contiguos tiene los 16 códigos postales de Granada capital"
        );
        assert_eq!(
            codigos.son_contiguos("18016", "18007"),
            Ok(true),
            "18016 y 18007 se tocan según el CSV de contiguos"
        );
    }
}
