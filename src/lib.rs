/// NearGas
///
/// A `NearGas` type represents a amount Gas.
///
/// `NearGas` is composed of a `u64` whole numbers of Gas. 
/// 
/// You can convert decimail number into `NearGas` only from `&str` type.
///
/// # Basic usege:
/// ```
/// use near_token::*;
/// use core::str::FromStr;
///
/// let data = "123.4566 teragas";
/// let gas: NearGas = FromStr::from_str(data).unwrap();
/// assert_eq!(gas.as_gas(), 123456600000000u64);
/// ```
///
///
///

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NearGas {
    inner: u64,
}

use self::utils::parse_str;
/// Tera gas constant is equal to 10^12 gas.
/// 
/// # Example 
/// ```
/// use near_token::*;
/// let teragas: u64 =  2;
/// let gas: u64 = teragas * ONE_TERA_GAS;
/// assert_eq!(gas, 2_0000_0000_0000u64);
/// ```
pub const ONE_TERA_GAS: u64 = 10u64.pow(12);

/// Giga gas constant is equal to 10^6 gas.
/// 
/// # Example
/// ```
/// use near_token::*;
/// let gigagas = 2;
/// let gas = gigagas * ONE_GIGA_GAS;
/// assert_eq!(gas, 2_000_000_000u64);
/// ```
pub const ONE_GIGA_GAS: u64 = 10u64.pow(9);


impl std::str::FromStr for NearGas {
    type Err = NearGasError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let upcase = s.trim().to_ascii_uppercase();
        let (num, currency) = upcase.split_at(
            s.find(|c: char| c.is_ascii_alphabetic())
                .ok_or_else(|| NearGasError::IncorectCurrency(s.to_owned()))?,
        );
        let number = match currency {
            "TGAS" | "TERAGAS" => parse_str(num.trim(), ONE_TERA_GAS).map_err(|error| NearGasError::ParcingError(error))?,
            "GIGAGAS" | "GGAS" => parse_str(num.trim(), ONE_GIGA_GAS).map_err(|error| NearGasError::ParcingError(error))?,
            _ => return Err(NearGasError::IncorectCurrency(s.to_owned())),
        };
        let gas = NearGas::from_gas(number);
        Ok(gas)
    }
}

impl NearGas {
    pub fn from_tgas(mut inner: u64) -> Self {
        inner *= ONE_TERA_GAS;
        Self { inner }
    }

    pub fn from_ggas(mut inner: u64) -> Self {
        inner *= ONE_GIGA_GAS;
        Self { inner }
    }

    pub fn from_gas(inner: u64) -> Self {
        Self { inner }
    }

    pub fn as_gas(self) -> u64 {
        self.inner
    }

    pub fn as_ggas(self) -> u64 {
        self.inner / ONE_GIGA_GAS
    }

    pub fn as_tgas(self) -> u64 {
        self.inner / ONE_TERA_GAS
    }
}

pub mod utils {

    pub fn parse_str(s: &str, pref_const: u64) -> Result<u64, NearGasParsingError> {
        let (int, fract) = if let Some((whole, fractional)) = s.trim().split_once('.') {
            let int: u64 = whole
                .parse()
                .map_err(|_| NearGasParsingError::InvalidNumber(s.to_owned()))?;
            let mut fract: u64 = fractional
                .parse()
                .map_err(|_| NearGasParsingError::InvalidNumber(s.to_owned()))?;
            let len = u32::try_from(fractional.len())
                .map_err(|_| NearGasParsingError::InvalidNumber(s.to_owned()))?;
            fract = fract
                .checked_mul(
                    pref_const
                        .checked_div(10u64.pow(len))
                        .ok_or_else(|| NearGasParsingError::LongFractional(fract.to_owned()))?,
                )
                .ok_or_else(|| NearGasParsingError::LongFractional(fract.to_owned()))?;
            (int, fract)
        } else {
            let int: u64 = s
                .parse()
                .map_err(|_| NearGasParsingError::InvalidNumber(s.to_owned()))?;
            (int, 0)
        };
        let result = fract
            .checked_add(
                int.checked_mul(pref_const)
                    .ok_or_else(|| NearGasParsingError::LongWhole(int.to_owned()))?,
            )
            .ok_or_else(|| NearGasParsingError::LongWhole(int.to_owned()))?;
        Ok(result)
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum NearGasParsingError {
        InvalidNumber(String),
        LongWhole(u64),
        LongFractional(u64),
    }
}

