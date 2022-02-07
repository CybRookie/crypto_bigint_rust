// Definition of a navbar.

use yew::prelude::*;
use router::*;

pub mod router;

// Variants of messages that will signal a change of state,
// was the navbar toggled, when it was in a mobile compact form,
// did the user log in.
pub enum NavbarMsg {
    ToggleNavbar,
    IsLoggedIn,
}

pub struct Navbar {
    link: ComponentLink<Self>,
    navbar_active: bool,
    is_logged_in: bool,
}

impl Component for Navbar {
    type Message = NavbarMsg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            navbar_active: false,
            is_logged_in: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            NavbarMsg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            },
            NavbarMsg::IsLoggedIn => {
                false
            },
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <nav>
                <ul>
                    <li>
                        <AppAnchor route=AppRoute::Home>
                            { "Home" }
                        </AppAnchor>
                    </li>
                    <li>{ "Multi-DF" }</li>
                    <li>{ "History" }</li>
                </ul>
                <ul>
                    <li>{ "Register" }</li>
                    <li>
                        <AppAnchor route=AppRoute::Login>
                            { "Login" }
                        </AppAnchor>
                    </li>
                    <li>{ "Account" }</li>
                    <li>{ "Logout" }</li>
                </ul>
                <button onclick=self.link.callback(|_| NavbarMsg::ToggleNavbar)>
                    <span>{ "Menu" }</span>
                </button>
            </nav>
        }
    }
}
