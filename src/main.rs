use log::*;

mod modbus_datatypes;
use modbus_datatypes::*;

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
    c1_exp: i32,
    c1_mantissa: i32,
    c1_val: f32,
    c1_x10: f32,
    c1_float: f32,
    c4_exp: i32,
    c4_mantissa: i32,
    c4_val: f32,
    c4_x10: f32,
    c4_float: f32,
}

/// Connect to modbus server and get all measurements
async fn get_measurements(modbus_conn_string: String, device_id: u8) -> Result<Measurement, Box<dyn std::error::Error>> {
    use tokio_modbus::prelude::*;

    let socket_addr = modbus_conn_string.parse().unwrap();
    let mut ctx = tcp::connect(socket_addr).await?;
    ctx.set_slave(Slave(device_id));

    let raw_runtime = ctx.read_input_registers(103, 2).await?;
    let m_runtime = get_t3(raw_runtime.clone());
    debug!("Run time is '{:?}' => '{:?}'", raw_runtime, m_runtime);

    let raw_freq = ctx.read_input_registers(105, 2).await?;
    let m_freq = get_t5(raw_freq.clone());
    debug!("Frequency is '{:?}' => '{:?}'", raw_freq, m_freq);

    let raw_u1 = ctx.read_input_registers(107, 2).await?;
    let m_u1 = get_t5(raw_u1.clone());
    debug!("U1 is '{:?}' => '{:?}'", raw_u1, m_u1);

    let raw_i1 = ctx.read_input_registers(126, 2).await?;
    let m_i1 = get_t5(raw_i1.clone());
    debug!("I1 is '{:?}' => '{:?}'", raw_i1, m_i1);

    let raw_pt = ctx.read_input_registers(140, 2).await?;
    let m_pt = get_t6(raw_pt.clone());
    debug!("Active power total is '{:?}' => '{:?}'", raw_pt, m_pt);

    let raw_qt = ctx.read_input_registers(148, 2).await?;
    let m_qt = get_t6(raw_qt.clone());
    debug!("Reactive power total is '{:?}' => '{:?}'", raw_qt, m_qt);

    let raw_st = ctx.read_input_registers(156, 2).await?;
    let m_st = get_t5(raw_st.clone());
    debug!("Apparent power total is '{:?}' => '{:?}'", raw_st, m_st);

    let raw_pft = ctx.read_input_registers(164, 2).await?;
    let m_pft = get_t7(raw_pft.clone());
    debug!("Power factor total is '{:?}' => '{:?}'", raw_pft, m_pft);

    let raw_temp = ctx.read_input_registers(181, 1).await?;
    let m_temp = get_t17(raw_temp.clone());
    debug!("Internal temperature is '{:?}' => '{:?}'", raw_temp, m_temp);

    let raw_c1_exp = ctx.read_input_registers(401, 1).await?;
    let m_c1_exp = get_t2(raw_c1_exp.clone()) as i32;
    debug!("Energy counter 1 exponent is '{:?}' => '{:?}'", raw_c1_exp, m_c1_exp);

    let raw_c1_mantissa = ctx.read_input_registers(406, 2).await?;
    let m_c1_mantissa = get_t3(raw_c1_mantissa.clone());
    debug!("Energy counter 1 mantissa is '{:?}' => '{:?}'", raw_c1_mantissa, m_c1_mantissa);

    let m_c1_val = (m_c1_mantissa as f32) * (10.0_f32).powf(m_c1_exp as f32);
    debug!("Energy counter 1 coarse value is '{:?}'", m_c1_val);

    let raw_c1_x10 = ctx.read_input_registers(462, 2).await?;
    let m_c1_x10 = get_t3(raw_c1_x10.clone()) as f32 / 10.0;
    debug!("Energy counter 1 fine value is '{:?}' => '{:?}'", raw_c1_x10, m_c1_x10);

    let raw_c1_float = ctx.read_input_registers(2638, 2).await?;
    let m_c1_float = get_float(raw_c1_float.clone());
    debug!("Energy counter 1 float value is '{:?}' => '{:?}'", m_c1_float, raw_c1_float);

    let raw_c4_exp = ctx.read_input_registers(404, 1).await?;
    let m_c4_exp = get_t2(raw_c4_exp.clone()) as i32;
    debug!("Energy counter 4 exponent is '{:?}' => '{:?}'", raw_c4_exp, m_c4_exp);

    let raw_c4_mantissa = ctx.read_input_registers(412, 2).await?;
    let m_c4_mantissa = get_t3(raw_c4_mantissa.clone());
    debug!("Energy counter 4 mantissa is '{:?}' => '{:?}'", raw_c4_mantissa, m_c4_mantissa);

    let m_c4_val = (m_c4_mantissa as f32) * (10.0_f32).powf(m_c4_exp as f32);
    debug!("Energy counter 4 coarse value is '{:?}'", m_c4_val);

    let raw_c4_x10 = ctx.read_input_registers(468, 2).await?;
    let m_c4_x10 = get_t3(raw_c4_x10.clone()) as f32 / 10.0;
    debug!("Energy counter 4 fine value is '{:?}' => '{:?}'", raw_c4_x10, m_c4_x10);

    let raw_c4_float = ctx.read_input_registers(2644, 2).await?;
    let m_c4_float = get_float(raw_c4_float.clone());
    debug!("Energy counter 4 float value is '{:?}' => '{:?}'", m_c4_float, raw_c4_float);

    let measurement = Measurement{
        device_id: device_id as i32,
        device_timestamp: m_runtime,
        frequency: m_freq,
        u1: m_u1,
        i1: m_i1,
        pt: m_pt,
        qt: m_qt,
        st: m_st,
        pft: m_pft,
        temp: m_temp,
        c1_exp: m_c1_exp,
        c1_mantissa: m_c1_mantissa,
        c1_val: m_c1_val,
        c1_x10: m_c1_x10,
        c1_float: m_c1_float,
        c4_exp: m_c4_exp,
        c4_mantissa: m_c4_mantissa,
        c4_val: m_c4_val,
        c4_x10: m_c4_x10,
        c4_float: m_c4_float,
    };

    Ok(measurement)
}

