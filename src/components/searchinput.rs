// src/components/searchinput.rs

use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct SearchFieldProps {
    pub label: String,
    pub field_type: String,
    pub name: String,
    pub oninput: Callback<InputEvent>,
    pub value: String,
}

#[function_component(SearchField)]
pub fn search_field(props: &SearchFieldProps) -> Html {
    let SearchFieldProps {
        label,
        field_type,
        name,
        oninput,
        value,
    } = props;

    html! {
        <label for="cautious-input">
            { label }
            <input
                type={field_type.clone()}
                name={name.clone()}
                oninput={oninput}
                value={value.clone()}
            />
        </label>
    }
}

