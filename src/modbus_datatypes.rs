/// Conversions of Modbus data types to Rust data types
pub trait ModbusConversions {
    fn get_t1(&self) -> u16;
    fn get_t2(&self) -> i16;
    fn get_t3(&self) -> i32;
    fn get_t5(&self) -> f32;
    fn get_t6(&self) -> f32;
    fn get_t7(&self) -> i32;
    fn get_t16(&self) -> f32;
    fn get_t17(&self) -> f32;
    fn get_float(&self) -> f32;
}

impl ModbusConversions for Vec<u16> {
    /// Returns Unsigned Value (16 bit)
    /// Based on 7M.24 modbus data types
    /// Example: 12345 stored as 12345 = 3039(16)
    fn get_t1(&self) -> u16 {
        assert_eq!(self.len(), 1);
        self[0] as u16
    }

    /// Returns Signed Value (16 bit)
    /// Based on 7M.24 modbus data types
    /// Example: -12345 stored as -12345 = CFC7(16)
    fn get_t2(&self) -> i16 {
        assert_eq!(self.len(), 1);
        self[0] as i16
    }

    /// Returns Signed Long Value (32 bit)
    /// Based on 7M.24 modbus data types
    /// Example: 123456789 stored as 123456789 = 075B CD 15(16)
    fn get_t3(&self) -> i32 {
        assert_eq!(self.len(), 2);

        ((self[0] as u32) << 16 | self[1] as u32) as i32
    }

    /// Returns Unsigned Measurement (32 bit)
    /// Based on 7M.24 modbus data types
    /// bits # 31..24 = Decade Exponent(Signed 8 bit)
    /// bits # 23..00 = Binary Unsigned Value (24 bit)
    /// Example: 123456*10-3 stored as FD01 E240(16)
    fn get_t5(&self) -> f32 {
        assert_eq!(self.len(), 2);
        use ux::u24;

        let exp: i8 = (self[0] >> 8) as i8;
        let val: u24 = u24::from(self[0] as u8) << 16 | u24::from(self[1] as u16);

        ((u32::from(val) as f32) * (10.0_f32).powi(exp as i32)) as f32
    }

    /// Returns Signed Measurement (32 bit)
    /// Based on 7M.24 modbus data types
    /// bits # 31..24 = Decade Exponent (Signed 8 bit)
    /// bits # 23..00 = Binary Signed value (24 bit)
    /// Example: - 123456*10-3 stored as FDFE 1DC0(16)
    fn get_t6(&self) -> f32 {
        assert_eq!(self.len(), 2);
        use ux::i24;

        let exp: i8 = (self[0] >> 8) as i8;
        let val: i24 = i24::from(self[0] as i8) << 16 | i24::from(self[1] as i16);

        ((i32::from(val) as f32) * (10.0_f32).powi(exp as i32)) as f32
    }

    /// Returns Power Factor (32 bit)
    /// Based on 7M.24 modbus data types
    /// bits # 31..24 = Sign: Import/Export (00/FF)
    /// bits # 23..16 = Sign: Inductive/Capacitive (00/FF)
    /// bits # 15..00 = Unsigned Value (16 bit), 4 decimal places
    fn get_t7(&self) -> i32 {
        assert_eq!(self.len(), 2);
        let sign_dir = (if (self[0] >> 8) == 0xFF { -1 } else { 1 }) as i32;
        let _sign_t = self[0] as u8;
        let cap = self[1] as i32;

        sign_dir * cap
    }

    /// Returns Unsigned Value (16 bit), 2 decimal places
    /// Based on 7M.24 modbus data types
    /// Example: 123.45 stored as 123.45 = 3039(16)
    fn get_t16(&self) -> f32 {
        assert_eq!(self.len(), 1);

        (self[0] as f32) / 100.0
    }

    /// Returns Signed Value (16 bit), 2 decimal places
    /// Based on 7M.24 modbus data types
    /// Example: -123.45 stored as -123.45 = CFC7(16)
    fn get_t17(&self) -> f32 {
        assert_eq!(self.len(), 1);

        ((self[0] as i16) as f32) / 100.0
    }

