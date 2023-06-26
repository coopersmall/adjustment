use super::code::CurrencyCode;
use super::symbol::CurrencySymbol;

#[derive(Debug, Clone, PartialEq)]
#[macros::json]
pub enum CurrencyName {
    Dollar,
    Bitcoin,
    Euro,
    Pound,
}

impl<'a> CurrencyName {
    pub fn new() -> CurrencyNameBuilder<'a> {
        CurrencyNameBuilder::new()
    }

    pub fn to_string(&self) -> &str {
        match self {
            Self::Dollar => "Dollar",
            Self::Bitcoin => "Bitcoin",
            Self::Euro => "Euro",
            Self::Pound => "Pound",
        }
    }

    pub fn to_string_plural(&self) -> &str {
        match self {
            Self::Dollar => "Dollars",
            Self::Bitcoin => "Bitcoin",
            Self::Euro => "Euros",
            Self::Pound => "Pounds",
        }
    }

    pub fn get_symbol(&self) -> CurrencySymbol {
        match self {
            Self::Dollar => CurrencySymbol::USD,
            Self::Bitcoin => CurrencySymbol::BTC,
            Self::Euro => CurrencySymbol::EUR,
            Self::Pound => CurrencySymbol::GBP,
        }
    }

    pub fn get_code(&self) -> CurrencyCode {
        match self {
            Self::Dollar => CurrencyCode::USD,
            Self::Bitcoin => CurrencyCode::BTC,
            Self::Euro => CurrencyCode::EUR,
            Self::Pound => CurrencyCode::GBP,
        }
    }
}

impl Default for CurrencyName {
    fn default() -> Self {
        Self::Dollar
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
                "Dollar" => Some(CurrencyName::Dollar),
                "Bitcoin" => Some(CurrencyName::Bitcoin),
                "Euro" => Some(CurrencyName::Euro),
                "Pound" => Some(CurrencyName::Pound),
                _ => None,
            },
            None => None,
        }
    }
}

pub fn is_valid(currency_name: &str) -> bool {
    match currency_name {
        "Dollar" => true,
        "Bitcoin" => true,
        "Euro" => true,
        "Pound" => true,
        _ => false,
    }
}

pub fn get_currency_name_from_code(currency_code: &str) -> Option<CurrencyName> {
    match currency_code {
        "USD" => Some(CurrencyName::Dollar),
        "BTC" => Some(CurrencyName::Bitcoin),
        "EUR" => Some(CurrencyName::Euro),
        "GBP" => Some(CurrencyName::Pound),
        _ => None,
    }
}

pub fn get_currency_name_from_symbol(symbol: &str) -> Option<CurrencyName> {
    match symbol {
        "$" => Some(CurrencyName::Dollar),
        "₿" => Some(CurrencyName::Bitcoin),
        "€" => Some(CurrencyName::Euro),
        "£" => Some(CurrencyName::Pound),
        _ => None,
    }
}
