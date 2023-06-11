use macros::*;

use super::code::CurrencyCode;
use super::name::CurrencyName;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[json]
pub enum CurrencySymbol {
    USD,
    BTC,
    EUR,
    GBP,
}

impl CurrencySymbol {
    pub fn new<'a>() -> SymbolBuilder<'a> {
        SymbolBuilder::new()
    }

    pub fn get_symbol(&self) -> &str {
        match self {
            CurrencySymbol::BTC => "₿",
            CurrencySymbol::USD => "$",
            CurrencySymbol::EUR => "€",
            CurrencySymbol::GBP => "£",
        }
    }

    pub fn get_name(&self) -> CurrencyName {
        match self {
            CurrencySymbol::BTC => CurrencyName::Bitcoin,
            CurrencySymbol::USD => CurrencyName::US_Dollar,
            CurrencySymbol::EUR => CurrencyName::Euro,
            CurrencySymbol::GBP => CurrencyName::British_Pound,
        }
    }

    pub fn get_code(&self) -> CurrencyCode {
        match self {
            CurrencySymbol::BTC => CurrencyCode::BTC,
            CurrencySymbol::USD => CurrencyCode::USD,
            CurrencySymbol::EUR => CurrencyCode::EUR,
            CurrencySymbol::GBP => CurrencyCode::GBP,
        }
    }

    pub fn get_decimal_places(&self) -> u32 {
        match self {
            CurrencySymbol::BTC => 8,
            CurrencySymbol::USD => 2,
            CurrencySymbol::EUR => 2,
            CurrencySymbol::GBP => 2,
        }
    }
}

impl Default for CurrencySymbol {
    fn default() -> Self {
        Self::USD
    }
}

impl std::fmt::Display for CurrencySymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.get_symbol())
    }
}

pub struct SymbolBuilder<'a> {
    symbol: Option<&'a str>,
}

impl<'a> SymbolBuilder<'a> {
    fn new() -> Self {
        Self { symbol: None }
    }

    pub fn symbol(&mut self, symbol: &'a str) -> &mut Self {
        self.symbol = Some(symbol);
        self
    }

    pub fn build(&self) -> Option<CurrencySymbol> {
        if let Some(symbol) = self.symbol {
            match symbol {
                "USD" => Some(CurrencySymbol::USD),
                "BTC" => Some(CurrencySymbol::BTC),
                "EUR" => Some(CurrencySymbol::EUR),
                "GBP" => Some(CurrencySymbol::GBP),
                _ => None,
            }
        } else {
            None
        }
    }
}

pub fn is_valid(symbol: &str) -> bool {
    match symbol {
        "USD" => true,
        "BTC" => true,
        "EUR" => true,
        "GBP" => true,
        _ => false,
    }
}

pub fn get_symbol_from_code(code: &CurrencyCode) -> CurrencySymbol {
    match code {
        CurrencyCode::USD => CurrencySymbol::USD,
        CurrencyCode::BTC => CurrencySymbol::BTC,
        CurrencyCode::EUR => CurrencySymbol::EUR,
        CurrencyCode::GBP => CurrencySymbol::GBP,
    }
}

pub fn get_symbol_from_name(name: &CurrencyName) -> CurrencySymbol {
    match name {
        CurrencyName::US_Dollar => CurrencySymbol::USD,
        CurrencyName::Bitcoin => CurrencySymbol::BTC,
        CurrencyName::Euro => CurrencySymbol::EUR,
        CurrencyName::British_Pound => CurrencySymbol::GBP,
    }
}
