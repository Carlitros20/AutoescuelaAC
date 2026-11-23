use crate::alumno::Alumno;
use crate::codigos_postales::CodigosPostales;
use crate::error_carga_alumnos::ErrorCargaAlumnos;
use crate::error_codigo_postal::ErrorCodigoPostal;
use std::collections::{HashMap, HashSet};
use std::time::Duration;

pub struct AlumnosRegistrados {
    alumnos: HashMap<u32, Alumno>,
}

impl AlumnosRegistrados {
    pub fn new() -> Self {
        AlumnosRegistrados {
            alumnos: HashMap::new(),
        }
    }

    pub fn desde_csv(csv: &str) -> Result<Self, ErrorCargaAlumnos> {
        let mut registrados = AlumnosRegistrados::new();
        for (indice, fila) in csv.lines().enumerate().skip(1) {
            if fila.trim().is_empty() {
                continue;
            }
            let linea = indice + 1;
            let campos: Vec<&str> = fila.split(',').map(str::trim).collect();
            let [numero, nombre, _dni, codigo_postal] = campos[..] else {
                return Err(ErrorCargaAlumnos::FilaIncompleta(linea));
            };
            if [numero, nombre, codigo_postal]
                .iter()
                .any(|campo| campo.is_empty())
            {
                return Err(ErrorCargaAlumnos::FilaIncompleta(linea));
            }
            let numero: u32 = numero
                .parse()
                .map_err(|_| ErrorCargaAlumnos::NumeroMalEscrito(linea))?;
            if codigo_postal.len() != 5 || !codigo_postal.bytes().all(|c| c.is_ascii_digit()) {
                return Err(ErrorCargaAlumnos::CodigoPostalMalEscrito(linea));
            }
            if registrados.alumnos.contains_key(&numero) {
                return Err(ErrorCargaAlumnos::NumeroRepetido(numero));
            }
            registrados.alumnos.insert(
                numero,
                Alumno::new(nombre.to_string(), codigo_postal.to_string()),
            );
        }
        Ok(registrados)
    }

    pub fn alumno(&self, numero: u32) -> Option<&Alumno> {
        self.alumnos.get(&numero)
    }

