use ::domains::{users, users::model::Entity as User};
use prelude::Uuid;
use sea_orm::*;

pub struct Query;

impl Query {
    // pub async fn find_post_by_id(db: &DbConn, id: Uuid) -> Result<Option<user::Model>, DbErr> {
    //     User::find_by_id(id).one(db).await
    // }

    // /// If ok, returns (user models, num pages).
    // pub async fn find_users_in_page(
    //     db: &DbConn,
    //     page: u64,
    //     users_per_page: u64,
    // ) -> Result<(Vec<user::Model>, u64), DbErr> {
    //     // Setup paginator
    //     let paginator = User::find()
    //         .order_by_asc(user::Column::Id)
    //         .paginate(db, users_per_page);
    //     let num_pages = paginator.num_pages().await?;

    //     // Fetch paginated users
    //     paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    // }
}