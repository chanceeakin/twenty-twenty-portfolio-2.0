use crate::{AppRoute, HomeRoute};
use yew::{prelude::*, virtual_dom::VNode, Properties};
use yew_router::{prelude::*, switch::AllowMissing};

pub struct HomeModel {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub route: Option<HomeRoute>,
}

pub enum Msg {}

impl Component for HomeModel {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        HomeModel { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> VNode {
        html! {
            <div>
                { "I am the Home component"}
                <div>
                    <RouterButton<AppRoute>
                        route=AppRoute::A(AllowMissing(Some(HomeRoute)))
                    />
                </div>
            </div>
        }
    }
}
