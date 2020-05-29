use yew::{prelude::*, Children, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct WrapperProps {
    pub children: Children,
}

pub struct Wrapper {
    props: WrapperProps,
}

// #[link(wasm_import_module = "particlesjs")]
// extern "C" {
//     fn init(init: Init);
// }

pub enum Msg {}
impl Component for Wrapper {
    type Message = Msg;
    type Properties = WrapperProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Wrapper { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }
    fn view(&self) -> Html {
        html! {
        <div id="particle-mount" class="flex flex-col text-center bg-gray-900 justify-center items-center w-screen h-screen">
                { self.props.children.render() }
            </div>
        }
    }
}
