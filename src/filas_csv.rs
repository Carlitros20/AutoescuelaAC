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
