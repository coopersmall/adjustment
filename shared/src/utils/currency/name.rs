use macros::*;

use super::code::CurrencyCode;
use super::symbol::CurrencySymbol;

#[warn(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
#[json_parse]
pub enum CurrencyName {
    US_Dollar,
    Bitcoin,
    Euro,
    British_Pound,
}

impl<'a> CurrencyName {
    pub fn new() -> CurrencyNameBuilder<'a> {
        CurrencyNameBuilder::new()
    }

    pub fn to_string(&self) -> &str {
        match self {
            Self::US_Dollar => "US Dollar",
            Self::Bitcoin => "Bitcoin",
            Self::Euro => "Euro",
            Self::British_Pound => "British Pound",
        }
    }

    pub fn get_symbol(&self) -> CurrencySymbol {
        match self {
            Self::US_Dollar => CurrencySymbol::USD,
            Self::Bitcoin => CurrencySymbol::BTC,
            Self::Euro => CurrencySymbol::EUR,
            Self::British_Pound => CurrencySymbol::GBP,
        }
    }

    pub fn get_code(&self) -> CurrencyCode {
        match self {
            Self::US_Dollar => CurrencyCode::USD,
            Self::Bitcoin => CurrencyCode::BTC,
            Self::Euro => CurrencyCode::EUR,
            Self::British_Pound => CurrencyCode::GBP,
        }
    }
}

impl Default for CurrencyName {
    fn default() -> Self {
        Self::US_Dollar
    }
}

pub struct CurrencyNameBuilder<'a> {
    currency_name: Option<&'a str>,
}

impl<'a> CurrencyNameBuilder<'a> {
    fn new() -> Self {
        Self {
            currency_name: None,
        }
    }

    pub fn currency_name(&mut self, currency_name: &'a str) -> &mut Self {
        self.currency_name = Some(currency_name);
        self
    }

    pub fn build(&self) -> Option<CurrencyName> {
        match self.currency_name {
            Some(currency_name) => match currency_name {
                "US Dollar" => Some(CurrencyName::US_Dollar),
                "Bitcoin" => Some(CurrencyName::Bitcoin),
                "Euro" => Some(CurrencyName::Euro),
                "British Pound" => Some(CurrencyName::British_Pound),
                _ => None,
            },
            None => None,
        }
    }
}

pub fn is_valid(currency_name: &str) -> bool {
    match currency_name {
        "US Dollar" => true,
        "Bitcoin" => true,
        "Euro" => true,
        "British Pound" => true,
        _ => false,
    }
}

pub fn get_currency_name_from_code(currency_code: &str) -> Option<CurrencyName> {
    match currency_code {
        "USD" => Some(CurrencyName::US_Dollar),
        "BTC" => Some(CurrencyName::Bitcoin),
        "EUR" => Some(CurrencyName::Euro),
        "GBP" => Some(CurrencyName::British_Pound),
        _ => None,
    }
}

pub fn get_currency_name_from_symbol(symbol: &str) -> Option<CurrencyName> {
    match symbol {
        "$" => Some(CurrencyName::US_Dollar),
        "₿" => Some(CurrencyName::Bitcoin),
        "€" => Some(CurrencyName::Euro),
        "£" => Some(CurrencyName::British_Pound),
        _ => None,
    }
}
