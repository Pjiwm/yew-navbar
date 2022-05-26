use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::{self};

#[function_component(Main)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<router::Route> render={Switch::render(router::switch)} />
        </BrowserRouter>
    }
}