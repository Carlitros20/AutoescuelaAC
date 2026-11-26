use autoescuela_ac::agenda::Agenda;
use autoescuela_ac::alumnos_registrados::AlumnosRegistrados;
use autoescuela_ac::codigos_postales::CodigosPostales;
use autoescuela_ac::error_agregar_clase::ErrorAgregarClase;
use autoescuela_ac::error_busqueda_sustituto::ErrorBusquedaSustituto;
use autoescuela_ac::error_carga_csv::ErrorCargaCsv;
use autoescuela_ac::error_codigo_postal::ErrorCodigoPostal;
use autoescuela_ac::error_margen::ErrorMargen;
use std::collections::HashSet;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn hora(horas: u64, minutos: u64) -> SystemTime {
    UNIX_EPOCH + Duration::from_secs(horas * 3600 + minutos * 60)
}

fn agenda_con_clase_del_alumno_1_a_las_17_entre(alumnos_csv: &str) -> Agenda {
    let alumnos =
        AlumnosRegistrados::desde_csv(alumnos_csv).expect("el CSV de alumnos debería cargarse");
    let codigos = CodigosPostales::desde_csv(include_str!("../datos/codigos_contiguos.csv"))
        .expect("el CSV de códigos postales contiguos debería cargarse");
    let mut agenda = Agenda::new(alumnos, codigos);
    agenda
        .agregar_clase(hora(17, 0), 1)
        .expect("el alumno 1 está registrado y la agenda está vacía");
    agenda
}

fn agenda_con_clase_del_alumno_1_a_las_17() -> Agenda {
    agenda_con_clase_del_alumno_1_a_las_17_entre(include_str!("fixtures/alumnos.csv"))
}

fn buscar_sustituto(
    mut agenda: Agenda,
    aviso: SystemTime,
    codigo_postal_coche: &str,
) -> Result<HashSet<u32>, ErrorBusquedaSustituto> {
    agenda
        .registrar_aviso(hora(17, 0), aviso)
        .expect("a las 17:00 hay una clase");
    agenda.registrar_ubicacion_coche(codigo_postal_coche.to_string());
    agenda.alumnos_alcanzables(hora(17, 0))
}

fn alcanzables_con_aviso_a_las_y_coche_en(
    aviso: SystemTime,
    codigo_postal_coche: &str,
) -> Result<HashSet<u32>, ErrorBusquedaSustituto> {
    buscar_sustituto(
        agenda_con_clase_del_alumno_1_a_las_17(),
        aviso,
        codigo_postal_coche,
    )
}

const ALUMNOS_EN_18007_Y_SUS_CONTIGUOS_SALVO_EL_1: [u32; 6] = [17, 7, 23, 4, 25, 16];

#[test]
fn hu001_un_aviso_a_las_1610_para_la_clase_de_las_17_deja_50_minutos_de_margen() {
    let mut agenda = agenda_con_clase_del_alumno_1_a_las_17();
    agenda.registrar_aviso(hora(17, 0), hora(16, 10)).unwrap();

    assert_eq!(
        agenda.margen(hora(17, 0)),
        Ok(Duration::from_secs(50 * 60)),
        "de las 16:10 a las 17:00 hay 50 minutos"
    );
}

#[test]
fn hu001_con_50_minutos_se_llega_al_codigo_del_coche_y_a_sus_contiguos() {
    assert_eq!(
        alcanzables_con_aviso_a_las_y_coche_en(hora(16, 10), "18007"),
        Ok(HashSet::from(ALUMNOS_EN_18007_Y_SUS_CONTIGUOS_SALVO_EL_1)),
        "18006, 18008 y 18016 son contiguos a 18007; el alumno 1 está en 18007, pero es el que cancela"
    );
}

#[test]
fn hu001_con_10_minutos_justos_aun_se_llega_a_los_contiguos() {
    assert_eq!(
        alcanzables_con_aviso_a_las_y_coche_en(hora(16, 50), "18007"),
        Ok(HashSet::from(ALUMNOS_EN_18007_Y_SUS_CONTIGUOS_SALVO_EL_1)),
        "a un código postal contiguo se tardan 10 minutos, justo el margen"
    );
}

