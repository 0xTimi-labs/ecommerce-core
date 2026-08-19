use serde::{Deserialize, Deserializer, Serialize, Serializer};
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

/// 币种代码（ISO 4217 三位大写字母标准代码，支持全局任意合法货币）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Currency([u8; 3]);

impl Currency {
    /// 人民币
    pub const CNY: Self = Self(*b"CNY");
    /// 美元
    pub const USD: Self = Self(*b"USD");
    /// 欧元
    pub const EUR: Self = Self(*b"EUR");

    /// 创建币种代码并校验 ISO 4217 规范
    pub fn new(code: &str) -> Result<Self, &'static str> {
        let bytes = code.as_bytes();
        if bytes.len() != 3 {
            return Err("币种代码必须为 3 位 ISO 4217 字符");
        }
        if !bytes[0].is_ascii_uppercase()
            || !bytes[1].is_ascii_uppercase()
            || !bytes[2].is_ascii_uppercase()
        {
            return Err("币种代码必须为大写英文字母");
        }
        Ok(Self([bytes[0], bytes[1], bytes[2]]))
    }

    /// 获取币种字符串切片
    #[must_use]
    pub fn as_str(&self) -> &str {
        // 安全保证：构造时已校验必须为 ASCII 大写字符
        match std::str::from_utf8(&self.0) {
            Ok(s) => s,
            Err(_) => "CNY",
        }
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Serialize for Currency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for Currency {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::new(&s).map_err(serde::de::Error::custom)
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
