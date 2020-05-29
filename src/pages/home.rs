use crate::components::wrapper::Wrapper;
use yew::{prelude::*, virtual_dom::VNode, Properties};

pub struct HomeModel;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

pub enum Msg {}

impl Component for HomeModel {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        HomeModel
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> VNode {
        html! {
            <Wrapper>
                <h1 class="text-4xl text-white">{"Chance Eakin"}</h1>
                <h2 class="text-xl text-white">{"Software Engineer"}</h2>
            </Wrapper>
        }
    }
}
