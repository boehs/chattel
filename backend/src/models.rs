use crate::schema::items;
use chrono::{NaiveDate};
use diesel::prelude::*;

#[derive(Debug, diesel_derive_enum::DbEnum)]
pub enum When {
    Today,
    Evening,
    Someday,
    Anytime,
}

#[derive(Debug, diesel_derive_enum::DbEnum)]
pub enum ItemType {
    Area,
    Project,
    Section,
    Item,
}

#[derive(Debug, diesel_derive_enum::DbEnum)]
pub enum Status {
    ToDo,
    Completed,
    Canceled,
    Trashed,
}

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Item, foreign_key = parent ))]
pub struct Item {
    pub id: i32,
    pub when_type: Option<When>,
    pub when_date: Option<NaiveDate>,
    pub deadline: Option<NaiveDate>,
    pub parent: Option<i32>,
    pub title: String,
    pub body: Option<String>,
    pub item_type: ItemType,
    pub item_status: Status
}

impl From<Option<When>> for When {
    fn from(o: Option<When>) -> Self {
        match o {
            None => When::Anytime,
            Some(at) => at,
        }
    }
}

pub fn id(post_id: i32, conn: &mut SqliteConnection) {
    use self::items::dsl::*;

    let item = items.find(post_id).first::<Item>(conn).expect(":(");
    let children = Item::belonging_to(&item).load::<Item>(conn);
}
