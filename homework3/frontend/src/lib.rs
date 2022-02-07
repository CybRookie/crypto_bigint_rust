use yew::prelude::*;
use yew_router::{route::Route, switch::Permissive};
use nav::{Navbar, router::*};
use crate::pages::*;

pub mod nav;
pub mod pages;

pub struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    router_target: AppRoute,
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            router_target: AppRoute::Home,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Navbar/>
                <main>
                    <AppRouter
                        render=AppRouter::render(Self::switch)
                        redirect=AppRouter::redirect(|route: Route| {
                            // AppRoute::PageNotFound(Permissive(Some(route.route)))
                            AppRoute::Home
                        })
                    />
                </main>
                <noscript>
                    <p>
                        { "This website requires Javascript and WebAssembly for provision of its services, please, enable them in your browser, if you would like to continue." }
                    </p>
                </noscript>
            </>
        }
    }
}

impl Model {
    fn switch(switch: AppRoute) -> Html {
        match switch {
            AppRoute::Login => {
                html! { <pages::login::LoginPage /> }
            },
            AppRoute::PageNotFound(Permissive(route)) => {
                html! { <pages::not_found::PageNotFound route=route.clone() /> }
            },
            AppRoute::Home => {
                html! {
                    <pages::home::MainPage />
                    // <p>{"TEST"}</p>
                }
            },
        }
    }
}