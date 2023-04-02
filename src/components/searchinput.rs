// src/components/searchinput.rs

use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct SearchFieldProps {
    pub label: String,
    pub field_type: String,
    pub name: String,
    pub input_node_ref: NodeRef,
}

#[function_component(SearchField)]
pub fn search_field(props: &SearchFieldProps) -> Html {
    let SearchFieldProps {
        label,
        field_type,
        name,
        input_node_ref,
    } = props;

    html! {
        <label for="cautious-input">
            { label }
            <input
                type={field_type.clone()}
                name={name.clone()}
                ref={input_node_ref.clone()}
            />
        </label>
    }
}