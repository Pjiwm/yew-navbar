use yew::prelude::*;
use yew_router::{history::History, prelude::RouterScopeExt};

use crate::router::Route;

pub struct NavbarComponent;

impl Component for NavbarComponent {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let history = ctx.link().history().unwrap();
        let _onclick: Callback<MouseEvent> = Callback::once(move |_| history.push(Route::OtherComponent));
        html! {
            <nav class="navbar navbar-expand-lg bg-light">
              <div class="container-fluid">
                <a class="navbar-brand" href="#">{"Navbar"}</a>
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
                  <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="navbarSupportedContent">
                  <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                    <li class="nav-item">
                      <a class="nav-link active" aria-current="page" href="#">{"Home"}</a>
                    </li>
                    <li class="nav-item">
                      <a class="nav-link" href="#">{"Link"}</a>
                    </li>
                    <li class="nav-item dropdown">
                      <a class="nav-link dropdown-toggle" href="#" id="navbarDropdown" role="button" data-bs-toggle="dropdown" aria-expanded="false">
                        {"Dropdown"}
                      </a>
                      <ul class="dropdown-menu" aria-labelledby="navbarDropdown">
                        <li><a class="dropdown-item" href="#">{"Action"}</a></li>
                        <li><a class="dropdown-item" href="#">{"Another action"}</a></li>
                        <li><hr class="dropdown-divider"/></li>
                        <li><a class="dropdown-item" href="#">{"Something else here"}</a></li>
                      </ul>
                    </li>
                    <li class="nav-item">
                      <a class="nav-link disabled">{"Disabled"}</a>
                    </li>
                  </ul>
                  <form class="d-flex" role="search">
                    <input class="form-control me-2" type="search" placeholder="Search" aria-label="Search"/>
                    <button class="btn btn-outline-success" type="submit">{"Search"}</button>
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