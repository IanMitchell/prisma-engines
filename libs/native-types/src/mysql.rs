use serde::*;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MySqlType {
    Int,
    SmallInt,
    TinyInt,
    MediumInt,
    BigInt,
    Decimal(u8, u8),
    Numeric(u8, u8),
    Float,
    Double,
    Bit(u32),
    Char(u32),
    VarChar(u32),
    Binary(u32),
    VarBinary(u32),
    TinyBlob,
    Blob,
    MediumBlob,
    LongBlob,
    TinyText,
    Text,
    MediumText,
    LongText,
    Date,
    Time(Option<u32>),
    DateTime(Option<u32>),
    Timestamp(Option<u32>),
    Year,
    JSON,
}

impl super::NativeType for MySqlType {
    fn to_json(&self) -> Value {
        serde_json::to_value(&self).expect(&format!("Serializing the native type to json failed: {:?}", &self))
    }
}
