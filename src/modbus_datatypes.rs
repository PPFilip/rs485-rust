
/// Returns Unsigned Value (16 bit)
/// Based on 7M.24 modbus data types
/// Example: 12345 stored as 12345 = 3039(16)
pub fn get_t1(vec:Vec<u16>) -> u16 {
    assert_eq!(vec.len(), 1);

    vec[0] as u16
}


/// Returns Signed Value (16 bit)
/// Based on 7M.24 modbus data types
/// Example: -12345 stored as -12345 = CFC7(16)
pub fn get_t2(vec:Vec<u16>) -> i16 {
    assert_eq!(vec.len(), 1);

    vec[0] as i16
}


/// Returns Signed Long Value (32 bit)
/// Based on 7M.24 modbus data types
/// Example: 123456789 stored as 123456789 = 075B CD 15(16)
pub fn get_t3(vec:Vec<u16>) -> i32 {
    assert_eq!(vec.len(), 2);

    ((vec[0] as u32) << 16 | vec[1] as u32) as i32
}


/// Returns Unsigned Measurement (32 bit)
/// Based on 7M.24 modbus data types
/// bits # 31..24 = Decade Exponent(Signed 8 bit)
/// bits # 23..00 = Binary Unsigned Value (24 bit)
/// Example: 123456*10-3 stored as FD01 E240(16)
pub fn get_t5(vec:Vec<u16>) -> f32 {
    assert_eq!(vec.len(), 2);
    use ux::{u24};

    let exp:i8 = (vec[0] >> 8) as i8;
    let val:u24 = u24::from(vec[0] as u8) << 16 | u24::from(vec[1] as u16);

    ((u32::from(val) as f32) * (10.0_f32).powi(exp as i32)) as f32
}


/// Returns Signed Measurement (32 bit)
/// Based on 7M.24 modbus data types
/// bits # 31..24 = Decade Exponent (Signed 8 bit)
/// bits # 23..00 = Binary Signed value (24 bit)
/// Example: - 123456*10-3 stored as FDFE 1DC0(16)
pub fn get_t6(vec:Vec<u16>) -> f32 {
    assert_eq!(vec.len(), 2);
    use ux::{i24};

    let exp:i8 = (vec[0] >> 8) as i8;
    let val:i24 = i24::from(vec[0] as i8) << 16 | i24::from(vec[1] as i16);

    ((i32::from(val) as f32) * (10.0_f32).powi(exp as i32)) as f32
}


/// Returns Power Factor (32 bit)
/// Based on 7M.24 modbus data types
/// bits # 31..24 = Sign: Import/Export (00/FF)
/// bits # 23..16 = Sign: Inductive/Capacitive (00/FF)
/// bits # 15..00 = Unsigned Value (16 bit), 4 decimal places
pub fn get_t7(vec:Vec<u16>) -> i32 {
    assert_eq!(vec.len(), 2);
    let sign_dir = (if (vec[0] >> 8) == 0xFF {-1} else {1}) as i32;
    let _sign_t = vec[0] as u8;
    let cap = vec[1] as i32;

    sign_dir * cap
}


/// Returns Signed Value (16 bit), 2 decimal places
/// Based on 7M.24 modbus data types
/// Example: -123.45 stored as -123.45 = CFC7(16)
pub fn get_t17(vec:Vec<u16>) -> f32 {
    assert_eq!(vec.len(), 1);

    ((vec[0] as i16) as f32) / 100.0
}


/// Returns IEEE 754 Floating-Point Single Precision Value (32 bit)
/// Based on 7M.24 modbus data types
/// bits # 31 = Sign Bit (1 bit)
/// bits # 30..23 = Exponent Field (8 bit)
/// bits # 22..0 = Significand (23 bit)
/// Example: 123.45 stored as 123.45000 = 42F6 E666(16)
pub fn get_float(vec:Vec<u16>) -> f32 {
    use ieee754::Ieee754;
    assert_eq!(vec.len(), 2);

    let sign_bit = (vec[0] >> 15) == 1;
    let exponent = ((vec[0] << 1) >> 8) as u8;
    let significand=((vec[0] << 9) as u32) << 7 | vec[1] as u32;

    f32::recompose_raw(sign_bit, exponent, significand)
}
