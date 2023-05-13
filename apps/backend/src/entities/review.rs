//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.2

use async_trait::async_trait;
use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{migrator::ReviewVisibility, utils::associate_user_with_metadata};

use super::utils::SeenExtraInformation;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "review")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub posted_on: DateTimeUtc,
    pub rating: Option<Decimal>,
    pub text: Option<String>,
    pub visibility: ReviewVisibility,
    pub spoiler: bool,
    pub user_id: i32,
    pub metadata_id: i32,
    pub extra_information: Option<SeenExtraInformation>,
    pub identifier: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::metadata::Entity",
        from = "Column::MetadataId",
        to = "super::metadata::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Metadata,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::metadata::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Metadata.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn after_save<C>(model: Model, db: &C, insert: bool) -> Result<Model, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert {
            associate_user_with_metadata(&model.user_id, &model.metadata_id, db)
                .await
                .ok();
        }
        Ok(model)
    }
}