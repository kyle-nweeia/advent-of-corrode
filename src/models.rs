#[derive(diesel::prelude::Insertable)]
#[diesel(table_name = crate::schema::session_cookies)]
pub struct NewSessionCookie<'a> {
    pub username: &'a str,
    pub val: &'a str,
}
