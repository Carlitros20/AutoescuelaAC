use crate::alumno::Alumno;
use crate::error_carga_alumnos::ErrorCargaAlumnos;
use std::collections::HashMap;

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
}
