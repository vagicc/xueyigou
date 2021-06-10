use crate::routes::admin_route;
use crate::routes::home_route;
use crate::routes::link_route;
use crate::routes::login_route;
use crate::routes::signup_route;
use crate::session::create_session_filter;
use warp::Filter;

pub fn all_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let pgsession = create_session_filter();
    // let s = move || pgsession.clone();

    let dir = warp::path("static").and(warp::fs::dir("./static"));

    let home = home_route::index();

    let login = login_route::login();

    let signup = signup_route::signup().or(signup_route::do_signup(pgsession.clone()));

    let admin = admin_route::add_admin();

    let link = link_route::test();

    let routes = home.or(dir).or(login).or(signup).or(admin).or(link);
    routes
}
