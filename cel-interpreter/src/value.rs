use cel_parser::ast::Literal;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use uuid::Uuid;

use std::{collections::HashMap, rc::Rc};

use crate::{cel_type::*, error::*};

#[derive(Debug, Clone, PartialEq)]
pub enum CelValue {
    Map(Rc<CelMap>),
    Int(i64),
    UInt(u64),
    Double(Decimal),
    String(Rc<String>),
    Bytes(Rc<Vec<u8>>),
    Bool(bool),
    Null,

    Date(NaiveDate),
    Uuid(Uuid),
}

impl CelValue {
    pub(crate) fn try_bool(&self) -> Result<bool, CelError> {
        if let CelValue::Bool(val) = self {
            Ok(*val)
        } else {
            Err(CelError::BadType(CelType::Bool, CelType::from(self)))
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CelMap {
    inner: HashMap<CelKey, CelValue>,
}

impl CelMap {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn insert(&mut self, k: impl Into<CelKey>, val: impl Into<CelValue>) {
        self.inner.insert(k.into(), val.into());
    }

    pub fn get(&self, key: impl Into<CelKey>) -> CelValue {
        self.inner
            .get(&key.into())
            .map(Clone::clone)
            .unwrap_or(CelValue::Null)
    }
}

impl Default for CelMap {
    fn default() -> Self {
        Self::new()
    }
}

impl From<HashMap<String, CelValue>> for CelMap {
    fn from(map: HashMap<String, CelValue>) -> Self {
        let mut res = CelMap::new();
        for (k, v) in map {
            res.insert(CelKey::String(Rc::from(k)), v);
        }
        res
    }
}

impl From<CelMap> for CelValue {
    fn from(m: CelMap) -> Self {
        CelValue::Map(Rc::from(m))
    }
}

impl From<i64> for CelValue {
    fn from(i: i64) -> Self {
        CelValue::Int(i)
    }
}

impl From<Decimal> for CelValue {
    fn from(d: Decimal) -> Self {
        CelValue::Double(d)
    }
}

impl From<NaiveDate> for CelValue {
    fn from(d: NaiveDate) -> Self {
        CelValue::Date(d)
    }
}

impl From<&str> for CelValue {
    fn from(s: &str) -> Self {
        CelValue::String(Rc::from(s.to_string()))
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum CelKey {
    Int(i64),
    UInt(u64),
    Bool(bool),
    String(Rc<String>),
}

impl From<&str> for CelKey {
    fn from(s: &str) -> Self {
        CelKey::String(Rc::from(s.to_string()))
    }
}

impl From<String> for CelKey {
    fn from(s: String) -> Self {
        CelKey::String(Rc::from(s))
    }
}

impl From<&Rc<String>> for CelKey {
    fn from(s: &Rc<String>) -> Self {
        CelKey::String(s.clone())
    }
}

impl From<&CelValue> for CelType {
    fn from(v: &CelValue) -> Self {
        match v {
            CelValue::Map(_) => CelType::Map,
            CelValue::Int(_) => CelType::Int,
            CelValue::UInt(_) => CelType::UInt,
            CelValue::Double(_) => CelType::Double,
            CelValue::String(_) => CelType::String,
            CelValue::Bytes(_) => CelType::Bytes,
            CelValue::Bool(_) => CelType::Bool,
            CelValue::Null => CelType::Null,

            CelValue::Date(_) => CelType::Date,
            CelValue::Uuid(_) => CelType::Uuid,
        }
    }
}

impl From<&Literal> for CelValue {
    fn from(l: &Literal) -> Self {
        use Literal::*;
        match l {
            Int(i) => CelValue::Int(*i),
            UInt(u) => CelValue::UInt(*u),
            Double(d) => CelValue::Double(d.parse().expect("Couldn't parse Decimal")),
            String(s) => CelValue::String(s.clone()),
            Bytes(b) => CelValue::Bytes(b.clone()),
            Bool(b) => CelValue::Bool(*b),
            Null => CelValue::Null,
        }
    }
}

impl TryFrom<&CelValue> for Rc<String> {
    type Error = CelError;

    fn try_from(v: &CelValue) -> Result<Self, Self::Error> {
        if let CelValue::String(s) = v {
            Ok(s.clone())
        } else {
            Err(CelError::BadType(CelType::String, CelType::from(v)))
        }
    }
}

impl TryFrom<CelValue> for NaiveDate {
    type Error = CelError;

    fn try_from(v: CelValue) -> Result<Self, Self::Error> {
        if let CelValue::Date(d) = v {
            Ok(d)
        } else {
            Err(CelError::BadType(CelType::Date, CelType::from(&v)))
        }
    }
}

impl TryFrom<CelValue> for Uuid {
    type Error = CelError;

    fn try_from(v: CelValue) -> Result<Self, Self::Error> {
        if let CelValue::Uuid(id) = v {
            Ok(id)
        } else {
            Err(CelError::BadType(CelType::Uuid, CelType::from(&v)))
        }
    }
}

impl TryFrom<CelValue> for String {
    type Error = CelError;

    fn try_from(v: CelValue) -> Result<Self, Self::Error> {
        if let CelValue::String(s) = v {
            Ok(s.to_string())
        } else {
            Err(CelError::BadType(CelType::String, CelType::from(&v)))
        }
    }
}

impl TryFrom<CelValue> for Decimal {
    type Error = CelError;

    fn try_from(v: CelValue) -> Result<Self, Self::Error> {
        match v {
            CelValue::Double(n) => Ok(n),
            CelValue::Int(n) => Ok(Decimal::from(n)),
            CelValue::UInt(n) => Ok(Decimal::from(n)),
            _ => Err(CelError::BadType(CelType::Double, CelType::from(&v))),
        }
    }
}

impl From<&CelKey> for CelType {
    fn from(v: &CelKey) -> Self {
        match v {
            CelKey::Int(_) => CelType::Int,
            CelKey::UInt(_) => CelType::UInt,
            CelKey::Bool(_) => CelType::Bool,
            CelKey::String(_) => CelType::String,
        }
    }
}

impl TryFrom<&CelKey> for String {
    type Error = CelError;

    fn try_from(v: &CelKey) -> Result<Self, Self::Error> {
        if let CelKey::String(s) = v {
            Ok(s.to_string())
        } else {
            Err(CelError::BadType(CelType::String, CelType::from(v)))
        }
    }
}

impl TryFrom<CelValue> for serde_json::Value {
    type Error = CelError;

    fn try_from(v: CelValue) -> Result<Self, Self::Error> {
        use serde_json::*;
        Ok(match v {
            CelValue::Int(n) => Value::from(n),
            CelValue::UInt(n) => Value::from(n),
            CelValue::Double(n) => Value::from(n.to_string()),
            CelValue::Bool(b) => Value::from(b),
            CelValue::String(n) => Value::from(n.as_str()),
            CelValue::Null => Value::Null,
            CelValue::Date(d) => Value::from(d.to_string()),
            CelValue::Uuid(u) => Value::from(u.to_string()),
            CelValue::Map(m) => {
                let mut res = serde_json::Map::new();
                for (k, v) in m.inner.iter() {
                    let key: String = k.try_into()?;
                    let value = Self::try_from(v.clone())?;
                    res.insert(key, value);
                }
                Value::from(res)
            }
            _ => unimplemented!(),
        })
    }
}
