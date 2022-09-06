use crate::postgres::{PgTypeInfo, Postgres};
use crate::types::Type;
use ethereum_types::U256;

impl Type<Postgres> for U256 {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::NUMERIC
    }
}

impl Type<Postgres> for [U256] {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::NUMERIC_ARRAY
    }
}

impl Type<Postgres> for Vec<U256> {
    fn type_info() -> PgTypeInfo {
        <[U256] as Type<Postgres>>::type_info()
    }
}
