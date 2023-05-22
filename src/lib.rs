//! A `NearGas` type to represent a value of Gas.
//!
//! Each `NearGas` is composed of a whole number of Gases.
//! `NearGas` is implementing the common trait `FromStr`. Also, have utils function to parse from `str` into `u64`.
//!
//! # Examples
//! ```
//! use near_token::*;
//!
//! let one_tera_gas = NearGas::from_gas(10u64.pow(12));
//! assert_eq!(one_tera_gas, NearGas::from_tgas(1u64));
//! assert_eq!(one_tera_gas, NearGas::from_ggas(1000u64));
//! ```

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NearGas {
    inner: u64,
}
mod utils;
pub use utils::*;

/// The `u64` constant of one tera Gas(10^12).
///
/// # Examples
/// ```
/// use near_token::*;
///
/// let teragas: u64 =  2;
/// let gas: u64 = teragas * ONE_TERA_GAS;
/// assert_eq!(gas, 2_0000_0000_0000u64);
/// ```
pub const ONE_TERA_GAS: u64 = 10u64.pow(12);

/// The `u64`constsnt of one giga Gas(10^9).
///
/// # Examples
/// ```
/// use near_token::*;
///
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
                .ok_or_else(|| NearGasError::IncorrectUnit(s.to_owned()))?,
        );
        let number = match currency {
            "TGAS" | "TERAGAS" => parse_decimal_number(num.trim(), ONE_TERA_GAS)
                .map_err(NearGasError::IncorrectNumber)?,
            "GIGAGAS" | "GGAS" => parse_decimal_number(num.trim(), ONE_GIGA_GAS)
                .map_err(NearGasError::IncorrectNumber)?,
            _ => return Err(NearGasError::IncorrectUnit(s.to_owned())),
        };
        let gas = NearGas::from_gas(number);
        Ok(gas)
    }
}

impl NearGas {
    /// Creates a new `NearGas` from the specified number of whole tera Gas.
    ///
    /// # Examples
    /// ```
    /// use near_token::*;
    ///
    /// let tera_gas = NearGas::from_tgas(5);
    ///
    /// assert_eq!(tera_gas.as_gas(), 5 * ONE_TERA_GAS);
    /// ```    
    pub fn from_tgas(mut inner: u64) -> Self {
        inner *= ONE_TERA_GAS;
        Self { inner }
    }

    /// Creates a new `NearGas` from the specified number of whole giga Gas.
    ///
    /// # Examples
    /// ```
    /// use near_token::*;
    ///
    /// let giga_gas = NearGas::from_ggas(5);
    ///
    /// assert_eq!(giga_gas.as_gas(), 5 * ONE_GIGA_GAS);
    /// ```    
    pub fn from_ggas(mut inner: u64) -> Self {
        inner *= ONE_GIGA_GAS;
        Self { inner }
    }

    /// Creates a new `NearGas` from the specified number of whole Gas.
    ///
    /// # Examples
    /// ```
    /// use near_token::*;
    ///
    /// let gas = NearGas::from_gas(5 * ONE_TERA_GAS);
    ///
    /// assert_eq!(gas.as_tgas(), 5);
    /// ```    
    pub fn from_gas(inner: u64) -> Self {
        Self { inner }
    }

    /// Returns the total number of whole Gas contained by this `NearGas`.
    ///
    /// # Examples
    /// ```
    /// use near_token::*;
    /// let neargas = NearGas::from_gas(12345);
    /// assert_eq!(neargas.as_gas(), 12345);
    /// ```
    pub fn as_gas(self) -> u64 {
        self.inner
    }

    /// Returns the total number of a whole part of giga Gas contained by this `NearGas`.
    ///
    /// # Examples
    /// ```
    /// use near_token::*;
    /// let neargas = NearGas::from_gas(1 * ONE_GIGA_GAS);
    /// assert_eq!(neargas.as_ggas(), 1);
    /// ```
    pub fn as_ggas(self) -> u64 {
        self.inner / ONE_GIGA_GAS
    }

    /// Returns the total number of a whole part of tera Gas contained by this `NearGas`.
    ///
    /// # Examples
    /// ```
    /// use near_token::*;
    /// let neargas = NearGas::from_gas(1 * ONE_TERA_GAS);
    /// assert_eq!(neargas.as_tgas(), 1);
    /// ```
    pub fn as_tgas(self) -> u64 {
        self.inner / ONE_TERA_GAS
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NearGasError {
    IncorrectNumber(utils::DecimalNumberParsingError),
    IncorrectUnit(String),
}

#[cfg(test)]
mod test {
    use super::utils::DecimalNumberParsingError;
    use super::*;
    use std::str::FromStr;

    #[test]
    fn doubledot() {
        let data = "1.1.1 TeraGas";
        let gas: Result<NearGas, NearGasError> = FromStr::from_str(data);
        assert_eq!(
            gas,
            Err(NearGasError::IncorrectNumber(
                DecimalNumberParsingError::InvalidNumber("1.1.1".to_owned())
            ))
        )
    }

    #[test]
    fn space_after_dot() {
        let data = "1. 0 TeraGas";
        let gas: Result<NearGas, NearGasError> = FromStr::from_str(data);
        assert_eq!(
            gas,
            Err(NearGasError::IncorrectNumber(
                DecimalNumberParsingError::InvalidNumber("1. 0".to_owned())
            ))
        )
    }

    #[test]
    fn decimal_tgas() {
        let data = "0.5 TGas";
        let gas: Result<NearGas, NearGasError> = FromStr::from_str(data);
        assert_eq!(gas, Ok(NearGas::from_ggas(500)))
    }

    #[test]
    fn incorect_currency() {
        let data = "0 pas";
        let gas: Result<NearGas, NearGasError> = FromStr::from_str(data);
        assert_eq!(gas, Err(NearGasError::IncorrectUnit(data.to_owned())))
    }

    #[test]
    fn without_currency() {
        let data = "0";
        let gas: Result<NearGas, NearGasError> = FromStr::from_str(data);
        assert_eq!(gas, Err(NearGasError::IncorrectUnit("0".to_owned())))
    }

    #[test]
    fn invalid_whole() {
        let data = "-1 TeraGas";
        let gas: Result<NearGas, NearGasError> = FromStr::from_str(data);
        assert_eq!(
            gas,
            Err(NearGasError::IncorrectNumber(
                DecimalNumberParsingError::InvalidNumber("-1".to_owned())
            ))
        )
    }
}
