use crate::components::wrapper::Wrapper;
use yew::{prelude::*, virtual_dom::VNode, Properties};

pub struct AboutModel;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

pub enum Msg {}

impl Component for AboutModel {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        AboutModel
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
                <h1 class="text-xl text-white">{"About me."}</h1>
            </Wrapper>
        }
    }
}
