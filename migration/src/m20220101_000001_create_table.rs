use sea_orm::Set;
use sea_orm_migration::prelude::*;
use entity::appuser;
use sea_orm_migration::sea_orm::ActiveModelTrait;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        manager
            .create_table(
                Table::create()
                    .table(Appuser::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Appuser::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Appuser::Username).string().not_null())
                    .col(ColumnDef::new(Appuser::Password).string())
                    .col(ColumnDef::new(Appuser::AccessToken).string())
                    .to_owned(),
            )
            .await?;

        appuser::ActiveModel {
            username: Set("admin".to_owned()),
            password: Set(Some("$argon2id$v=19$m=19456,t=2,p=1$VE0e3g7DalWHgDwou3nuRA$uC6TER156UQpk0lNQ5+jHM0l5poVjPA1he/Tyn9J4Zw".to_owned())),
            ..Default::default()
        }.insert(db)
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Appuser::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Appuser {
    Table,
    Id,
    Username,
    Password,
    AccessToken
}