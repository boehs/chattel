use sea_orm_migration::{
    prelude::*,
    sea_orm::{EnumIter, Iterable},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden, EnumIter)]
enum Type {
    Table,
    #[iden = "Area"]
    Area,
    #[iden = "Project"]
    Project,
    #[iden = "Section"]
    Section,
    #[iden = "Item"]
    Item,
}

#[derive(Iden, EnumIter)]
enum Status {
    Table,
    #[iden = "ToDo"]
    ToDo,
    #[iden = "Completed"]
    Completed,
    #[iden = "Canceled"]
    Canceled,
    #[iden = "Trashed"]
    Trashed,
}

#[derive(Iden, EnumIter)]
enum When {
    Table,
    #[iden = "Today"]
    Today,
    #[iden = "Evening"]
    Evening,
    #[iden = "Someday"]
    Someday,
    #[iden = "Date"]
    Date,
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Item {
    Table,
    Id,
    When,
    Deadline,
    Parent,
    #[iden = "title"]
    Title,
    #[iden = "body"]
    Body,
    Type,
    Status,
}

#[derive(Iden)]
enum Timeblock {
    Table,
    Id,
    Item,
    Start,
    End,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Item::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Item::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Item::When).enumeration(When::Table, When::iter().skip(1)))
                    .col(ColumnDef::new(Item::Deadline).date())
                    .col(ColumnDef::new(Item::Parent).integer())
                    .col(ColumnDef::new(Item::Title).string().not_null())
                    .col(ColumnDef::new(Item::Body).string())
                    .col(
                        ColumnDef::new(Item::Type)
                            .enumeration(Type::Table, Type::iter().skip(1))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Item::Status)
                            .enumeration(Status::Table, Status::iter().skip(1))
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("Item_Parent")
                            .from(Item::Table, Item::Parent)
                            .to(Item::Table, Item::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Timeblock::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Timeblock::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Timeblock::Item).integer().not_null())
                    .col(ColumnDef::new(Timeblock::Start).date_time().not_null())
                    .col(ColumnDef::new(Timeblock::End).date_time().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("Timeblock_Item")
                            .from(Timeblock::Table, Timeblock::Item)
                            .to(Item::Table, Item::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    #[allow(unused_variables)]
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!()
    }
}
