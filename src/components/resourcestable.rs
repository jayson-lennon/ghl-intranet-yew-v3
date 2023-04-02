// src/components/resourcestable.rs

use yew::prelude::*;

#[function_component(ResourcesTable)]
pub fn resourcestable() -> Html {
    html! {
        <table>
            <thead>
                <tr>
                    <td>{ "Organization" }</td>
                    <td>{ "Resource" }</td>
                    <td>{ "Link" }</td>
                    <td>{ "Username" }</td>
                    <td>{ "Password" }</td>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>{ "National Research Council" }</td>
                    <td>{ "National Model Codes" }</td>
                    <td><a href="https://nrc-publications.canada.ca/eng/search/?q=NRCCode&q=&q=&ps=50&s=dtp&m=1">{ "National Model Codes" }</a></td>
                    <td>{ "None" }</td>
                    <td>{ "None" }</td>
                </tr>
                <tr>
                    <td>{ "National Research Council" }</td>
                    <td>{ "NBC 2015 Intent Statements" }</td>
                    <td><a href="https://codes-guides.nrc.ca/IA/15NBC/intentframe.html">{ "NBC 2015 Intent Statements" }</a></td>
                    <td>{ "None" }</td>
                    <td>{ "None" }</td>
                </tr>
                <tr>
                    <td>{ "National Research Council" }</td>
                    <td>{ "NBC 1995 Users Guide" }</td>
                    <td><a href="https://nrc-publications.canada.ca/eng/view/ft/?id=f4c59aa5-16df-4b40-b9db-d6c65cae71de">{ "NBC 1995 Users Guide" }</a></td>
                    <td>{ "None" }</td>
                    <td>{ "None" }</td>
                </tr>
                <tr>
                    <td>{ "Province of British Columbia" }</td>
                    <td>{ "BC Building Code Resources" }</td>
                    <td><a href="https://www2.gov.bc.ca/gov/content/industry/construction-industry/building-codes-standards">{ "BC Building Code Resources" }</a></td>
                    <td>{ "None" }</td>
                    <td>{ "None" }</td>
                </tr>
                <tr>
                    <td>{ "Province of British Columbia" }</td>
                    <td>{ "BC Access Handbook 2020" }</td>
                    <td><a href="https://www2.gov.bc.ca/assets/gov/farming-natural-resources-and-industry/construction-industry/building-codes-and-standards/guides/2020_building_accessibility_handbook.pdf">{ "Building Access Handbook 2020" }</a></td>
                    <td>{ "None" }</td>
                    <td>{ "None" }</td>
                </tr>
                <tr>
                    <td>{ "Province of British Columbia" }</td>
                    <td>{ "BC Building Code Appeals" }</td>
                    <td><a href="https://www2.gov.bc.ca/gov/content/industry/construction-industry/building-codes-standards/building-code-appeal-board/building-code-appeal-board-decisions">{ "BC Building Code Appeals" }</a></td>
                    <td>{ "None" }</td>
                    <td>{ "None" }</td>
                </tr>
                <tr>
                    <td>{ "Building Officials Association of BC" }</td>
                    <td>{ "BC Building Code Interpretations" }</td>
                    <td><a href="https://boabc.org/interpretations-committee/#:~:text=The%20purpose%20of%20the%20BC%20Building%20Code%20Interpretation,To%20disseminate%20the%20completed%20interpretations%20to%20code%20users">{ "BC Building and Plumbing Code Appeals" }</a></td>
                    <td>{ "None" }</td>
                    <td>{ "None" }</td>
                </tr>
            </tbody>
        </table>
    }
}