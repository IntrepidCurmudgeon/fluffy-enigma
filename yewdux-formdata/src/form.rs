use crate::stores::profile_store::ProfileStore;
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(MyForm)]
pub fn my_form() -> Html {
    let (profile, dispatch) = use_store::<ProfileStore>();
    let onsubmit = dispatch.reduce_mut_callback_with(|profile, event: FocusEvent| {
        event.prevent_default();
        let form: HtmlFormElement = event.target().unwrap().unchecked_into::<HtmlFormElement>();
        let data: FormData = FormData::new_with_form(&form).unwrap();
        let first_name = data.get("first").as_string().unwrap_or_default();
        let last_name = data.get("last").as_string().unwrap_or_default();
        profile.first_name = first_name;
        profile.last_name = last_name;
        gloo::console::log!("Profile Store:", profile.first_name.clone(), profile.last_name.clone());
    });
    html! {
        <div>
            <form onsubmit={onsubmit}>
                <h2>{"Profile"}</h2>
                <div>
                    <label for="first">{"First Name"}</label>
                    <input id="first" name="first" type="text" value={ profile.first_name.clone() } />
                </div>
                <div>
                    <label for="last">{"Last Name"}</label>
                    <input id="last" name="last" type="text" value={ profile.last_name.clone() }/>
                </div>
                <div>
                    <button>{"submit"}</button>
                </div>
            </form>
        </div>
    }
}