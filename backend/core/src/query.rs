use ::entity::{item, item::Entity as Item}
use sea_orm::*;

pub struct ChildrenFilter {
    status: Option<Status>
}

pub struct Query;

impl Query {
    pub async fn get_item_by_id(db: &DbConn, id: i32) -> Result<Option<post::Model>, DbErr> {
        Item::find_by_id(id).one(db).await
    }
}
