// src/views/lunchandlearn.rs

use yew::prelude::*;

use crate::components::header::Header;
use crate::components::navigation::Navigation;
use crate::components::footer::Footer;

#[function_component(LunchandLearn)]
pub fn lunchandlearn() -> Html {
    html! {
        <>
            <Header />
            <Navigation />
            <main>
                <section>
                    <article>
                        <h2>{ "Lunch and Learns" }</h2>
                        <p>{ "This is the lunch and learns section. Here you will find access to recent GHL Lunch and Learn sessions..." }</p>
                    </article>
                </section>
            </main>
            <Footer />
        </>

    }
}