// src/view/home.rs

use yew::prelude::*;

use crate::components::header::Header;
use crate::components::navigation::Navigation;
use crate::components::footer::Footer;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <Header />
            <Navigation />
            <main>
                <section>
                    <article>
                        <h2>{ "Welcome" }</h2>
                        <h3>{ "Today's News:" }</h3>
                        <p>{ "GHL FDS Guide now available from the Technical References Page."}</p>
                    </article>
                </section>
            </main>
            <Footer />
        </>
    }
}