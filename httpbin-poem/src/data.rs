use poem::Route;

use crate::utils::RouteExt;

mod base64;

pub fn api(route: Route) -> Route {
    route.attach(base64::api)
}
