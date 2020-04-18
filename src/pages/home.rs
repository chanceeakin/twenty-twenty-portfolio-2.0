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
            <div>
                <h1 class="text-xl text-gray-900 leading-tight">{"I am the Home component"}</h1>
            </div>
        }
    }
}
