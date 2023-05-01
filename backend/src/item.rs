use crate::{item, schema::items};
use chrono::NaiveDate;
use diesel::{insert_into, prelude::*};
use serde::{Serialize, Deserialize};

#[derive(Debug, diesel_derive_enum::DbEnum, Deserialize)]
pub enum When {
    Today,
    Evening,
    Someday,
    Anytime,
    Date,
}


#[derive(Debug, diesel_derive_enum::DbEnum, Serialize, Deserialize)]
pub enum ItemType {
    Area,
    Project,
    Section,
    Item,
}

#[derive(Debug, diesel_derive_enum::DbEnum, Serialize, Deserialize)]
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
    pub item_status: Status,
}

#[derive(Serialize)]
pub enum WhenR {
    Today,
    Evening,
    Someday,
    Anytime,
    Date(NaiveDate),
}

#[derive(Serialize)]
pub struct ItemReturn {
    pub id: i32,
    pub when: Option<WhenR>,
    pub deadline: Option<NaiveDate>,
    pub children: Option<Box<Vec<ItemReturn>>>,
    pub title: String,
    pub body: Option<String>,
    pub item_type: ItemType,
    pub status: Status,
}

impl From<Item> for ItemReturn {
    fn from(value: Item) -> Self {
        let when = match value.when_type {
            Some(When::Date) => match value.when_date {
                Some(at) => Some(WhenR::Date(at)),
                None => None,
            },
            Some(When::Anytime) => Some(WhenR::Anytime),
            Some(When::Evening) => Some(WhenR::Evening),
            Some(When::Someday) => Some(WhenR::Someday),
            Some(When::Today) => Some(WhenR::Today),
            None => None,
        };

        Self {
            id: value.id,
            when: when,
            deadline: value.deadline,
            children: None,
            title: value.title,
            body: value.body,
            item_type: value.item_type,
            status: value.item_status,
        }
    }
}

pub fn by_id(
    post_id: i32,
    completed: bool,
    conn: &mut SqliteConnection,
) -> Result<ItemReturn, diesel::result::Error> {
    use self::items::dsl::*;

    // This type is needed because of a rust analyzer bug
    let item: item::Item = items.find(post_id).first::<Item>(conn)?;
    let mut children_query = Item::belonging_to(&item).into_boxed();

    if completed {
        children_query = children_query.filter(
            item_status
                .eq(Status::Completed)
                .or(item_status.eq(Status::Canceled)),
        )
    } else {
        children_query = children_query.filter(item_status.eq(Status::ToDo))
    }

    let children: Vec<Item> = children_query.load::<Item>(conn)?;

    let done = children
        .into_iter()
        .filter_map(|x| {
            use ItemType::*;
            match x.item_type {
                Project | Item => by_id(x.id, completed, conn).ok(),
                _ => Some(x.into()),
            }
        })
        .collect::<Vec<_>>();

    let mut almost_final: ItemReturn = item.into();
    almost_final.children = Some(Box::new(done));

    Ok(almost_final)
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = items)]
pub struct NewItem {
    pub parent: Option<i32>,
    pub title: String,
    pub item_type: ItemType,
    pub item_status: Status,
}

// Todo: Check parent (no nesting sections)
pub fn create(
    info: NewItem,
    conn: &mut SqliteConnection,
) -> Result<Option<ItemReturn>, diesel::result::Error> {
    use self::items::dsl::*;
    let rows: ItemReturn = insert_into(items)
        .values(&info)
        .get_results::<Item>(conn)?
        .remove(0)
        .into();
    Ok(Some(rows))
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = items)]
pub struct UpdateItem {
    pub when_type: Option<When>,
    pub when_date: Option<NaiveDate>,
    pub deadline: Option<NaiveDate>,
    pub parent: Option<i32>,
    pub title: Option<String>,
    pub body: Option<String>,
    pub item_type: Option<ItemType>,
    pub item_status: Option<Status>,
}

pub fn update(
    item_id: i32,
    updates: UpdateItem,
    conn: &mut SqliteConnection,
) -> Result<Option<ItemReturn>, diesel::result::Error> {
    use self::items::dsl::*;
    let rows: ItemReturn = diesel::update(items.filter(id.eq(item_id)))
        .set(updates)
        .get_results::<Item>(conn)?
        .remove(0)
        .into();
    Ok(Some(rows))
}
