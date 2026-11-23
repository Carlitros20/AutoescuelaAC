use crate::alumnos_registrados::AlumnosRegistrados;
use crate::codigos_postales::CodigosPostales;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub(crate) fn hora(horas: u64, minutos: u64) -> SystemTime {
    UNIX_EPOCH + Duration::from_secs(horas * 3600 + minutos * 60)
}

pub(crate) fn minutos(minutos: u64) -> Duration {
    Duration::from_secs(minutos * 60)
}

pub(crate) fn codigos_18007_18009_y_18009_18001_contiguos() -> CodigosPostales {
    let mut codigos = CodigosPostales::new();
    codigos.agregar_contiguos("18007", "18009");
    codigos.agregar_contiguos("18009", "18001");
    codigos
}

pub(crate) fn alumnos_en_18007_18009_y_18001() -> AlumnosRegistrados {
    AlumnosRegistrados::desde_csv(
        "numero,nombre,dni,codigo_postal\n1,Lu*** Ga***,45*******,18007\n2,Ma*** Ro***,74*******,18009\n3,An*** Ló***,26*******,18001\n",
    )
    .expect("un CSV bien escrito debería cargarse")
}