#[test]
fn hu001_con_menos_de_10_minutos_solo_se_llega_al_codigo_del_coche() {
    assert_eq!(
        alcanzables_con_aviso_a_las_y_coche_en(hora(16, 51), "18007"),
        Ok(HashSet::from([17])),
        "con 9 minutos no da tiempo a ir a un contiguo, y en 18007 solo queda el alumno 17"
    );
}

#[test]
fn hu001_un_aviso_a_la_hora_de_la_clase_o_despues_no_deja_margen() {
    for aviso in [hora(17, 0), hora(17, 5)] {
        assert_eq!(
            alcanzables_con_aviso_a_las_y_coche_en(aviso, "18007"),
            Err(ErrorBusquedaSustituto::Margen(ErrorMargen::AvisoTardio)),
            "si avisan cuando la clase ya ha empezado no hay tiempo para recoger a nadie"
        );
    }
}

#[test]
fn hu001_sin_aviso_de_cancelacion_no_se_busca_sustituto() {
    let mut agenda = agenda_con_clase_del_alumno_1_a_las_17();
    agenda.registrar_ubicacion_coche("18007".to_string());

    assert_eq!(
        agenda.alumnos_alcanzables(hora(17, 0)),
        Err(ErrorBusquedaSustituto::Margen(ErrorMargen::SinAviso)),
        "la clase de las 17:00 no se ha cancelado"
    );
}

#[test]
fn hu001_sin_saber_donde_esta_el_coche_no_se_busca_sustituto() {
    let mut agenda = agenda_con_clase_del_alumno_1_a_las_17();
    agenda.registrar_aviso(hora(17, 0), hora(16, 10)).unwrap();

    assert_eq!(
        agenda.alumnos_alcanzables(hora(17, 0)),
        Err(ErrorBusquedaSustituto::SinUbicacionCoche),
        "sin la ubicación del coche no se sabe desde dónde se sale"
    );
}

#[test]
fn hu001_con_el_coche_en_un_codigo_postal_desconocido_no_se_busca_sustituto() {
    assert_eq!(
        alcanzables_con_aviso_a_las_y_coche_en(hora(16, 10), "29001"),
        Err(ErrorBusquedaSustituto::CodigoPostal(
            ErrorCodigoPostal::Desconocido
        )),
        "29001 no está entre los códigos postales de Granada capital"
    );
}

#[test]
fn hu001_con_un_alumno_en_un_codigo_postal_desconocido_no_se_busca_sustituto() {
    let agenda = agenda_con_clase_del_alumno_1_a_las_17_entre(
        "numero,nombre,dni,codigo_postal\n1,Lu*** Ga***,45*******,18007\n2,Ma*** Ro***,74*******,18100\n",
    );

    assert_eq!(
        buscar_sustituto(agenda, hora(16, 10), "18007"),
        Err(ErrorBusquedaSustituto::CodigoPostal(
            ErrorCodigoPostal::Desconocido
        )),
        "18100 es de Armilla y no está entre los códigos postales de Granada capital"
    );
}

#[test]
fn hu001_a_una_hora_sin_clase_no_hay_nada_que_cancelar() {
    let mut agenda = agenda_con_clase_del_alumno_1_a_las_17();

    assert_eq!(
        agenda.registrar_aviso(hora(18, 0), hora(17, 10)),
        Err(ErrorMargen::ClaseInexistente),
        "a las 18:00 no hay clase que cancelar"
    );
    assert_eq!(
        agenda.alumnos_alcanzables(hora(18, 0)),
        Err(ErrorBusquedaSustituto::Margen(
            ErrorMargen::ClaseInexistente
        )),
        "a las 18:00 no hay clase para la que buscar sustituto"
    );
}

