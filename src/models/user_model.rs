use crate::db::PgPooledConnection;
use crate::handlers::signup_handler::SignupForm;
use crate::schema::users::dsl::*;
use bcrypt::{hash_with_salt, verify, DEFAULT_COST};
use core::str;
use diesel::prelude::*;
use log::error;

#[derive(Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub realname: String,
}

impl User {
    /* 身份认证 */
    pub fn authenticate(db: &PgPooledConnection, user: &str, passwd: &str) -> Option<Self> {
        use crate::schema::users::dsl::*;
        let (user, hash) = match users
            .filter(username.eq(user))
            .select(((id, username, realname), password))
            .first::<(User, String)>(db)
        {
            Ok((user, hash)) => (user, hash),
            Err(e) => {
                error!("读取用户 {:?} 信息出错: {:?}", user, e);
                return None;
            }
        };

        // let hashed=hash_with_salt(passwd, DEFAULT_COST, salt);
        /* 判断密码是否正确 */
        match verify(&passwd, &hash) {
            Ok(true) => Some(user),
            Ok(false) => None,
            Err(e) => {
                error!("密码认证失败：{:?}: {:?}", user, e);
                None
            }
        }
    }

    pub fn insert_user(newdata: SignupForm, conn: &PgPooledConnection) -> QueryResult<usize> {
        let insertID = diesel::insert_into(users)
            .values((
                username.eq(newdata.username),
                password.eq(newdata.password),
                realname.eq("888"),
            ))
            .execute(conn);
        insertID
    }
}
