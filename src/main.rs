use log::*;

mod modbus_datatypes;
use modbus_datatypes::ModbusConversions;

/// Structure to store counter data in types compatible with psql
struct Counter {
    exp: i32,
    mantissa: i32,
    val: f32,
    x10: f32,
    float: f32,
}

/// Structure to store measurements in types compatible with psql
struct Measurement {
    device_id: i32,
    device_timestamp: i32,
    frequency: f32,
    u1: f32,
    i1: f32,
    pt: f32,
    qt: f32,
    st: f32,
    pft: i32,
    temp: f32,
    u1_thd: f32,
    i1_thd: f32,
    c1: Counter,
    c4: Counter,
    x3: Counter,
}

/// Connect to modbus server and get all measurements
async fn get_measurements(
    modbus_conn_string: String,
    device_id: u8,
) -> Result<Measurement, Box<dyn std::error::Error>> {
    use tokio_modbus::prelude::*;

    let socket_addr = modbus_conn_string.parse().unwrap();
    let mut ctx = tcp::connect(socket_addr).await?;
    ctx.set_slave(Slave(device_id));

    let m_runtime = read_finder_register!(ctx, "Run time", 103, 2, get_t3);
    let m_freq = read_finder_register!(ctx, "Frequency", 105, 2, get_t5);
    let m_u1 = read_finder_register!(ctx, "U1", 107, 2, get_t5);
    let m_i1 = read_finder_register!(ctx, "I1", 126, 2, get_t5);
    let m_pt = read_finder_register!(ctx, "Active power total", 140, 2, get_t6);
    let m_qt = read_finder_register!(ctx, "Reactive power total", 148, 2, get_t6);
    let m_st = read_finder_register!(ctx, "Apparent power total", 156, 2, get_t5);
    let m_pft = read_finder_register!(ctx, "Power factor total", 164, 2, get_t7);
    let m_temp = read_finder_register!(ctx, "Internal temperature", 181, 1, get_t17);
    let m_u1_thd = read_finder_register!(ctx, "U1 THD%", 182, 1, get_t17);
    let m_i1_thd = read_finder_register!(ctx, "I1 THD%", 188, 1, get_t17);

    // // C1 (MID certified) - Import Active Energy
    let counter_c1 = read_finder_counter!(ctx, "C1", 401, 406, 462, 2638);

    // C4 (MID Certified) - Export reactive energy
    let counter_c4 = read_finder_counter!(ctx, "C4", 404, 412, 468, 2644);

    // X3 (not certified) - Total Absolute Apparent Energy
    let counter_x3 = read_finder_counter!(ctx, "X3", 448, 418, 474, 2764);

    let measurement = Measurement {
        device_id: device_id as i32,
        device_timestamp: m_runtime,
        frequency: m_freq,
        u1: m_u1,
        i1: m_i1,
        u1_thd: m_u1_thd,
        i1_thd: m_i1_thd,
        pt: m_pt,
        qt: m_qt,
        st: m_st,
        pft: m_pft,
        temp: m_temp,
        c1: counter_c1,
        c4: counter_c4,
        x3: counter_x3,
    };

    Ok(measurement)
}

/// write results to psql
async fn write_to_psql(
    psql_conn_string: String,
    measurement: Measurement,
) -> Result<(), Box<dyn std::error::Error>> {
    use tokio_postgres::NoTls;

    let (client, connection) = tokio_postgres::connect(&psql_conn_string, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            error!("connection error: {}", e);
        }
    });
    client.execute("INSERT INTO energy \
        (device_id, device_timestamp, frequency, U1, I1, \
        Pt, Qt, St, Pft, int_temp, \
        u1_thd, i1_thd, \
        c1_exp, c1_mantissa, c1_val, c1_x10, c1_float,\
        c4_exp, c4_mantissa, c4_val, c4_x10, c4_float,\
        x3_exp, x3_mantissa, x3_val, x3_x10, x3_float) \
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27)",
                   &[&measurement.device_id, &measurement.device_timestamp, &measurement.frequency, &measurement.u1, &measurement.i1,
                       &measurement.pt, &measurement.qt, &measurement.st, &measurement.pft, &measurement.temp,
                       &measurement.u1_thd, &measurement.i1_thd,
                       &measurement.c1.exp, &measurement.c1.mantissa, &measurement.c1.val, &measurement.c1.x10, &measurement.c1.float,
                       &measurement.c4.exp, &measurement.c4.mantissa, &measurement.c4.val, &measurement.c4.x10, &measurement.c4.float,
                       &measurement.x3.exp, &measurement.x3.mantissa, &measurement.x3.val, &measurement.x3.x10, &measurement.x3.float])
        .await.expect("Cannot write into database");

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use config::Config;
    use std::str::FromStr;

    let settings = Config::builder()
        .add_source(config::File::with_name("Settings"))
        .build()
        .unwrap();

    let log_level_str = settings.get_string("log_level").unwrap();
    let log_level = Level::from_str(&log_level_str).unwrap();
    stderrlog::new()
        .module(module_path!())
        .verbosity(log_level)
        .init()
        .unwrap();

    let modbus_addr: String = settings.get_string("modbus_server").unwrap();
    let modbus_device_id: u8 = settings.get_int("modbus_device_id").unwrap() as u8;
    let psql_addr: String = settings.get_string("psql").unwrap();

    let measurement = get_measurements(modbus_addr, modbus_device_id)
        .await
        .unwrap();
    write_to_psql(psql_addr, measurement).await.unwrap();

    Ok(())
}