#[test]
fn hu001_no_se_agenda_una_clase_que_se_pisa_con_otra() {
    let mut agenda = agenda_con_clase_del_alumno_1_a_las_17();

    for (inicio, motivo) in [
        (hora(17, 30), "empieza antes de que acabe la de las 17:00"),
        (hora(16, 30), "acaba después de que empiece la de las 17:00"),
        (hora(17, 0), "empieza a la misma hora que la de las 17:00"),
    ] {
        assert_eq!(
            agenda.agregar_clase(inicio, 2),
            Err(ErrorAgregarClase::Solapada),
            "la clase {motivo}"
        );
    }
    assert_eq!(
        agenda.margen(hora(17, 30)),
        Err(ErrorMargen::ClaseInexistente),
        "la clase solapada no debería quedarse en la agenda"
    );
}

#[test]
fn hu001_se_agenda_una_clase_que_empieza_o_acaba_justo_cuando_otra() {
    let mut agenda = agenda_con_clase_del_alumno_1_a_las_17();

    assert_eq!(
        agenda.agregar_clase(hora(17, 45), 2),
        Ok(()),
        "la clase de las 17:00 dura 45 minutos y acaba a las 17:45"
    );
    assert_eq!(
        agenda.agregar_clase(hora(16, 15), 3),
        Ok(()),
        "una clase a las 16:15 acaba justo a las 17:00"
    );
}

#[test]
fn hu001_no_se_agenda_una_clase_de_un_alumno_no_registrado() {
    let mut agenda = agenda_con_clase_del_alumno_1_a_las_17();

    assert_eq!(
        agenda.agregar_clase(hora(18, 0), 26),
        Err(ErrorAgregarClase::AlumnoNoRegistrado),
        "solo hay 25 alumnos registrados"
    );
    assert_eq!(
        agenda.margen(hora(18, 0)),
        Err(ErrorMargen::ClaseInexistente),
        "la clase de un alumno no registrado no debería quedarse en la agenda"
    );
}

#[test]
fn hu001_no_se_cargan_alumnos_mal_escritos() {
    for (fila, error, motivo) in [
        (
            "1,Lu*** Ga***,45*******",
            ErrorCargaCsv::FilaIncompleta(2),
            "falta el código postal",
        ),
        (
            "1,Lu*** Ga***,,18007",
            ErrorCargaCsv::FilaIncompleta(2),
            "el DNI está vacío",
        ),
        (
            "uno,Lu*** Ga***,45*******,18007",
            ErrorCargaCsv::NumeroMalEscrito(2),
            "el número no es un número",
        ),
        (
            "1,Lu*** Ga***,45*******,18O07",
            ErrorCargaCsv::CodigoPostalMalEscrito(2),
            "el código postal lleva una O en vez de un cero",
        ),
        (
            "1,Lu*** Ga***,45*******,1807",
            ErrorCargaCsv::CodigoPostalMalEscrito(2),
            "el código postal tiene cuatro cifras",
        ),
        (
            "1,Lu*** Ga***,45*******,18007\n1,Ma*** Ro***,74*******,18014",
            ErrorCargaCsv::NumeroRepetido(1),
            "dos alumnos tienen el número 1",
        ),
    ] {
        assert_eq!(
            AlumnosRegistrados::desde_csv(&format!("numero,nombre,dni,codigo_postal\n{fila}\n"))
                .err(),
            Some(error),
            "{motivo}"
        );
    }
}

#[test]
fn hu001_no_se_cargan_codigos_contiguos_mal_escritos() {
    for (fila, error, motivo) in [
        (
            "18007",
            ErrorCargaCsv::FilaIncompleta(2),
            "falta el código contiguo",
        ),
        (
            "18007,1809",
            ErrorCargaCsv::CodigoPostalMalEscrito(2),
            "1809 tiene cuatro cifras",
        ),
    ] {
        assert_eq!(
            CodigosPostales::desde_csv(&format!("codigo_postal,contiguo\n{fila}\n")).err(),
            Some(error),
            "{motivo}"
        );
    }
}
