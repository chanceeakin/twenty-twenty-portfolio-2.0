use yew::{prelude::*, virtual_dom::VNode, Properties};

pub struct HomeHero;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

pub enum Msg {}

impl Component for HomeHero {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        HomeHero
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> VNode {
        html! {
          <>
            <h1 class="text-6xl text-white">{"Chance Eakin"}</h1>
            <h2 class="text-2xl text-white">{"Software Engineer"}</h2>
          </>
        }
    }
}
