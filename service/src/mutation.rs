use ::domains::{users, users::model::Entity as User};
use prelude::Uuid;
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    // pub async fn create_user(
    //     db: &DbConn,
    //     form_data: user::Model,
    // ) -> Result<user::ActiveModel, DbErr> {
    //     user::ActiveModel {
    //         username: Set(form_data.username.to_owned()),
    //         // text: Set(form_data.text.to_owned()),
    //         ..Default::default()
    //     }
    //     .save(db)
    //     .await
    // }

    // pub async fn update_user_by_id(
    //     db: &DbConn,
    //     id: Uuid,
    //     form_data: user::Model,
    // ) -> Result<user::Model, DbErr> {
    //     let post: user::ActiveModel = User::find_by_id(id)
    //         .one(db)
    //         .await?
    //         .ok_or(DbErr::Custom("Cannot find user.".to_owned()))
    //         .map(Into::into)?;

    //     user::ActiveModel {
    //         id: post.id,
    //         username: Set(form_data.username.to_owned()),
    //         password: Set(form_data.password.to_owned()),
    //         access_token: Set(form_data.access_token.to_owned()),
    //         created_at: todo!(),
    //         updated_at: todo!(),
    //     }
    //     .update(db)
    //     .await
    // }

    // pub async fn delete_user(db: &DbConn, id: Uuid) -> Result<DeleteResult, DbErr> {
    //     let user: user::ActiveModel = User::find_by_id(id)
    //         .one(db)
    //         .await?
    //         .ok_or(DbErr::Custom("Cannot find user.".to_owned()))
    //         .map(Into::into)?;

    //     user.delete(db).await
    // }

    // pub async fn delete_all_users(db: &DbConn) -> Result<DeleteResult, DbErr> {
    //     User::delete_many().exec(db).await
    // }
}