// src/views/codeknowledge.rs

use yew::prelude::*;

use crate::components::header::Header;
use crate::components::navigation::Navigation;
use crate::components::resourcestable::ResourcesTable;
use crate::components::altsolnskb::AltSolnsKB;
use crate::components::footer::Footer;

#[function_component(CodeKnowledge)]
pub fn codeknowledge() -> Html {
    html! {
        <>
            <Header />
            <Navigation />
            <main>
                <section>
                    <article>
                        <h2>{ "Code Knowledge Base" }</h2>
                        <p>{ "This is a collection of appeals, interpretations, and opinions, both internal and external." }</p>
                    </article>
                </section>
                <section>
                    <ResourcesTable />
                </section>
                <section>
                    <AltSolnsKB />
                </section>
            </main>
            <Footer />
        </>

    }
}