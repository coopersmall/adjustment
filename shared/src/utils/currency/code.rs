use macros::*;

use super::name::CurrencyName;
use super::symbol::CurrencySymbol;

#[json]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum CurrencyCode {
    USD,
    BTC,
    EUR,
    GBP,
    // Add other currency codes as needed
}

impl<'a> CurrencyCode {
    pub fn new() -> CurrencyCodeBuilder<'a> {
        CurrencyCodeBuilder::new()
    }

    pub fn to_string(&self) -> &str {
        match self {
            Self::USD => "USD",
            Self::BTC => "BTC",
            Self::EUR => "EUR",
            Self::GBP => "GBP",
        }
    }

    pub fn get_symbol(&self) -> CurrencySymbol {
        match self {
            Self::USD => CurrencySymbol::USD,
            Self::BTC => CurrencySymbol::BTC,
            Self::EUR => CurrencySymbol::EUR,
            Self::GBP => CurrencySymbol::GBP,
        }
    }

    pub fn get_name(&self) -> CurrencyName {
        match self {
            Self::USD => CurrencyName::US_Dollar,
            Self::BTC => CurrencyName::Bitcoin,
            Self::EUR => CurrencyName::Euro,
            Self::GBP => CurrencyName::British_Pound,
        }
    }
}

impl Default for CurrencyCode {
    fn default() -> Self {
        Self::USD
    }
}

pub struct CurrencyCodeBuilder<'a> {
    currency_code: Option<&'a str>,
}

impl<'a> CurrencyCodeBuilder<'a> {
    fn new() -> Self {
        Self {
            currency_code: None,
        }
    }

    pub fn currency_code(mut self, currency_code: &'a str) -> Self {
        self.currency_code = Some(currency_code);
        self
    }

    pub fn build(self) -> Option<CurrencyCode> {
        match self.currency_code {
            Some(currency_code) => match currency_code {
                "USD" => Some(CurrencyCode::USD),
                "BTC" => Some(CurrencyCode::BTC),
                "EUR" => Some(CurrencyCode::EUR),
                "GBP" => Some(CurrencyCode::GBP),
                _ => None,
            },
            None => None,
        }
    }
}

pub fn is_valid(currency_code: &str) -> bool {
    match currency_code {
        "USD" => true,
        "BTC" => true,
        "EUR" => true,
        "GBP" => true,
        _ => false,
    }
}

pub fn get_currency_code_from_symbol(currency_symbol: CurrencySymbol) -> Option<CurrencyCode> {
    match currency_symbol {
        CurrencySymbol::USD => Some(CurrencyCode::USD),
        CurrencySymbol::BTC => Some(CurrencyCode::BTC),
        CurrencySymbol::EUR => Some(CurrencyCode::EUR),
        CurrencySymbol::GBP => Some(CurrencyCode::GBP),
    }
}

pub fn get_currency_code_from_name(currency_name: CurrencyName) -> Option<CurrencyCode> {
    match currency_name {
        CurrencyName::US_Dollar => Some(CurrencyCode::USD),
        CurrencyName::Bitcoin => Some(CurrencyCode::BTC),
        CurrencyName::Euro => Some(CurrencyCode::EUR),
        CurrencyName::British_Pound => Some(CurrencyCode::GBP),
    }
}
