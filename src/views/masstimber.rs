// src/views/masstimber.rs

use yew::prelude::*;

use crate::components::header::Header;
use crate::components::navigation::Navigation;
use crate::components::footer::Footer;

#[function_component(MassTimber)]
pub fn masstimber() -> Html {
    html! {
        <>
            <Header />
            <Navigation />
            <main>
                <section>
                    <article>
                        <h2>{ "Mass Timber" }</h2>
                        <p>{ "This is the mass timber section..." }</p>
                    </article>
                </section>
            </main>
            <Footer />
        </>

    }
}