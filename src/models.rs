use crate::schema::session_cookies;
use diesel::prelude::*;

#[derive(Insertable)]
#[diesel(table_name = session_cookies)]
pub struct NewSessionCookie<'a> {
    pub username: &'a str,
    pub val: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = session_cookies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SessionCookie {
    pub id: i32,
    pub username: String,
    pub val: String,
}
