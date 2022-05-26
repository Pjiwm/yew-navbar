use yew::prelude::*;
use yew_router::{history::History, prelude::RouterScopeExt};

use crate::router::Route;

pub struct OtherComponent;

impl Component for OtherComponent {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let history = ctx.link().history().unwrap();
        let onclick = Callback::once(move |_| history.push(Route::App));
        html! {
            <div class="container">
                <p>{ "This is a differnt component" }</p>
                <button {onclick}>{ "Go to home" }</button>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }
}
