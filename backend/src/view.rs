use crate::{
    item::{Item, ItemType, Status, When, WhenR, ItemReturn, self},
    schema::items,
};
use chrono::NaiveDate;
use diesel::{prelude::*};

pub enum Page {
    Inbox,
    Today,
    Upcoming,
    Anytime,
    Someday,
    Logbook,
    Trash,
}

pub struct Parents {
    section: Option<crate::item::ItemReturn>,
    project: Option<crate::item::ItemReturn>,
}

pub struct ViewItemReturn {
    pub id: i32,
    pub when: Option<WhenR>,
    pub deadline: Option<NaiveDate>,
    pub parents: Option<Parents>,
    pub title: String,
    pub body: Option<String>,
    pub status: Status,
}

impl From<Item> for ViewItemReturn {
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
            parents: None,
            title: value.title,
            body: value.body,
            status: value.item_status,
        }
    }
}

// Eventually this should chain on a vec of items.
fn chain_reverse<'a>(conn: &mut SqliteConnection, item_id: i32, parents: Parents) -> Parents {
    use self::items::dsl::*;

    let par: Result<Item, diesel::result::Error> = items.find(item_id).first::<Item>(conn);

    match par {
        Ok(unwap) => match unwap.item_type {
            ItemType::Area | ItemType::Project => {
                Parents { section: parents.section, project: Some(ItemReturn::from(unwap)) }
            }
            ItemType::Section => {
                Parents { section: Some(ItemReturn::from(unwap)), project: parents.project }
            }
            ItemType::Item => chain_reverse(conn, unwap.id, parents)
        }
        Err(_) => parents
    }
}

fn chain_entry(conn: &mut SqliteConnection, items: Vec<item::Item>) -> Vec<ViewItemReturn> {
    items
    .into_iter()
    .map(|x| ViewItemReturn::from(x))
    .map(|mut x| {
        x.parents = Some(chain_reverse(conn, x.id, Parents { section: None, project: None }));
        x
    }).collect::<Vec<_>>()
}

pub fn get_inbox(conn: &mut SqliteConnection) -> Result<Vec<ViewItemReturn>, diesel::result::Error> {
    use self::items::dsl::*;

    let item: Vec<Item> = items
        .filter(parent.is_null())
        .filter(item_status.eq(Status::Completed))
        .filter(item_type.eq(ItemType::Item))
        .load::<Item>(conn)?;
    Ok(item
        .into_iter()
        .map(|x| ViewItemReturn::from(x))
        .collect::<Vec<_>>())
}


// Today, Someday, Anytime
pub fn get_by_when(conn: &mut SqliteConnection, when: When) -> Result<Vec<ViewItemReturn>, diesel::result::Error> {
    use self::items::dsl::*;

    let item: Vec<item::Item> = items
        .filter(when_type.eq(when))
        .filter(item_status.eq(Status::Completed))
        .filter(item_type.eq(ItemType::Item))
        .load::<Item>(conn)?;

    Ok(chain_entry(conn, item))
}

// Logbook, Trash
pub fn get_by_status(conn: &mut SqliteConnection, status: Status) -> Result<Vec<ViewItemReturn>, diesel::result::Error> {
    use self::items::dsl::*;

    let item: Vec<item::Item> = items
        .filter(item_status.eq(status))
        .filter(item_type.eq(ItemType::Item))
        .load::<Item>(conn)?;

    Ok(chain_entry(conn, item))
}
