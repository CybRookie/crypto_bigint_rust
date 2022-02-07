use yew::prelude::*;
use yewtil::NeqAssign;
use web_sys::Window;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub route: Option<String>,
}

pub struct PageNotFound {
    props: Props,
}

impl Component for PageNotFound {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <section>
                <h1 class="title">
                { "The requested page was not found, error code: 404" }
                </h1>
                <h2 class="subtitle">
                { "This page does not seem to exist:" }
                </h2>
                <p>
                {display_incorrect_path(self.props.route.clone())}
                </p>
            </section>
        }
    }
}

fn display_incorrect_path(path: Option<String>) -> Html {
    match path {
        None => {
            return html! {
                <p>
                    { "An incorrect/empty path was provided." }
                </p>
            }
        },
        Some(path) => {
            return html! {
                <>
                    <p>
                        { web_sys::window().unwrap().location().href().unwrap_or("Incorrect link".to_string()) }
                    </p>
                    <h2>
                        { "This app state does not exist:"}
                    </h2>
                    <p>
                        { path.as_str() }
                    </p>
                </>
            }
        },
    }
}
