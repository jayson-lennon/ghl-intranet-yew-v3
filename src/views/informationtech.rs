// src/views/informationtech.rs

use yew::prelude::*;

use crate::components::header::Header;
use crate::components::navigation::Navigation;
use crate::components::footer::Footer;

#[function_component(InformationTech)]
pub fn informationtech() -> Html {
    html! {
        <>
            <Header />
            <Navigation />
            <main>
                <section>
                    <article>
                        <h2>{ "Information Technology" }</h2>
                        <h3>{ "Nucleus Networks" }</h3>
                        <p>{ "GHL's IT provider is Nucleus Networks. They can be contacted by phone, email, or through the support app. The support app is installed on your desktop." }</p>
                        <p>{ "Phone: 604-682-3444" }</p>
                        <p>{ "Email: helpdesk@ghl.ca" }</p>
                    </article>
                </section>
            </main>
            <Footer />
        </>

    }
}