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
//!
//! # Crate features
//!
//! * **borsh** -
//!   When enabled allows `NearGas` to serialized and deserialized by `borsh`.
//!
//! * **serde** -
//!  Implements `serde::Serialize` and `serde::Deserialize` for `NearGas`.
//!
//! * **schemars** -
//!  Implements `schemars::JsonSchema` for `NearGas`.
#[cfg(feature = "borsh")]
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
#[cfg(feature = "serde")]
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[cfg_attr(
    feature = "borsh",
    derive(BorshDeserialize, BorshSerialize, BorshSchema)
)]
#[repr(transparent)]
pub struct NearGas {
    inner: u64,
}
mod utils;
use std::u64;

use std::str::FromStr;
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

impl std::convert::From<String> for NearGas {
    fn from(s: String) -> NearGas {
        NearGas::from_str(&s).unwrap_or(NearGas::from_gas(0))
    }
}

impl std::fmt::Display for NearGas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.as_gas() == 0 {
            return write!(f, "0 Tgas");
        }
        let tgas = self.as_tgas();
        if tgas > 0 {
            write!(f, "{} Tgas", tgas)
        } else {
            let float = self.as_gas() as f64 / ONE_TERA_GAS as f64;
            if float < 0.001 {
                return write!(f, "<0.001 Tgas");
            } else {
                return write!(f, "{:.1$} Tgas", float, 3);
            }
        }
    }
}

impl std::fmt::Display for NearGasError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NearGasError::IncorrectNumber(err) => write!(f, "Incorrect number: {:?}", err),
            NearGasError::IncorrectUnit(err) => write!(f, "Incorrect unit: {}", err),
        }
    }
}

impl NearGas {
    /// Creates a new `NearGas` from the specified number of whole tera Gas.
    ///
    /// # Examples
    /// ```
    /// use near_gas::NearGas;
    ///
    /// let tera_gas = NearGas::from_tgas(5);
    ///
    /// assert_eq!(tera_gas.as_gas(), 5 * 1_000_000_000_000);
    /// ```    
    pub const fn from_tgas(mut inner: u64) -> Self {
        inner *= ONE_TERA_GAS;
        Self { inner }
    }

    /// Creates a new `NearGas` from the specified number of whole giga Gas.
    ///
    /// # Examples
    /// ```
    /// use near_gas::NearGas;
    ///
    /// let giga_gas = NearGas::from_ggas(5);
    ///
    /// assert_eq!(giga_gas.as_gas(), 5 * 1_000_000_000);
    /// ```    
    pub const fn from_ggas(mut inner: u64) -> Self {
        inner *= ONE_GIGA_GAS;
        Self { inner }
    }

    /// Creates a new `NearGas` from the specified number of whole Gas.
    ///
    /// # Examples
    /// ```
    /// use near_gas::NearGas;
    ///
    /// let gas = NearGas::from_gas(5 * 1_000_000_000_000);
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
    /// use near_gas::NearGas;
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
    /// use near_gas::NearGas;
    /// let neargas = NearGas::from_gas(1 * 1_000_000_000);
    /// assert_eq!(neargas.as_ggas(), 1);
    /// ```
    pub const fn as_ggas(self) -> u64 {
        self.inner / ONE_GIGA_GAS
    }

    /// Returns the total number of a whole part of tera Gas contained by this `NearGas`.
    ///
    /// # Examples
    /// ```
    /// use near_gas::NearGas;
    /// let neargas = NearGas::from_gas(1 * 1_000_000_000_000);
    /// assert_eq!(neargas.as_tgas(), 1);
    /// ```
    pub const fn as_tgas(self) -> u64 {
        self.inner / ONE_TERA_GAS
    }

    /// Checked integer addition. Computes self + rhs, returning None if overflow occurred.
    ///
    /// # Examples
    /// ```
    /// use near_gas::NearGas;
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
    /// use near_gas::NearGas;
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
    /// use near_gas::NearGas;
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
    /// use near_gas::NearGas;
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
    /// use near_gas::NearGas;
    /// assert_eq!(NearGas::from_gas(5).saturating_add(NearGas::from_gas(5)), NearGas::from_gas(10));
    /// assert_eq!(NearGas::from_gas(u64::MAX).saturating_add(NearGas::from_gas(1)), NearGas::from_gas(u64::MAX));
    /// ```
    pub fn saturating_add(self, rhs: NearGas) -> NearGas {
        NearGas::from_gas(self.as_gas().saturating_add(rhs.as_gas()))
    }

