use ::entity::{appuser, appuser::Entity as Appuser};
use prelude::Uuid;
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn find_post_by_id(db: &DbConn, id: Uuid) -> Result<Option<appuser::Model>, DbErr> {
        Appuser::find_by_id(id).one(db).await
    }

    /// If ok, returns (appuser models, num pages).
    pub async fn find_appusers_in_page(
        db: &DbConn,
        page: u64,
        appusers_per_page: u64,
    ) -> Result<(Vec<appuser::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = Appuser::find()
            .order_by_asc(appuser::Column::Id)
            .paginate(db, appusers_per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated appusers
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}