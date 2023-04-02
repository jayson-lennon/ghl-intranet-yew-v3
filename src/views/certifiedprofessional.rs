// src/views/certifiedprofessional.rs

use yew::prelude::*;

use crate::components::header::Header;
use crate::components::navigation::Navigation;
use crate::components::footer::Footer;

#[function_component(CertifiedProfessional)]
pub fn certifiedprofessional() -> Html {
    html! {
        <>
            <Header />
            <Navigation />
            <main>
                <section>
                    <article>
                        <h2>{ "Certified Professional" }</h2>
                        <p>{ "This is the Certified Professional section..." }</p>
                    </article>
                </section>
            </main>
            <Footer />
        </>

    }
}