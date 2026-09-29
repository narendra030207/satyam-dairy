use dioxus::prelude::*;
#[derive(Debug, Clone, Routable, PartialEq)]
enum Route { #[route("/")] Home {} }
fn test() {
    let _ = dioxus::router::router_cfg::RouterConfig::default().history(dioxus::router::history::WebHashHistory::<Route>::new(false));
}
