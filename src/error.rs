#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NearGasError {
    IncorrectNumber(crate::utils::DecimalNumberParsingError),
    IncorrectUnit(String),
}

impl std::error::Error for NearGasError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            NearGasError::IncorrectNumber(err) => Some(err),
            NearGasError::IncorrectUnit(_) => None,
        }
    }
}
