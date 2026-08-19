use serde::{Deserialize, Serialize};
use std::fmt;

/// 金额值对象（通过 `try_from` 确保反序列化时严格校验非负不变量）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "MoneyRaw")]
pub struct Money {
    amount_minor: i64,
    currency: Currency,
}

#[derive(Deserialize)]
struct MoneyRaw {
    amount_minor: i64,
    currency: Currency,
}

impl TryFrom<MoneyRaw> for Money {
    type Error = &'static str;

    fn try_from(raw: MoneyRaw) -> Result<Self, Self::Error> {
        Self::new(raw.amount_minor, raw.currency)
    }
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
    /// 构造金额值对象（最小货币单位，如分、美分）
    pub const fn new(amount_minor: i64, currency: Currency) -> Result<Self, &'static str> {
        if amount_minor < 0 {
            return Err("金额不能为负数");
        }
        Ok(Self {
            amount_minor,
            currency,
        })
    }

    /// 构造零元值对象
    #[must_use]
    pub const fn zero(currency: Currency) -> Self {
        Self {
            amount_minor: 0,
            currency,
        }
    }

    /// 获取金额数值（最小货币单位）
    #[must_use]
    pub const fn amount_minor(&self) -> i64 {
        self.amount_minor
    }

    /// 获取币种
    #[must_use]
    pub const fn currency(&self) -> Currency {
        self.currency
    }

    /// 是否为零元
    #[must_use]
    pub const fn is_zero(&self) -> bool {
        self.amount_minor == 0
    }

    /// 金额累加
    pub fn checked_add(self, other: Self) -> Result<Self, &'static str> {
        if self.currency != other.currency {
            return Err("不同币种无法直接相加");
        }
        let total = self
            .amount_minor
            .checked_add(other.amount_minor)
            .ok_or("金额累加溢出")?;
        Ok(Self {
            amount_minor: total,
            currency: self.currency,
        })
    }

    /// 金额乘法（乘以数量）
    pub fn checked_mul(self, factor: u32) -> Result<Self, &'static str> {
        let total = self
            .amount_minor
            .checked_mul(i64::from(factor))
            .ok_or("金额乘法溢出")?;
        Ok(Self {
            amount_minor: total,
            currency: self.currency,
        })
    }
}
