use crate::session::Session;
use crate::template::to_html;
use handlebars::to_json;
use serde::Deserialize;
use serde_json::value::Map;
use warp::{Rejection, Reply};

type ResultWarp<T> = std::result::Result<T, Rejection>;

/* 输出注册表单 */
pub async fn signup_form() -> ResultWarp<impl Reply> {
    // let html = "模板返回html".to_string();
    // let html = register_model::register_html();

    let mut data = Map::new();
    data.insert("title".to_string(), to_json("title传过来的值"));
    let html = to_html("signup.html", data);

    let id = 32;
    if id != 0 {
        Ok(warp::reply::html(html))
    } else {
        Err(warp::reject::not_found())
    }
}

#[derive(Deserialize, Debug)]
pub struct SignupForm {
    pub email: String,
    pub mobile: String,
    pub username: String,
    // realname: String,
    pub password: String,
    pub is_student: bool,
    passwd: String,
}

impl SignupForm {
    fn validate(self) -> Result<Self, &'static str> {
        if self.username.is_empty() {
            Err("用户名不能为空")
        } else if self.username.len() < 2 {
            Err("用户名长度不能小于2")
        } else if self.password.is_empty() {
            Err("密码不能为空")
        } else if self.password.len() < 5 || self.passwd.len() < 5 {
            Err("密码与确认密码的长度不能小于5位")
        } else if self.password != self.passwd {
            Err("密码与确认密码必须一致")
        } else {
            Ok(self)
        }
    }
}

pub async fn do_signup(session: Session, form: SignupForm) -> ResultWarp<impl Reply> {
    let mut html = "处理注册表单".to_string();
    let email = &form.email.to_string();
    let mobile = &form.mobile.to_string();
    let username = &form.username.to_string();
    let is_student = &form.is_student;
    println!("是否进行学生认证：{:?}", is_student);

    let result = form.validate().map_err(|e| e.to_string());

    match result {
        Ok(form) => {
            println!("认证通过，插入用户表…");
            println!("{:?}", form);
            use crate::models::user_model::User;
            let insertID = User::insert_user(form, session.db());
            html = format!("插入的ID：{:?}", insertID);
        }
        Err(msg) => {
            // html = "注册表单填写没完束".to_string();
            let mut data = Map::new();
            data.insert("email".to_string(), to_json(email));
            data.insert("mobile".to_string(), to_json(mobile));
            data.insert("username".to_string(), to_json(username));
            data.insert("msg".to_string(), to_json(&msg));
            html = to_html("signup.html", data);
            println!("注册表单认证没通过：{}", msg);
        }
    };

    Ok(warp::reply::html(html))
}
