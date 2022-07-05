use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <h1 class="homeTitle" >{ "Hainstech Homepage" }</h1>
            <p class="homeSubtitle" >{ "Made with Rust & Wasm (Blazingly fast 🚀 🥵)" }</p>
        </div>
    }
}
