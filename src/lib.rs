const ONE_TERA_GAS: u64 = 10u64.pow(12);
const ONE_GIGA_GAS: u64 = 10u64.pow(9);
const GAS: u64 = 1;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NearGas {
    pub inner: u64,
}

impl std::str::FromStr for NearGas {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num = s.trim().trim_end_matches(char::is_alphabetic).trim();
        let currency = s.trim().trim_start_matches(num).trim().to_uppercase();
        let number = match currency.as_str() {
            "T" | "TGAS" | "TERAGAS" => parse_str(num, ONE_TERA_GAS)?,
            "GIGAGAS" | "GGAS" =>  parse_str(num, ONE_GIGA_GAS)?,
            _ => return Err("Near Gas: incorrect currency value entered".to_string()),
        };
        let gas = NearGas::from_gas(number);
        Ok(gas)
    }
}

impl NearGas {
    pub fn from_tgas(mut inner: u64) -> Self {
        inner = inner * ONE_TERA_GAS;
        Self {inner}
    }

    pub fn from_ggas(mut inner: u64) -> Self {
        inner = inner * ONE_GIGA_GAS;
        Self {inner}
    }

    pub fn from_gas(inner: u64) -> Self {
        Self {inner}
    }

    pub fn as_gas(self)-> u64{
        self.inner
    }

    pub fn as_ggas(self)-> u64{
        self.inner / ONE_GIGA_GAS
    }

    pub fn as_tgas(self)-> u64{
        self.inner / ONE_TERA_GAS
    }

}

pub fn parse_str(s: &str, pref_const: u64)-> Result<u64, String>{
    let (int, fract) = if let Some((whole,fractional)) = s.trim().split_once('.'){
        let int: u64  = match whole.parse(){
            Ok(int) => int,
            Err(_) => return Err("Parsing error: Invalid Data".to_string()),
        };
        let mut fract: u64 = match fractional.parse(){
            Ok(fract) => fract,
            Err(_) => return Err("Parsing error: Invalid Data".to_string())
        };
        let len = fractional.len() as u32;
        //fract =fract.checked_mul(pref_const.checked_div(10u64.pow(len)).unwrap()).unwrap();
        fract = if let Some(fract) = 
            fract.checked_mul(if let Some(mantiss) = 
                pref_const.checked_div(10u64.pow(len)){
                    mantiss
                }else{return Err("Parsing Error: Too Long fractional part".to_string());}){
                fract
            } else{return Err("Parsing Error: Too Long fractional part".to_string());};
        (int, fract)
    }else{
        let int: u64  = match s.parse(){
            Ok(int) => int,
            Err(_) => return Err("Parsing error: Invalid Data".to_string()),
        };
        (int, 0)
    };
    //let result = int * pref_const + fract ;
    let result =  if let Some(result ) = 
        fract.checked_add(if let Some(whole) = 
            int.checked_mul(pref_const){
                whole 
            }else{return Err("Parsing Error: Too long whole part".to_string());}){
        result
    }else{return Err("Parsing Error: Too long whole part".to_string());};
    Ok(result)
}

#[cfg(test)]
mod test{
    use super::*;
    const TEST: [(u64, &'static str, u64); 7]=[
        (129380_000_001u64, "129.380000001", ONE_GIGA_GAS),
        (12938_000_000_100_000_000u64, "12938000000.1", ONE_GIGA_GAS),
        (129380_000_001u64, "0.129380000001", ONE_TERA_GAS),
        (129380_000_001_000u64, "129.380000001000", ONE_TERA_GAS),
        (9488129380_000_001u64, "9488.129380000001", ONE_TERA_GAS),
        (129380_000_001u64, "00.129380000001", ONE_TERA_GAS),
        (129380_000_001u64, "129380000001", GAS),
    ];
    #[test]
    fn parse_test1(){
        for test in TEST{
            let test_data = test.0;
            let gas = parse_str(test.1, test.2).unwrap();
            assert_eq!(test_data, gas)
        }
   
    }
}   