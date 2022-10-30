CREATE TABLE "energy" (
    device_id INTEGER,
    db_timestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW(),
    device_timestamp INTEGER,
    frequency REAL,
    U1 REAL,
    I1 REAL,
    Pt REAL,
    Qt REAL,
    St REAL,
    Pft INTEGER,
    int_temp REAL,
    c1_exp INTEGER,
    c1_mantissa INTEGER,
    c1_val REAL,
    c1_x10 REAL,
    c1_float REAL,
    c3_exp INTEGER,
    c3_mantissa INTEGER,
    c3_val REAL,
    c3_x10 REAL,
    c3_float REAL
);

SELECT create_hypertable('energy', 'db_timestamp',   chunk_time_interval => INTERVAL '1 day');
