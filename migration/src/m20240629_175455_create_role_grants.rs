use sea_orm_migration::prelude::*;


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        println!("Migration: Role Grants...");

        manager
            .create_table(
                Table::create()
                    .table(RoleGrants::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RoleGrants::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RoleGrants::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(RoleGrants::UpdatedAt).date_time().not_null())
                    .col(ColumnDef::new(RoleGrants::UserId).uuid().not_null())
                    .col(ColumnDef::new(RoleGrants::ResourceTable).string())
                    .col(ColumnDef::new(RoleGrants::ResourceId).uuid())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RoleGrants::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RoleGrants {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    UserId,
    ResourceTable,
    ResourceId
}
