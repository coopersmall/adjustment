use macros::*;

pub mod code;
pub mod name;
pub mod symbol;

use code::CurrencyCode;
use name::CurrencyName;
use symbol::CurrencySymbol;

#[derive(Debug, Clone, PartialEq)]
#[json_parse]
pub struct Currency {
    code: CurrencyCode,
    name: CurrencyName,
    symbol: CurrencySymbol,
}

impl Currency {
    pub fn new<'a>() -> CurrencyBuilder<'a> {
        CurrencyBuilder::new()
    }

    pub fn code(&self) -> &CurrencyCode {
        &self.code
    }

    pub fn name(&self) -> &CurrencyName {
        &self.name
    }

    pub fn symbol(&self) -> &CurrencySymbol {
        &self.symbol
    }
}

impl Default for Currency {
    fn default() -> Self {
        Self {
            code: CurrencyCode::default(),
            name: CurrencyName::default(),
            symbol: CurrencySymbol::default(),
        }
    }
}

pub struct CurrencyBuilder<'a> {
    code: Option<&'a str>,
    name: Option<&'a str>,
    symbol: Option<&'a str>,
}

impl<'a> CurrencyBuilder<'a> {
    pub fn new() -> Self {
        Self {
            code: None,
            name: None,
            symbol: None,
        }
    }

    pub fn code(mut self, code: &'a str) -> Self {
        self.code = Some(code);
        self
    }

    pub fn name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    pub fn symbol(mut self, symbol: &'a str) -> Self {
        self.symbol = Some(symbol);
        self
    }

    pub fn build(self) -> Result<Currency, &'static str> {
        let code = self.code.ok_or("Missing code")?;
        let name = self.name.ok_or("Missing name")?;
        let symbol = self.symbol.ok_or("Missing symbol")?;

        Ok(Currency {
            code: CurrencyCode::new()
                .currency_code(code)
                .build()
                .ok_or("Invalid code")?,
            name: CurrencyName::new()
                .currency_name(name)
                .build()
                .ok_or("Invalid name")?,
            symbol: CurrencySymbol::new()
                .symbol(symbol)
                .build()
                .ok_or("Invalid symbol")?,
        })
    }
}
