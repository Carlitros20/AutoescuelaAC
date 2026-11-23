use crate::error_carga_csv::ErrorCargaCsv;

pub(crate) fn filas_csv<const CAMPOS: usize>(
    csv: &str,
) -> Result<Vec<(usize, [&str; CAMPOS])>, ErrorCargaCsv> {
    let mut filas = Vec::new();
    for (indice, fila) in csv.lines().enumerate().skip(1) {
        if fila.trim().is_empty() {
            continue;
        }
        let linea = indice + 1;
        let campos: Vec<&str> = fila.split(',').map(str::trim).collect();
        let campos: [&str; CAMPOS] = campos
            .try_into()
            .map_err(|_| ErrorCargaCsv::FilaIncompleta(linea))?;
        if campos.iter().any(|campo| campo.is_empty()) {
            return Err(ErrorCargaCsv::FilaIncompleta(linea));
        }
        filas.push((linea, campos));
    }
    Ok(filas)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cada_fila_sale_con_su_linea_y_sin_la_cabecera() {
        assert_eq!(
            filas_csv::<2>("codigo_postal,contiguo\n18007, 18009\n\n18009,18001\n"),
            Ok(vec![(2, ["18007", "18009"]), (4, ["18009", "18001"])]),
            "la cabecera y las líneas vacías no son filas, y los espacios sobran"
        );
    }

    #[test]
    fn una_fila_a_la_que_le_falta_un_dato_da_error() {
        assert_eq!(
            filas_csv::<2>("codigo_postal,contiguo\n18007,18009\n18009,\n"),
            Err(ErrorCargaCsv::FilaIncompleta(3)),
            "a la fila de la línea 3 le falta el segundo dato"
        );
        assert_eq!(
            filas_csv::<2>("codigo_postal,contiguo\n18007\n"),
            Err(ErrorCargaCsv::FilaIncompleta(2)),
            "a la fila de la línea 2 le falta una columna"
        );
    }
}
