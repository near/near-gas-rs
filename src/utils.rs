/// Parsing data from `&str` type in `u64` with using constant prefix from data of gas.
/// Usege prefixes are `ONE_GIGA_GAS` and `ONE_TERA_GAS`.
/// `parse_str` use `u64` type, and have same max and min values.
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
    #[test]
    fn test_long_fract() {
        let data = "1.23456";
        let prefix = 10000u64;
        assert_eq!(
            parse_decimal_number(data, prefix),
            Err(DecimalNumberParsingError::LongFractional(23456))
        );
    }
}
