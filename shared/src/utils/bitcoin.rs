use macros::*;

use super::currency::Currency;

#[json_parse]
#[derive(Debug, Clone, PartialEq)]
pub struct Bitcoin<'a> {
    name: &'a str,
    price: Currency,
}

impl<'a> Bitcoin<'a> {
    pub fn new() -> BitcoinBuilder<'a> {
        BitcoinBuilder::new()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn price(&self) -> &Currency {
        &self.price
    }
}

impl<'a> Default for Bitcoin<'a> {
    fn default() -> Self {
        Self {
            name: "",
            price: Currency::default(),
        }
    }
}

pub struct BitcoinBuilder<'a> {
    name: Option<&'a str>,
    price: Option<Currency>,
}

impl<'a> BitcoinBuilder<'a> {
    pub fn new() -> Self {
        Self {
            name: None,
            price: None,
        }
    }

    pub fn name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    pub fn price(mut self, price: Currency) -> Self {
        self.price = Some(price);
        self
    }

    pub fn build(self) -> Bitcoin<'a> {
        Bitcoin {
            name: self.name.unwrap(),
            price: self.price.unwrap(),
        }
    }
}
