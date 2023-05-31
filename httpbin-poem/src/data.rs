use poem::Route;

use crate::utils::RouteExt;

mod base64;
mod uuid;

pub fn api(route: Route) -> Route {
    route.attach(base64::api).attach(uuid::api)
}
