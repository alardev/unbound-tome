//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15
use chrono::Utc;
use oso::PolarClass;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Authorization rules
pub const AUTHORIZATION: &str = include_str!("authorization.polar");

#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    DeriveEntityModel,
    Deserialize,
    Serialize,
    PolarClass,
)]
#[sea_orm(table_name = "shows")]
pub struct Model {
    /// The Campaign id
    #[sea_orm(primary_key, auto_increment = false)]
    #[polar(attribute)]
    pub id: Uuid,

    /// The date the Campaign was created
    pub created_at: DateTime,

    /// The date the Campaign was last updated
    pub updated_at: DateTime,

    /// The Campaign title
    #[sea_orm(column_type = "Text")]
    pub title: String,

    /// An optional Campaign summary
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,

    /// An optional Campaign image
    #[sea_orm(column_type = "Text", nullable)]
    pub picture: Option<String>,
}

pub type Campaign = Model;

/// Show entity relationships
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Default for Model {
    fn default() -> Self {
        Self {
            id: Uuid::default(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            title: String::default(),
            description: Option::default(),
            picture: Option::default(),
        }
    }
}