use crate::template::to_html;
use handlebars::to_json;
use serde_json::value::Map;
use warp::{Rejection, Reply};

type ResultWarp<T> = std::result::Result<T, Rejection>;

/* 输出注册表单 */
pub async fn login_form() -> ResultWarp<impl Reply> {
    // let html = "模板返回html".to_string();
    // let html = register_model::register_html();

    let mut data = Map::new();
    // data.insert("title".to_string(), to_json("title传过来的值"));
    let html = to_html("login.html", data);

    let id = 32;
    if id != 0 {
        Ok(warp::reply::html(html))
    } else {
        Err(warp::reject::not_found())
    }
}
