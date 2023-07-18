//! A `NearGas` type to represent a value of Gas.
//!
//! Each `NearGas` is composed of a whole number of Gases.
//! `NearGas` is implementing the common trait `FromStr`. Also, have utils function to parse from `str` into `u64`.
//!
//! # Examples
//! ```
//! use near_gas::*;
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
use std::u64;
pub use utils::*;

const ONE_TERA_GAS: u64 = 10u64.pow(12);
const ONE_GIGA_GAS: u64 = 10u64.pow(9);

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
    /// use near_gas::*;
    ///
    /// let tera_gas = NearGas::from_tgas(5);
    ///
    /// assert_eq!(tera_gas.as_gas(), 5 * ONE_TERA_GAS);
    /// ```    
    pub const fn from_tgas(mut inner: u64) -> Self {
        inner *= ONE_TERA_GAS;
        Self { inner }
    }

    /// Creates a new `NearGas` from the specified number of whole giga Gas.
    ///
    /// # Examples
    /// ```
    /// use near_gas::*;
    ///
    /// let giga_gas = NearGas::from_ggas(5);
    ///
    /// assert_eq!(giga_gas.as_gas(), 5 * ONE_GIGA_GAS);
    /// ```    
    pub const fn from_ggas(mut inner: u64) -> Self {
        inner *= ONE_GIGA_GAS;
        Self { inner }
    }

    /// Creates a new `NearGas` from the specified number of whole Gas.
    ///
    /// # Examples
    /// ```
    /// use near_gas::*;
    ///
    /// let gas = NearGas::from_gas(5 * ONE_TERA_GAS);
    ///
    /// assert_eq!(gas.as_tgas(), 5);
    /// ```    
    pub const fn from_gas(inner: u64) -> Self {
        Self { inner }
    }

    /// Returns the total number of whole Gas contained by this `NearGas`.
    ///
    /// # Examples
    /// ```
    /// use near_gas::*;
    /// let neargas = NearGas::from_gas(12345);
    /// assert_eq!(neargas.as_gas(), 12345);
    /// ```
    pub const fn as_gas(self) -> u64 {
        self.inner
    }

    /// Returns the total number of a whole part of giga Gas contained by this `NearGas`.
    ///
    /// # Examples
    /// ```
    /// use near_gas::*;
    /// let neargas = NearGas::from_gas(1 * ONE_GIGA_GAS);
    /// assert_eq!(neargas.as_ggas(), 1);
    /// ```
    pub const fn as_ggas(self) -> u64 {
        self.inner / ONE_GIGA_GAS
    }

    /// Returns the total number of a whole part of tera Gas contained by this `NearGas`.
    ///
    /// # Examples
    /// ```
    /// use near_gas::*;
    /// let neargas = NearGas::from_gas(1 * ONE_TERA_GAS);
    /// assert_eq!(neargas.as_tgas(), 1);
    /// ```
    pub const fn as_tgas(self) -> u64 {
        self.inner / ONE_TERA_GAS
    }

    /// Checked integer addition. Computes self + rhs, returning None if overflow occurred.
    ///
    /// # Examples
    /// ```
    /// use near_gas::*;
    /// use std::u64;
    /// assert_eq!(NearGas::from_gas(u64::MAX -2).checked_add(NearGas::from_gas(2)), Some(NearGas::from_gas(u64::MAX)));
    /// assert_eq!(NearGas::from_gas(u64::MAX -2).checked_add(NearGas::from_gas(3)), None);
    /// ```
    pub fn checked_add(self, rhs: NearGas) -> Option<Self> {
        self.as_gas().checked_add(rhs.as_gas()).map(Self::from_gas)
    }

    /// Checked integer subtraction. Computes self - rhs, returning None if overflow occurred.
    ///
    /// # Examples
    /// ```
    /// use near_gas::*;
    /// assert_eq!(NearGas::from_gas(2).checked_sub(NearGas::from_gas(2)), Some(NearGas::from_gas(0)));
    /// assert_eq!(NearGas::from_gas(2).checked_sub(NearGas::from_gas(3)), None);
    /// ```
    pub fn checked_sub(self, rhs: NearGas) -> Option<Self> {
        self.as_gas().checked_sub(rhs.as_gas()).map(Self::from_gas)
    }

    /// Checked integer multiplication. Computes self * rhs, returning None if overflow occurred.
    ///
    /// # Examples
    /// ```
    /// use near_gas::*;
    /// use std::u64;
    /// assert_eq!(NearGas::from_gas(2).checked_mul(2), Some(NearGas::from_gas(4)));
    /// assert_eq!(NearGas::from_gas(u64::MAX).checked_mul(2), None)
    pub fn checked_mul(self, rhs: u64) -> Option<Self> {
        self.as_gas().checked_mul(rhs).map(Self::from_gas)
    }

    /// Checked integer division. Computes self / rhs, returning None if rhs == 0.
    ///
    /// # Examples
    /// ```
    /// use near_gas::*;
    /// assert_eq!(NearGas::from_gas(10).checked_div(2), Some(NearGas::from_gas(5)));
    /// assert_eq!(NearGas::from_gas(2).checked_div(0), None);
    /// ```
    pub fn checked_div(self, rhs: u64) -> Option<Self> {
        self.as_gas().checked_div(rhs).map(NearGas::from_gas)
    }

    /// Saturating integer addition. Computes self + rhs, saturating at the numeric bounds instead of overflowing.
    ///
    /// # Examples
    /// ```
    /// use near_gas::*;
    /// use std::u64;
    /// assert_eq!(NearGas::from_gas(5).saturating_add(NearGas::from_gas(5)), NearGas::from_gas(10));
    /// assert_eq!(NearGas::from_gas(u64::MAX).saturating_add(NearGas::from_gas(1)), NearGas::from_gas(u64::Max));
    /// ```
    pub fn saturating_add(self, rhs: NearGas) -> NearGas {
        NearGas::from_gas(self.as_gas().saturating_add(rhs.as_gas()))
    }

    /// Saturating integer subtraction. Computes self - rhs, saturating at the numeric bounds instead of overflowing.
    ///
    /// # Examples
    /// ```
    /// use near_gas::*;
    /// assert_eq!(NearGas::from_gas(5).saturating_sub(NearGas::from_gas(2)), NearGas::from_gas(3));
    /// assert_eq!(NearGas::from_gas(1).saturating_sub(NearGas::from_gas(2)), NearGas::from_gas(0));
    /// ```
    pub fn saturating_sub(self, rhs: NearGas) -> NearGas {
        NearGas::from_gas(self.as_gas().saturating_sub(rhs.as_gas()))
    }

    /// Saturating integer multiplication. Computes self * rhs, saturating at the numeric bounds instead of overflowing.
    ///
    /// # Examples
    /// ```
    /// use near_gas::*;
    /// use std::u64;
    /// assert_eq!(NearGas::from_gas(2).saturating_mul(5), NearGas::from_gas(10));
    /// assert_eq!(NearGas::from_gas(u64::MAX).saturating_mul(2), NearGas::from_gas(u64::MAX));
    /// ```
    pub fn saturating_mul(self, rhs: u64) -> NearGas {
        NearGas::from_gas(self.as_gas().saturating_mul(rhs))
    }

    /// Saturating integer division. Computes self / rhs, saturating at the numeric bounds instead of overflowing.
    ///
    /// # Examples
    /// ```
    /// use near_gas::*;
    /// assert_eq!(NearGas::from_gas(10).saturating_div(2), NearGas::from_gas(5));
    /// assert_eq!(NearGas::from_gas(10).saturating_div(0), NearGas::from_gas(0))
    /// ```
    pub fn saturating_div(self, rhs: u64) -> NearGas {
        if rhs == 0 {
            return NearGas::from_gas(0);
        }
        NearGas::from_gas(self.as_gas().saturating_div(rhs))
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

    use std::u64;

    #[test]
    fn add_gas() {
        let gas = NearGas::from_gas(u64::MAX - 3);
        let any_gas = NearGas::from_gas(3);
        let more_gas = NearGas::from_gas(4);
        assert_eq!(
            gas.clone().checked_add(any_gas),
            Some(NearGas::from_gas(u64::MAX))
        );
        assert_eq!(gas.checked_add(more_gas), None);
    }

    #[test]
    fn sub_gas() {
        let gas = NearGas::from_gas(3);
        let any_gas = NearGas::from_gas(1);
        let more_gas = NearGas::from_gas(4);
        assert_eq!(gas.clone().checked_sub(any_gas), Some(NearGas::from_gas(2)));
        assert_eq!(gas.checked_sub(more_gas), None);
    }

    #[test]
    fn mul_gas() {
        let gas = NearGas::from_gas(u64::MAX / 10);
        assert_eq!(
            gas.clone().checked_mul(10),
            Some(NearGas::from_gas(u64::MAX / 10 * 10))
        );
        assert_eq!(gas.checked_mul(11), None);
    }

    #[test]
    fn div_gas() {
        let gas = NearGas::from_gas(10);
        assert_eq!(gas.clone().checked_div(2), Some(NearGas::from_gas(5)));
        assert_eq!(gas.clone().checked_div(11), Some(NearGas::from_gas(0)));
        assert_eq!(gas.checked_div(0), None);
    }

    #[test]
    fn s_add_gas() {
        let gas = NearGas::from_gas(100);
        let added_gas = NearGas::from_gas(1);
        let another_gas = NearGas::from_gas(u64::MAX);
        assert_eq!(
            gas.saturating_add(added_gas.clone()),
            NearGas::from_gas(101)
        );
        assert_eq!(
            another_gas.saturating_add(added_gas),
            NearGas::from_gas(u64::MAX)
        );
    }

    #[test]
    fn s_sub_gas() {
        let gas = NearGas::from_gas(100);
        let rhs_gas = NearGas::from_gas(1);
        let another_gas = NearGas::from_gas(u64::MIN);
        assert_eq!(gas.saturating_sub(rhs_gas.clone()), NearGas::from_gas(99));
        assert_eq!(
            another_gas.saturating_sub(rhs_gas),
            NearGas::from_gas(u64::MIN)
        );
    }

    #[test]
    fn s_mul_gas() {
        let gas = NearGas::from_gas(2);
        let rhs = 10;
        let another_gas = u64::MAX;
        assert_eq!(gas.clone().saturating_mul(rhs), NearGas::from_gas(20));
        assert_eq!(gas.saturating_mul(another_gas), NearGas::from_gas(u64::MAX));
    }

    #[test]
    fn s_div_gas() {
        let gas = NearGas::from_gas(10);
        let rhs = 2;
        let another_gas = 20;
        assert_eq!(gas.clone().saturating_div(rhs), NearGas::from_gas(5));
        assert_eq!(gas.saturating_div(another_gas), NearGas::from_gas(0));
    }
}
