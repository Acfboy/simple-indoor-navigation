pub mod guidance;
pub mod map;

use std::collections::HashMap;

struct Navigator {
    map: Map,
    route: Route,
}