use yew_router::prelude::*;
use yew::prelude::*;
use crate::components::other_component::OtherComponent;
use crate::components::home_component::HomeComponent;
pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::App => html! {
            <HomeComponent/>
        },
        Route::OtherComponent => html! {
            <OtherComponent/>
        },
        Route::NotFound => html! {
            <div class="container">
                <p>{ "404" }</p>
            </div>
        },
    }
}
#[derive(Clone, Routable, PartialEq)]

pub enum Route {
    #[at("/")]
    App,
    #[at("/other")]
    OtherComponent,
    #[not_found]
    #[at("/404")]
    NotFound,
}