    /// Saturating integer subtraction. Computes self - rhs, saturating at the numeric bounds instead of overflowing.
    ///
    /// # Examples
    /// ```
    /// use near_gas::NearGas;
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
    /// use near_gas::NearGas;
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
    /// use near_gas::NearGas;
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

#[cfg(feature = "interactive-clap")]
impl interactive_clap::ToCli for NearGas {
    type CliVariant = NearGas;
}

#[cfg(feature = "serde")]
impl Serialize for NearGas {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::Error;
        let mut buf = [0u8; 20];
        let remainder = {
            use std::io::Write;
            let mut w: &mut [u8] = &mut buf;
            write!(w, "{}", self.inner).map_err(|err| {
                Error::custom(format!("Failed to serialize: {}", err.to_string()))
            })?;
            w.len()
        };
        let len = buf.len() - remainder;

        let s = std::str::from_utf8(&buf[..len])
            .map_err(|err| Error::custom(format!("Failed to serialize: {}", err.to_string())))?;
        serializer.serialize_str(s)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for NearGas {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        s.parse::<u64>()
            .map(NearGas::from_gas)
            .map_err(|err| de::Error::custom(err.to_string()))
    }
}

#[cfg(feature = "schemars")]
impl schemars::JsonSchema for NearGas {
    fn is_referenceable() -> bool {
        false
    }

    fn schema_name() -> String {
        String::schema_name()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
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
    #[cfg(feature = "serde")]
    fn json_ser() {
        fn test_json_ser(val: u64) {
            let gas = NearGas::from_gas(val);
            let ser = serde_json::to_string(&gas).unwrap();
            assert_eq!(ser, format!("\"{}\"", val));
            let de: NearGas = serde_json::from_str(&ser).unwrap();
            assert_eq!(de.as_gas(), val);
        }

        test_json_ser(u64::MAX);
        test_json_ser(8);
        test_json_ser(0);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn borsh() {
        fn test_borsh_ser(val: u64, expected_serialized_value: [u8; 8]) {
            use borsh::to_vec;
            let gas = NearGas::from_gas(val);
            let ser = to_vec(&gas).unwrap();
            // println!("{:?}", ser);
            assert_eq!(expected_serialized_value, ser.as_slice());
            let de: NearGas = NearGas::try_from_slice(&ser).unwrap();
            assert_eq!(de.as_gas(), val);
        }

        test_borsh_ser(u64::MAX, [255, 255, 255, 255, 255, 255, 255, 255]);
        test_borsh_ser(8, [8, 0, 0, 0, 0, 0, 0, 0]);
        test_borsh_ser(0, [0, 0, 0, 0, 0, 0, 0, 0]);
    }

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

    #[test]
    fn test_display() {
        assert_eq!(NearGas::from_tgas(17).to_string(), "17 Tgas");
        assert_eq!(NearGas::from_ggas(17).to_string(), "0.017 Tgas");
        assert_eq!(NearGas::from_gas(17).to_string(), "<0.001 Tgas");
        assert_eq!(NearGas::from_gas(0).to_string(), "0 Tgas");
        assert_eq!(NearGas::from_gas(1_000_000_000_017).to_string(), "1 Tgas");
    }
    #[test]
    fn test_from_str_f64_gas_without_int() {
        let near_gas = NearGas::from_str(".055ggas").unwrap_err();
        assert_eq!(
            near_gas,
            NearGasError::IncorrectNumber(DecimalNumberParsingError::InvalidNumber(
                ".055".to_string()
            ))
        );
    }
    #[test]
    fn test_from_str_without_currency() {
        let near_gas = NearGas::from_str("100").unwrap_err();
        assert_eq!(near_gas, NearGasError::IncorrectUnit("100".to_string()));
    }
    #[test]
    fn test_from_str_incorrect_currency() {
        let near_gas = NearGas::from_str("100 UAH").unwrap_err();
        assert_eq!(near_gas, NearGasError::IncorrectUnit("100 UAH".to_string()));
    }
    #[test]
    fn test_from_str_invalid_double_dot() {
        let near_gas = NearGas::from_str("100.55.").unwrap_err();
        assert_eq!(near_gas, NearGasError::IncorrectUnit("100.55.".to_string()));
    }
    #[test]
    fn test_from_str_large_fractional_part() {
        let near_gas = NearGas::from_str("100.1111122222333 ggas").unwrap_err(); // 13 digits after "."
        assert_eq!(
            near_gas,
            NearGasError::IncorrectNumber(DecimalNumberParsingError::LongFractional(
                "1111122222333".to_string()
            ))
        );
    }
    #[test]
    fn test_from_str_large_int_part() {
        let near_gas = NearGas::from_str("200123456789123.0 tgas").unwrap_err();
        assert_eq!(
            near_gas,
            NearGasError::IncorrectNumber(DecimalNumberParsingError::LongWhole(
                "200123456789123".to_string()
            ))
        );
    }
    #[test]
    fn test_from_str_negative_value() {
        let near_gas = NearGas::from_str("-100 ggas").unwrap_err();
        assert_eq!(
            near_gas,
            NearGasError::IncorrectNumber(DecimalNumberParsingError::InvalidNumber(
                "-100".to_string()
            ))
        );
    }
}
