#![cfg(feature = "sea-orm")]

use grain_id::GrainId;
use rand::RngExt as _;
use sea_orm::{
    DatabaseBackend, MockDatabase, MockExecResult, Transaction,
    entity::{prelude::*, *},
};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "grain_id")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: GrainId,
    pub value: GrainId,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

async fn assert_model(model: Model) {
    let active_model = model.clone().into_active_model().reset_all();

    let db = MockDatabase::new(DatabaseBackend::Sqlite)
        .append_query_results([vec![model.clone()], vec![model.clone()]])
        .append_exec_results([MockExecResult {
            last_insert_id: model.id.into(),
            rows_affected: 1,
        }])
        .into_connection();

    assert_eq!(active_model.insert(&db).await.unwrap(), model.clone());
    assert_eq!(Entity::find().one(&db).await.unwrap(), Some(model.clone()));
    assert_eq!(
        db.into_transaction_log(),
        [
            Transaction::from_sql_and_values(
                DatabaseBackend::Sqlite,
                r#"INSERT INTO "grain_id" ("id", "value") VALUES (?, ?)"#,
                [model.id.into(), model.value.into(),]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Sqlite,
                r#"SELECT "grain_id"."id", "grain_id"."value" FROM "grain_id" WHERE "grain_id"."id" = ? LIMIT ?"#,
                [model.id.into(), 1u64.into()]
            ),
            Transaction::from_sql_and_values(
                DatabaseBackend::Sqlite,
                r#"SELECT "grain_id"."id", "grain_id"."value" FROM "grain_id" LIMIT ?"#,
                [1u64.into()]
            )
        ]
    );
}

#[tokio::test]
async fn nil() {
    assert_model(Model {
        id: GrainId::NIL,
        value: GrainId::NIL,
    })
    .await;
}

#[tokio::test]
async fn max() {
    assert_model(Model {
        id: GrainId::MAX,
        value: GrainId::MAX,
    })
    .await;
}

#[tokio::test]
async fn random() {
    let mut rng = rand::rng();
    for _ in 0..10 {
        assert_model(Model {
            id: rng.random(),
            value: rng.random(),
        })
        .await;
    }
}
