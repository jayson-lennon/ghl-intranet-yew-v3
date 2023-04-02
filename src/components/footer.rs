// src/components/footer.rs

use yew::prelude::*;

use crate::time::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    let year = get_current_year();
    html! {
        <>
            <footer>
                <section>
                    <p>{ "800 - 700 W Pender St, Vancouver, BC V6C 1G8" }</p>
                    <p>{ "\u{00A9} " } {year} { " GHL Consultants Ltd | All Rights Reserved" }</p>
                </section>
                <section>
                    <a href="https://www.ghl.ca">{ "GHL Web Site" }</a>
                    <a href="https://www.google.com">{ "Google Search" }</a>
                    <a href="https://ghl.egnyte.com">{ "GHL Egnyte Web Portal" }</a>
                    <a href="https://portal.office.com">{ "Microsoft 365 Portal" }</a>
                    <a href="security.microsoft.com/quarantine">{ "Email Quarantine Viewer" }</a>
                </section>
            </footer>
        </>
    }
}