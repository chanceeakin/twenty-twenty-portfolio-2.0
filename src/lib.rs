#![recursion_limit = "1024"]

mod pages;
mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    utils::set_panic_hook();
    web_logger::init();
    yew::start_app::<App>();
    Ok(())
}

use yew::prelude::*;

use yew_router::{prelude::*, Switch};

use self::pages::{about::AboutModel, home::HomeModel};
use yew::virtual_dom::VNode;
use yew_router::switch::{AllowMissing, Permissive};

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> VNode {
        html! {
            <div>
                <nav class="menu",>
                    <RouterButton<AppRoute> route=AppRoute::A(AllowMissing(None))> {"Go to Home"} </RouterButton<AppRoute>>
                    <RouterButton<AppRoute> route=AppRoute::C> {"Go to About"} </RouterButton<AppRoute>>
                    <RouterButton<AppRoute> route=AppRoute::A(AllowMissing(Some(HomeRoute)))> {"Go to A/C"} </RouterButton<AppRoute>>
                    <RouterButton<AppRoute> route=AppRoute::E("there".to_string())> {"Go to E (hello there)"} </RouterButton<AppRoute>>
                    <RouterButton<AppRoute> route=AppRoute::E("world".to_string())> {"Go to E (hello world)"} </RouterButton<AppRoute>>
                    <RouterButton<AppRoute> route=AppRoute::PageNotFound(Permissive(Some("nope".to_string())))> {"Go to bad path"} </RouterButton<AppRoute>>
                </nav>
                <div>
                    <Router<AppRoute>
                        render = Router::render(|switch: AppRoute| {
                            match switch {
                                AppRoute::A(AllowMissing(route)) => html!{<HomeModel route = route />},
                                AppRoute::C => html!{<AboutModel/>},
                                AppRoute::E(string) => html!{format!("hello {}", string)},
                                AppRoute::PageNotFound(Permissive(None)) => html!{"Page not found"},
                                AppRoute::PageNotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found", missed_route)}
                            }
                        })
                        redirect = Router::redirect(|route: Route| {
                            AppRoute::PageNotFound(Permissive(Some(route.route)))
                        })
                    />
                </div>
            </div>
        }
    }
}

#[derive(Debug, Switch, Clone)]
pub enum AppRoute {
    #[to = "/a{*:inner}"]
    A(AllowMissing<HomeRoute>),
    #[to = "/c"]
    C,
    #[to = "/e/{string}"]
    E(String),
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
}

#[derive(Debug, Switch, PartialEq, Clone, Copy)]
#[to = "/c"]
pub struct HomeRoute;
