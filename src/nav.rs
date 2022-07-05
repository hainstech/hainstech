use crate::route::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <nav>
          <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
          <Link<Route> to={Route::Projects}>{ "Projects" }</Link<Route>>
        </nav>
    }
}
