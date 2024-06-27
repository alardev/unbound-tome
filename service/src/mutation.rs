use ::domains::{appuser, appuser::Entity as Appuser};
use prelude::Uuid;
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn create_appuser(
        db: &DbConn,
        form_data: appuser::Model,
    ) -> Result<appuser::ActiveModel, DbErr> {
        appuser::ActiveModel {
            username: Set(form_data.username.to_owned()),
            // text: Set(form_data.text.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_appuser_by_id(
        db: &DbConn,
        id: Uuid,
        form_data: appuser::Model,
    ) -> Result<appuser::Model, DbErr> {
        let post: appuser::ActiveModel = Appuser::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find appuser.".to_owned()))
            .map(Into::into)?;

        appuser::ActiveModel {
            id: post.id,
            username: Set(form_data.username.to_owned()),
            password: Set(form_data.password.to_owned()),
            access_token: Set(form_data.access_token.to_owned()),
            created_at: todo!(),
            updated_at: todo!(),
        }
        .update(db)
        .await
    }

    pub async fn delete_appuser(db: &DbConn, id: Uuid) -> Result<DeleteResult, DbErr> {
        let appuser: appuser::ActiveModel = Appuser::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find appuser.".to_owned()))
            .map(Into::into)?;

        appuser.delete(db).await
    }

    pub async fn delete_all_appusers(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Appuser::delete_many().exec(db).await
    }
}