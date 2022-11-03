use yew::{function_component, html, Html};

mod form;
use form::MyForm;

mod stores;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <h1>{"Yewdux Form"}</h1>
            <MyForm />
        </main>
    }
}