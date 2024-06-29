use sea_orm_migration::prelude::*;


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        println!("Migration: Campaigns...");

        manager
            .create_table(
                Table::create()
                    .table(Campaigns::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Campaigns::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Campaigns::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Campaigns::UpdatedAt).date_time().not_null())
                    .col(ColumnDef::new(Campaigns::Title).text().not_null())
                    .col(ColumnDef::new(Campaigns::Description).text())
                    .col(ColumnDef::new(Campaigns::Picture).text())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Campaigns::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Campaigns {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    Title,
    Description,
    Picture
}
