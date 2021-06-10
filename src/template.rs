use handlebars::{to_json, Handlebars};
use serde_json;
use serde_json::value::{Map, Value as Json};

pub fn to_html(name: &str, mut data: Map<String, Json>) -> String {
    let mut handlebars = Handlebars::new();

    /* 注册html模板 */
    handlebars
        .register_template_file(name, "src/views/".to_owned() + name)
        .unwrap_or_else(|e| println!("handlebars注册模板出错:{}", e));
    handlebars
        .register_template_file("frame.html", "src/views/frame.html")
        .unwrap_or_else(|e| println!("handlebars注册模板出错:{}", e));

    /* 传输数据给模板 */
    // let mut data = Map::new();
    data.insert("parent".to_string(), to_json("frame.html")); //必传,这个是插入父级的html
    data.insert("base_url".to_string(),to_json(crate::get_env("BASE_URL")));
                                                       
    let html = handlebars.render(name, &data).unwrap();
    html
}
