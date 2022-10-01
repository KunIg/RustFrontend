use crate::example::ExamplePage;

use yew::prelude::*;

#[function_component(Index)]
pub fn index() -> Html {
    html! {
        <>
            <ExamplePage title="2Mplus">
                {"Pick an example on the left"}
            </ExamplePage>
        </>
    }
}
