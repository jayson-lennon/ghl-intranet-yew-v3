// src/components/header.rs

use yew::prelude::*;

use crate::time::*;

#[function_component(Header)]
pub fn header() -> Html {
    let today = get_current_date();
    html! {
        <>
            <img src="GHL_Logo.jpg" alt="GHL Consultants Ltd Logo" />
            <h2>{ "GHL Intranet" }</h2>
            <h3>{ "Today is: " } { today }</h3>
        </>
    }
}