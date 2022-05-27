use yew::prelude::*;
use yew_router::prelude::Link;

use crate::router::Route;

pub struct NavbarComponent;

impl Component for NavbarComponent {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <nav class="navbar navbar-expand-lg bg-primary">
              <div class="container-fluid">
                <a class="navbar-brand">{"Navbar"}</a>
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
                  <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="navbarSupportedContent">
                  <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                    <li class="nav-item">
                         <a class="nav-link"><Link<Route> to={Route::App}>{ "Home" }</Link<Route>></a>
                    </li>
                    <li class="nav-item">
                        <a class="nav-link"><Link<Route> to={Route::OtherComponent}>{ "Other page" }</Link<Route>></a>
                    </li>
                  </ul>
                  <form class="d-flex" role="search">
                    <input class="form-control me-2" type="search" placeholder="Search" aria-label="Search"/>
                    <button class="btn btn-dark" type="submit">{"Search"}</button>
                  </form>
                </div>
              </div>
            </nav>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }
}