    pub fn alcanzables(
        &self,
        codigos: &CodigosPostales,
        origen: &str,
        margen: Duration,
    ) -> Result<HashSet<u32>, ErrorCodigoPostal> {
        let mut alcanzables = HashSet::new();
        for (numero, alumno) in &self.alumnos {
            if alumno.es_alcanzable(codigos, origen, margen)? {
                alcanzables.insert(*numero);
            }
        }
        Ok(alcanzables)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CABECERA: &str = "numero,nombre,dni,codigo_postal\n";

    fn cargar(filas: &str) -> Result<AlumnosRegistrados, ErrorCargaAlumnos> {
        AlumnosRegistrados::desde_csv(&format!("{CABECERA}{filas}"))
    }

    #[test]
    fn cada_alumno_del_csv_queda_registrado_por_su_numero() {
        let alumnos = cargar("1,Lu*** Ga***,45*******,18007\n2,Ma*** Ro***,74*******,18014\n")
            .expect("un CSV bien escrito debería cargarse");

        assert_eq!(
            alumnos.alumno(2),
            Some(&Alumno::new("Ma*** Ro***".to_string(), "18014".to_string())),
            "el alumno 2 debería estar registrado con su nombre y su código postal"
        );
        assert_eq!(
            alumnos.alumno(3),
            None,
            "el alumno 3 no está en el CSV, así que no debería estar registrado"
        );
    }

    #[test]
    fn una_fila_a_la_que_le_falta_un_dato_da_error() {
        assert_eq!(
            cargar("1,Lu*** Ga***,45*******,18007\n2,Ma*** Ro***,74*******,\n").err(),
            Some(ErrorCargaAlumnos::FilaIncompleta(3)),
            "a la fila de la línea 3 le falta el código postal"
        );
        assert_eq!(
            cargar("1,Lu*** Ga***,18007\n").err(),
            Some(ErrorCargaAlumnos::FilaIncompleta(2)),
            "a la fila de la línea 2 le falta una columna"
        );
    }

    #[test]
    fn un_numero_de_alumno_que_no_es_un_numero_da_error() {
        assert_eq!(
            cargar("uno,Lu*** Ga***,45*******,18007\n").err(),
            Some(ErrorCargaAlumnos::NumeroMalEscrito(2)),
            "\"uno\" no es un número de alumno"
        );
    }

    #[test]
    fn un_codigo_postal_mal_escrito_da_error() {
        assert_eq!(
            cargar("1,Lu*** Ga***,45*******,1807\n").err(),
            Some(ErrorCargaAlumnos::CodigoPostalMalEscrito(2)),
            "1807 tiene cuatro cifras en vez de cinco"
        );
        assert_eq!(
            cargar("1,Lu*** Ga***,45*******,18O07\n").err(),
            Some(ErrorCargaAlumnos::CodigoPostalMalEscrito(2)),
            "18O07 lleva una O en vez de un cero"
        );
    }

    #[test]
    fn dos_alumnos_con_el_mismo_numero_dan_error() {
        assert_eq!(
            cargar("1,Lu*** Ga***,45*******,18007\n1,Ma*** Ro***,74*******,18014\n").err(),
            Some(ErrorCargaAlumnos::NumeroRepetido(1)),
            "el número 1 está dos veces, así que no se sabe cuál es el alumno 1"
        );
    }

    fn alumnos_en_18007_18009_y_18001() -> AlumnosRegistrados {
        cargar("1,Lu*** Ga***,45*******,18007\n2,Ma*** Ro***,74*******,18009\n3,An*** Ló***,26*******,18001\n")
            .expect("un CSV bien escrito debería cargarse")
    }

    fn codigos_18007_18009_y_18009_18001_contiguos() -> CodigosPostales {
        let mut codigos = CodigosPostales::new();
        codigos.agregar_contiguos("18007", "18009");
        codigos.agregar_contiguos("18009", "18001");
        codigos
    }

    fn minutos(minutos: u64) -> Duration {
        Duration::from_secs(minutos * 60)
    }

    #[test]
    fn con_15_minutos_se_llega_a_los_alumnos_del_mismo_codigo_y_de_los_contiguos() {
        let alumnos = alumnos_en_18007_18009_y_18001();
        let codigos = codigos_18007_18009_y_18009_18001_contiguos();

        assert_eq!(
            alumnos.alcanzables(&codigos, "18009", minutos(15)),
            Ok(HashSet::from([1, 2, 3])),
            "desde 18009 con 15 minutos se llega a 18009 y a sus contiguos 18007 y 18001"
        );
        assert_eq!(
            alumnos.alcanzables(&codigos, "18007", minutos(15)),
            Ok(HashSet::from([1, 2])),
            "desde 18007 no se llega al alumno de 18001 porque no es contiguo"
        );
    }

    #[test]
    fn con_5_minutos_solo_se_llega_a_los_alumnos_del_mismo_codigo() {
        let alumnos = alumnos_en_18007_18009_y_18001();
        let codigos = codigos_18007_18009_y_18009_18001_contiguos();

        assert_eq!(
            alumnos.alcanzables(&codigos, "18009", minutos(5)),
            Ok(HashSet::from([2])),
            "con 5 minutos no da tiempo a ir a un código contiguo, que son 10 minutos"
        );
    }

    #[test]
    fn con_10_minutos_justos_se_llega_a_los_contiguos() {
        let alumnos = alumnos_en_18007_18009_y_18001();
        let codigos = codigos_18007_18009_y_18009_18001_contiguos();

        assert_eq!(
            alumnos.alcanzables(&codigos, "18007", minutos(10)),
            Ok(HashSet::from([1, 2])),
            "con un margen igual al tiempo de trayecto se llega justo a tiempo"
        );
    }

    #[test]
    fn si_no_hay_alumnos_a_tiempo_no_hay_alcanzables() {
        let alumnos = cargar("3,An*** Ló***,26*******,18001\n").unwrap();
        let codigos = codigos_18007_18009_y_18009_18001_contiguos();

        assert_eq!(
            alumnos.alcanzables(&codigos, "18007", minutos(50)),
            Ok(HashSet::new()),
            "18001 no es contiguo a 18007, así que no se llega aunque sobre margen"
        );
    }

    #[test]
    fn si_el_coche_esta_en_un_codigo_desconocido_da_error() {
        let alumnos = alumnos_en_18007_18009_y_18001();
        let codigos = codigos_18007_18009_y_18009_18001_contiguos();

        assert_eq!(
            alumnos.alcanzables(&codigos, "29001", minutos(15)),
            Err(ErrorCodigoPostal::Desconocido),
            "sin saber qué códigos tiene al lado 29001 no se sabe a quién se llega"
        );
    }

    #[test]
    fn si_un_alumno_esta_en_un_codigo_desconocido_da_error() {
        let alumnos =
            cargar("1,Lu*** Ga***,45*******,18007\n4,Jo*** Me***,75*******,29001\n").unwrap();
        let codigos = codigos_18007_18009_y_18009_18001_contiguos();

        assert_eq!(
            alumnos.alcanzables(&codigos, "18007", minutos(15)),
            Err(ErrorCodigoPostal::Desconocido),
            "sin saber qué códigos tiene al lado 29001 no se sabe si se llega al alumno 4"
        );
    }

    #[test]
    fn el_csv_de_alumnos_del_repositorio_se_carga_entero() {
        let alumnos = AlumnosRegistrados::desde_csv(include_str!("../datos/alumnos.csv"))
            .expect("el CSV de alumnos del repositorio debería estar bien escrito");

        assert_eq!(
            alumnos.alumnos.len(),
            25,
            "el CSV de alumnos tiene 25 filas, así que debería haber 25 alumnos registrados"
        );
    }
}
