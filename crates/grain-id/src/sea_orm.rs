use sea_orm::TryFromU64;

use super::GrainId;

impl From<GrainId> for sea_orm::Value {
    fn from(value: GrainId) -> Self {
        sea_orm::sea_query::Value::BigUnsigned(Some(value.into()))
    }
}

impl sea_orm::TryGetable for GrainId {
    fn try_get_by<I: sea_orm::ColIdx>(
        res: &sea_orm::QueryResult,
        index: I,
    ) -> Result<Self, sea_orm::TryGetError> {
        <u64 as sea_orm::TryGetable>::try_get_by(res, index).map(GrainId::from_u64_lossy)
    }
}

impl sea_orm::sea_query::ValueType for GrainId {
    fn try_from(v: sea_orm::Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
        <u64 as sea_orm::sea_query::ValueType>::try_from(v).map(GrainId::from_u64_lossy)
    }
    fn type_name() -> String {
        stringify!(GrainId).to_owned()
    }
    fn array_type() -> sea_orm::sea_query::ArrayType {
        sea_orm::sea_query::ArrayType::BigUnsigned
    }
    fn column_type() -> sea_orm::ColumnType {
        sea_orm::sea_query::ColumnType::BigUnsigned
    }
}

impl sea_orm::sea_query::Nullable for GrainId {
    fn null() -> sea_orm::Value {
        <u64 as sea_orm::sea_query::Nullable>::null()
    }
}

impl TryFromU64 for GrainId {
    fn try_from_u64(n: u64) -> Result<Self, sea_orm::DbErr> {
        Self::try_from(n).map_err(|x| sea_orm::DbErr::TryIntoErr {
            from: stringify!(u64),
            into: stringify!(GrainId),
            source: std::sync::Arc::new(x) as std::sync::Arc<dyn std::error::Error + Send + Sync>,
        })
    }
}
