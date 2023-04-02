// src/components/navigation.rs

use yew::prelude::*;

#[function_component(Navigation)]
pub fn navigation() -> Html {
    html! {
        <>
            <nav>
                <ul>
                    <li><a href="/">{ "Home" }</a></li>
                    <li><a href="/codeknowledge">{ "Code Knowledge Base" }</a></li>
                    <li><a href="/masstimber">{ "Mass Timber" }</a></li>
                    <li><a href="/certifiedprofessional">{ "Certified Professional" }</a></li>
                    <li><a href="/techreferences">{ "Technical References" }</a></li>
                    <li><a href="/workload">{ "Work Load" }</a></li>
                    <li><a href="/lunchandlearn">{ "Lunch and Learns" }</a></li>
                    <li><a href="/informationtech">{ "Information Tech" }</a></li>
                </ul>
            </nav>
        </>
    }
}