    /// Returns IEEE 754 Floating-Point Single Precision Value (32 bit)
    /// Based on 7M.24 modbus data types
    /// bits # 31 = Sign Bit (1 bit)
    /// bits # 30..23 = Exponent Field (8 bit)
    /// bits # 22..0 = Significand (23 bit)
    /// Example: 123.45 stored as 123.45000 = 42F6 E666(16)
    fn get_float(&self) -> f32 {
        use ieee754::Ieee754;
        assert_eq!(self.len(), 2);

        let sign_bit = (self[0] >> 15) == 1;
        let exponent = ((self[0] << 1) >> 8) as u8;
        let significand = ((self[0] << 9) as u32) << 7 | self[1] as u32;

        f32::recompose_raw(sign_bit, exponent, significand)
    }
}

/// Macro to read a single value from a modbus device
#[macro_export]
macro_rules! read_finder_register {
    ($ctx:ident, $name:expr, $addr:expr, $count:expr, $func:ident) => {{
        let tmp_vec: Vec<u16> = $ctx.read_input_registers($addr, $count).await?;
        let tmp_val = tmp_vec.clone().$func();
        debug!("{} is {:?}: {:?}", $name, tmp_vec, tmp_val);
        tmp_val
    }};
}

/// Macro to read a counter from a modbus device
#[macro_export]
macro_rules! read_finder_counter {
    ($ctx:ident, $name:expr, $addr_exp:expr, $addr_mantissa:expr, $addr_x10:expr, $addr_float:expr) => {{
        let tmp_exp = read_finder_register!(
            $ctx,
            format!("Energy counter {} exponent", $name),
            $addr_exp,
            1,
            get_t2
        ) as i32;
        let tmp_mantissa = read_finder_register!(
            $ctx,
            format!("Energy counter {} mantissa", $name),
            $addr_mantissa,
            2,
            get_t3
        );
        let tmp_x10 = read_finder_register!(
            $ctx,
            format!("Energy counter {} coarse value", $name),
            $addr_x10,
            2,
            get_t3
        ) as f32
            / 10.0;
        let tmp_float = read_finder_register!(
            $ctx,
            format!("Energy counter {} fine value", $name),
            $addr_float,
            2,
            get_float
        );

        let tmp_c_val = (tmp_mantissa as f32) * (10.0_f32).powf(tmp_exp as f32);

        Counter {
            exp: tmp_exp,
            mantissa: tmp_mantissa,
            val: tmp_c_val,
            x10: tmp_x10,
            float: tmp_float,
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_t1() {
        let test_vec: Vec<u16> = vec![0x3039];
        assert_eq!(test_vec.get_t2(), 12345);
    }

    #[test]
    fn test_get_t2() {
        let test_vec: Vec<u16> = vec![0xCFC7];
        assert_eq!(test_vec.get_t2(), -12345);
    }

    #[test]
    fn test_get_t3() {
        let test_vec: Vec<u16> = vec![0x075B, 0xCD15];
        assert_eq!(test_vec.get_t3(), 123456789);
    }

    #[test]
    fn test_get_t5() {
        let test_vec: Vec<u16> = vec![0xFD01, 0xE240];
        assert_eq!(test_vec.get_t5(), 123.45601_f32);
    }

    #[test]
    fn test_get_t6() {
        let test_vec: Vec<u16> = vec![0xFDFE, 0x1DC0];
        assert_eq!(test_vec.get_t6(), -123.45601_f32);
    }

    #[test]
    fn test_get_t16() {
        let test_vec: Vec<u16> = vec![0x3039];
        assert_eq!(test_vec.get_t16(), 123.45);
    }

    #[test]
    fn test_get_t17() {
        let test_vec: Vec<u16> = vec![0xCFC7];
        assert_eq!(test_vec.get_t17(), -123.45);
    }

    #[test]
    fn test_get_float() {
        let test_vec: Vec<u16> = vec![0x42F6, 0xE666];
        assert_eq!(test_vec.get_float(), 123.45);
    }
}
