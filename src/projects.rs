use reqwasm::http::Request;
use serde::Deserialize;
use yew::prelude::*;

#[derive(Clone, PartialEq, Deserialize, Debug)]
struct Project {
    name: String,
    description: Option<String>,
    language: Option<String>,
    url: String,
}

#[function_component(Projects)]
pub fn projects() -> Html {
    let current_projects = use_state(|| Vec::<Project>::new());

    {
        let current_projects = current_projects.clone();
        use_effect_with_deps(
            move |_| {
                log::info!("Fetching projects");
                wasm_bindgen_futures::spawn_local(async move {
                    let projects: Vec<Project> =
                        Request::get("https://api.github.com/orgs/Hainstech/repos")
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    current_projects.set(projects);
                });
                || ()
            },
            (),
        );
    }

    let projects_list: Html = current_projects
        .iter()
        .map(|project| {
            let project = project.clone();
            html! {
                <div class="projectCard">
                    <div class="projectCardHeader">
                        <h3 class="projectCardTitle">{ &project.name }</h3>
                    </div>
                    <div class="projectCardBody">
                        <div class="projectCardDescription">
                        if let Some(description) = &project.description {
                            <p>{ description }</p>
                        }
                        if let Some(language) = &project.language {
                            <p>{"made with "}{ language }</p>
                        }
                        </div>
                    </div>
                    <div class="projectCardFooter">
                        <a class="projectCardLink" href={ project.url.to_owned() }>{ project.url }</a>
                    </div>
                </div>
            }
        })
        .collect();

    html! {
        <div>
            <h1>{ "Projects" }</h1>
            { projects_list }
        </div>
    }
}
