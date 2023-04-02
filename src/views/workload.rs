// src/views/workload.rs

use yew::prelude::*;

use crate::components::header::Header;
use crate::components::navigation::Navigation;
use crate::components::footer::Footer;

#[function_component(WorkLoad)]
pub fn workload() -> Html {
    html! {
        <>
            <Header />
            <Navigation />
            <main>
                <section>
                    <article>
                        <h2>{ "Work Load" }</h2>
                        <p>{ "The current office work load..." }</p>
                    </article>
                </section>
            </main>
            <Footer />
        </>

    }
}