// src/app.rs

use yew::prelude::*;
use yew_router::prelude::*;

use crate::views::home::Home;
use crate::views::codeknowledge::CodeKnowledge;
use crate::views::masstimber::MassTimber;
use crate::views::certifiedprofessional::CertifiedProfessional;
use crate::views::techreferences::TechReferences;
use crate::views::workload::WorkLoad;
use crate::views::lunchandlearn::LunchandLearn;
use crate::views::informationtech::InformationTech;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/codeknowledge")]
    CodeKnowledge,
    #[at("/masstimber")]
    MassTimber,
    #[at("/certifiedprofessional")]
    CertifiedProfessional,
    #[at("/techreferences")]
    TechReferences,
    #[at("/workload")]
    WorkLoad,
    #[at("/lunchandlearn")]
    LunchandLearn,
    #[at("/informationtech")]
    InformationTech,
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::CodeKnowledge => html! { <CodeKnowledge />},
        Route::MassTimber => html! { <MassTimber />},
        Route::CertifiedProfessional => html! { <CertifiedProfessional /> },
        Route::TechReferences => html! { <TechReferences /> },
        Route::WorkLoad => html! { <WorkLoad /> },
        Route::LunchandLearn => html! { <LunchandLearn /> },
        Route::InformationTech => html! { <InformationTech />},
        Route::NotFound => html! { <h1>{ "404" }</h1> }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div id="fauxBody">
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </div>
    }
}