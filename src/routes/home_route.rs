use crate::handlers::home_handler;
use crate::session::Session;
use warp::{filters::BoxedFilter, Filter};

pub fn index() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let home = warp::get()
        .and(warp::path::end())
        .and_then(home_handler::index);
    home
}
