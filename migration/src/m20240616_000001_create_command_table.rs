use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Command::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Command::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Command::Command).string().not_null())
                    .col(ColumnDef::new(Command::Payload).string().not_null())
                    .col(ColumnDef::new(Command::Data).string().not_null())
                    .col(ColumnDef::new(Command::Timestamp).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Command::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Command {
    Table,
    Id,
    Command,
    Payload,
    Data,
    Timestamp,
}
