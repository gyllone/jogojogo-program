use anchor_lang::prelude::*;
use rust_decimal::Decimal;

use crate::error::JogoError;

#[derive(Clone, Copy, Eq, PartialEq, AnchorSerialize, AnchorDeserialize)]
pub struct JogoDecimal {
    pub mantissa: i128,
    pub scale: u32,
}

impl From<Decimal> for JogoDecimal {
    fn from(s: Decimal) -> Self {
        Self {
            mantissa: s.mantissa(),
            scale: s.scale(),
        }
    }
}

impl TryInto<Decimal> for JogoDecimal {
    type Error = Error;
    fn try_into(self) -> Result<Decimal> {
        Decimal::try_from_i128_with_scale(self.mantissa, self.scale)
            .map_err(|_| error!(JogoError::DecimalConversionError))
    }
}