/// write results to psql
async fn write_to_psql(psql_conn_string: String, measurement: Measurement) -> Result<(), Box<dyn std::error::Error>> {
    use tokio_postgres::{NoTls};

    let (client, connection) = tokio_postgres::connect(&psql_conn_string, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            error!("connection error: {}", e);
        }
    });
    client.execute("INSERT INTO energy \
        (device_id, device_timestamp, frequency, U1, I1, \
        Pt, Qt, St, Pft, int_temp, \
        c1_exp, c1_mantissa, c1_val, c1_x10, c1_float,\
        c4_exp, c4_mantissa, c4_val, c4_x10, c4_float) \
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20)",
                   &[&measurement.device_id, &measurement.device_timestamp, &measurement.frequency, &measurement.u1, &measurement.i1,
                       &measurement.pt, &measurement.qt, &measurement.st, &measurement.pft, &measurement.temp,
                       &measurement.c1_exp, &measurement.c1_mantissa, &measurement.c1_val, &measurement.c1_x10, &measurement.c1_float,
                       &measurement.c4_exp, &measurement.c4_mantissa, &measurement.c4_val, &measurement.c4_x10, &measurement.c4_float])
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
    stderrlog::new().module(module_path!()).verbosity(log_level).init().unwrap();

    let modbus_addr: String = settings.get_string("modbus_server").unwrap();
    let modbus_device_id: u8 = settings.get_int("modbus_device_id").unwrap() as u8;
    let psql_addr: String = settings.get_string("psql").unwrap();

    let measurement = get_measurements(modbus_addr, modbus_device_id).await.unwrap();
    write_to_psql(psql_addr, measurement).await.unwrap();

    Ok(())
}


