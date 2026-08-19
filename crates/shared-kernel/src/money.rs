use serde::{Deserialize, Serialize};
use std::fmt;

/// 金额值对象
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money {
    amount_cents: i64,
    currency: Currency,
}

/// 币种
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Currency {
    /// 人民币
    CNY,
    /// 美元
    USD,
    /// 欧元
    EUR,
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CNY => write!(f, "CNY"),
            Self::USD => write!(f, "USD"),
            Self::EUR => write!(f, "EUR"),
        }
    }
}

impl Money {
    /// 构造金额值对象
    pub const fn new(amount_cents: i64, currency: Currency) -> Result<Self, &'static str> {
        if amount_cents < 0 {
            return Err("金额不能为负数");
        }
        Ok(Self {
            amount_cents,
            currency,
        })
    }

    /// 构造零元值对象
    pub const fn zero(currency: Currency) -> Self {
        Self {
            amount_cents: 0,
            currency,
        }
    }

    /// 获取金额（分）
    pub const fn amount_cents(&self) -> i64 {
        self.amount_cents
    }

    /// 获取币种
    pub const fn currency(&self) -> Currency {
        self.currency
    }

    /// 是否为零元
    pub const fn is_zero(&self) -> bool {
        self.amount_cents == 0
    }

    /// 金额累加
    pub fn checked_add(self, other: Self) -> Result<Self, &'static str> {
        if self.currency != other.currency {
            return Err("不同币种无法直接相加");
        }
        let total = self
            .amount_cents
            .checked_add(other.amount_cents)
            .ok_or("金额累加溢出")?;
        Ok(Self {
            amount_cents: total,
            currency: self.currency,
        })
    }
}