    #[derive(Debug, Clone, PartialEq, Eq)]
pub enum NearGasError {
    ParcingError(utils::NearGasParsingError),
    IncorectCurrency(String),
}

#[cfg(test)]
mod test {
    use super::utils::NearGasParsingError;
    use super::*;
    const TEST: [(u64, &'static str, u64); 6] = [
        (129380_000_001u64, "129.380000001", ONE_GIGA_GAS),
        (12938_000_000_100_000_000u64, "12938000000.1", ONE_GIGA_GAS),
        (129380_000_001u64, "0.129380000001", ONE_TERA_GAS),
        (129380_000_001_000u64, "129.380000001000", ONE_TERA_GAS),
        (9488129380_000_001u64, "9488.129380000001", ONE_TERA_GAS),
        (129380_000_001u64, "00.129380000001", ONE_TERA_GAS),
    ];
    #[test]
    fn parse_test() {
        for test in TEST {
            let test_data = test.0;
            let gas = parse_str(test.1, test.2).unwrap();
            assert_eq!(test_data, gas)
        }
    }
    const TEST_DATA: [&'static str; 6] = [
        "1.1.1 TeraGas",
        "1. 0 TeraGas",
        "0.5 TGas",
        "0 pas",
        "0",
        "-1 TeraGas",
    ];

    use std::str::FromStr;

    #[test]
    fn doubledot() {
        let data = TEST_DATA[0];
        let gas: Result<NearGas, NearGasError> = FromStr::from_str(data);
        assert_eq!(
            gas,
            Err(NearGasError::ParcingError(NearGasParsingError::InvalidNumber("1.1.1".to_owned())))
        )
    }

    #[test]
    fn space_after_dot() {
        let data = TEST_DATA[1];
        let gas: Result<NearGas, NearGasError> = FromStr::from_str(data);
        assert_eq!(
            gas,
            Err(NearGasError::ParcingError(NearGasParsingError::InvalidNumber("1. 0".to_owned())))
        )
    }

    #[test]
    fn decimal_tgas() {
        let data = TEST_DATA[2];
        let gas: Result<NearGas, NearGasError> = FromStr::from_str(data);
        assert_eq!(gas, Ok(NearGas::from_ggas(500)))
    }

    #[test]
    fn incorect_currency() {
        let data = TEST_DATA[3];
        let gas: Result<NearGas, NearGasError> = FromStr::from_str(data);
        assert_eq!(
            gas,
            Err(NearGasError::IncorectCurrency(data.to_owned()))
        )
    }

    #[test]
    fn without_currency() {
        let data = TEST_DATA[4];
        let gas: Result<NearGas, NearGasError> = FromStr::from_str(data);
        assert_eq!(
            gas,
            Err(NearGasError::IncorectCurrency("0".to_owned()))
        )
    }

    #[test]
    fn invalid_whole() {
        let data = TEST_DATA[5];
        let gas: Result<NearGas, NearGasError> = FromStr::from_str(data);
        assert_eq!(
            gas,
            Err(NearGasError::ParcingError(NearGasParsingError::InvalidNumber("-1".to_owned())))
        )
    }

    #[test]
    fn parse_errortest() {
        let test_data = "hnim";
        let gas = parse_str(test_data, ONE_GIGA_GAS);
        assert_eq!(
            gas,
            Err(NearGasParsingError::InvalidNumber("hnim".to_string()))
        )
    }
    #[test]
    fn parse_u64_errortest() {
        let test_data = u64::MAX.to_string();
        let gas = parse_str(&test_data, ONE_GIGA_GAS);
        assert_eq!(gas, Err(NearGasParsingError::LongWhole(u64::MAX)));
    }
}
