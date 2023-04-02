// src/components/altsolnskb.rs

use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;

use yew::prelude::*;

use crate::components::searchinput::SearchField;

#[derive(Clone, PartialEq, Properties, Debug, Default, Serialize, Deserialize)]
pub struct SearchKeyword {
    pub keyword: String,
}

#[function_component(KeywordSearch)]
fn keyword_search() -> Html {
    let search_keyword = use_state(|| SearchKeyword::default());

    log::info!("search_keyword {:?}", search_keyword.clone());

    let onsubmit = {
        let search_keyword = search_keyword.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            log::info!("search_keyword {:?}", &search_keyword.clone());

            log::info!("search_keyword {:?}", &search_keyword);

            wasm_bindgen_futures::spawn_local(async move {
                let get_request = Request::get("http://ghl-van-app03:5050/altsolns")
                    .send()
                    .await
                    .unwrap();

                log::info!("get_request {:?}", &get_request);
            });
        })
    };

    let searchfield_oninput = {
        let search_keyword = search_keyword.clone();
        move |ev: InputEvent| {
            let input: HtmlInputElement = ev.target_unchecked_into();
            let value = input.value();
            search_keyword.set(SearchKeyword { keyword: value });
        }
    };

    let reset_onclick = {
        let search_keyword = search_keyword.clone();
        move |_| {
            search_keyword.set(SearchKeyword::default());
        }
    };

    html! {
        <>
            <form {onsubmit}>
                <SearchField
                    oninput={searchfield_oninput}
                    label={"Keyword: ".to_owned()}
                    name={"keyword".clone()}
                    field_type={"text".clone()}
                    value={search_keyword.keyword.clone()} />
                <button type="submit">{ "Search" }</button>
            </form>
            <br />
            <button onclick={reset_onclick}> { "Reset" }</button>
        </>
    }
}

#[function_component(AltSolnsKB)]
pub fn alt_solns_kb() -> Html {
    html! {
        <>
            <article>
                <h2>{ "Alternative Solutions Knowledge Base" }</h2>
                <p>{ "This is a searchable database of our alternative solutions." }</p>
                <p>{ "Enter a keyword such as: height, exit, glazing or distance" }</p>
                <p>{ "Alternatively, enter a code reference to search on." }</p>
                <p>{ "Click See Everything to see all the entries in the database." }</p>
            </article>
            <br />
            <KeywordSearch />
        </>
    }
}

