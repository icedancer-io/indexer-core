//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "transaction")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub slot: i64,
    #[sea_orm(
        primary_key,
        auto_increment = false,
        column_type = "Binary(BlobSize::Blob(None))"
    )]
    pub signature: Vec<u8>,
    pub is_vote: bool,
    pub message_type: Option<i16>,
    #[sea_orm(column_type = "custom(\"TransactionMessage\")", nullable)]
    pub legacy_message: Option<String>,
    #[sea_orm(column_type = "custom(\"LoadedMessageV0\")", nullable)]
    pub v0_loaded_message: Option<String>,
    pub signatures: Option<Vec<Vec<u8>>>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))", nullable)]
    pub message_hash: Option<Vec<u8>>,
    #[sea_orm(column_type = "custom(\"TransactionStatusMeta\")", nullable)]
    pub meta: Option<String>,
    pub write_version: Option<i64>,
    pub updated_on: DateTime,
    pub index: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}