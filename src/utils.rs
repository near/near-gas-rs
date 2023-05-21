/// Parsing decimal number from `&str` type in `u64`.
/// Function also takes value of metric prefix in u64 type.
/// `parse_str` use `u64` type, and have same max and min values.
///
/// If fractional part is longer than number of zero in prefix, it will return error `DecimalNumberParsingError::LongFractional`.
///
/// If string slise have invalid chars, it will return error `DecimalNumberParsingError::InvalidNumber`.
///
/// If whole part of number have value more than `u64` maximum value, it will return error `DecimalNumberParsingError::LongWhole`.
///  
/// # Examples
/// ```
/// use near_token::*;
///
/// let number = "2.65790";
/// let prefix = 100000u64;
/// assert_eq!(parse_decimal_number(number, prefix).unwrap(), 265790u64);
/// ```
pub fn parse_decimal_number(s: &str, pref_const: u64) -> Result<u64, DecimalNumberParsingError> {
    let (int, fract) = if let Some((whole, fractional)) = s.trim().split_once('.') {
        let int: u64 = whole
            .parse()
            .map_err(|_| DecimalNumberParsingError::InvalidNumber(s.to_owned()))?;
        let mut fract: u64 = fractional
            .parse()
            .map_err(|_| DecimalNumberParsingError::InvalidNumber(s.to_owned()))?;
        let len = u32::try_from(fractional.len())
            .map_err(|_| DecimalNumberParsingError::InvalidNumber(s.to_owned()))?;
        fract = fract
            .checked_mul(
                pref_const
                    .checked_div(10u64.pow(len))
                    .filter(not_null)
                    .ok_or_else(|| DecimalNumberParsingError::LongFractional(fract.to_owned()))?,
            )
            .ok_or_else(|| DecimalNumberParsingError::LongFractional(fract.to_owned()))?;
        (int, fract)
    } else {
        let int: u64 = s
            .parse()
            .map_err(|_| DecimalNumberParsingError::InvalidNumber(s.to_owned()))?;
        (int, 0)
    };
    let result = fract
        .checked_add(
            int.checked_mul(pref_const)
                .ok_or_else(|| DecimalNumberParsingError::LongWhole(int.to_owned()))?,
        )
        .ok_or_else(|| DecimalNumberParsingError::LongWhole(int.to_owned()))?;
    Ok(result)
}

fn not_null(n: &u64) -> bool {
    n != &0u64
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecimalNumberParsingError {
    InvalidNumber(String),
    LongWhole(u64),
    LongFractional(u64),
}
//mast be chenged also in near_balanse!!!

#[cfg(test)]
mod tests {
    use crate::utils::*;
    use crate::*;

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
            let gas = parse_decimal_number(test.1, test.2).unwrap();
            assert_eq!(test_data, gas)
        }
    }

    #[test]
    fn test_long_fract() {
        let data = "1.23456";
        let prefix = 10000u64;
        assert_eq!(
            parse_decimal_number(data, prefix),
            Err(DecimalNumberParsingError::LongFractional(23456))
        );
    }

    #[test]
    fn invalidnumber_whole() {
        let num = "1h4.7859";
        let prefix: u64 = 10000;
        assert_eq!(
            parse_decimal_number(num, prefix),
            Err(DecimalNumberParsingError::InvalidNumber(
                "1h4.7859".to_owned()
            ))
        );
    }
    #[test]
    fn invalidnumber_fract() {
        let num = "14.785h9";
        let prefix: u64 = 10000;
        assert_eq!(
            parse_decimal_number(num, prefix),
            Err(DecimalNumberParsingError::InvalidNumber(
                "14.785h9".to_owned()
            ))
        );
    }

    #[test]
    fn max_long_fract() {
        let max_data = 10u64.pow(17) + 1;
        let data = "1.".to_string() + max_data.to_string().as_str();
        let prefix = 10u64.pow(17);
        assert_eq!(
            parse_decimal_number(data.as_str(), prefix),
            Err(DecimalNumberParsingError::LongFractional(max_data))
        );
    }

    #[test]
    fn long_whole_test() {
        let data = 10u64.pow(17) + 1;
        let prefix = ONE_TERA_GAS;
        let s = data.to_string() + "." + "1";
        assert_eq!(
            parse_decimal_number(s.as_str(), prefix),
            Err(DecimalNumberParsingError::LongWhole(data))
        );
    }
}
