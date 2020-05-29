use yew::{prelude::*, virtual_dom::VNode, Properties};
use yew_router::components::RouterButton;

use crate::AppRoute;

pub struct NavBar;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

pub enum Msg {}

impl Component for NavBar {
  type Message = Msg;
  type Properties = Props;

  fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    NavBar
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    false
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    true
  }

  fn view(&self) -> VNode {
    html! {
      <nav class="flex fixed top-0 left-0 text-white m-4">
        <button class="py-1 px-2 border border-white rounded">
          <RouterButton<AppRoute> route=AppRoute::Home > {"Home"} </RouterButton<AppRoute>>
         </button>
        <button class="py-1 px-2 border border-white rounded mx-1">
          <RouterButton<AppRoute> route=AppRoute::About > {"About"} </RouterButton<AppRoute>>
        </button>
      </nav>
    }
  }
}
