// src/views/techreferences.rs

use yew::prelude::*;

use crate::components::header::Header;
use crate::components::navigation::Navigation;
use crate::components::footer::Footer;

#[function_component(TechReferences)]
pub fn techreferences() -> Html {
    html! {
        <>
            <Header />
            <Navigation />
            <main>
                <section>
                    <article>
                        <h2>{ "Technical References" }</h2>
                        <p>{ "This is a collection of fire science, code, and other technical references..." }</p>
                    </article>
                </section>
            </main>
            <Footer />
        </>

    }
}