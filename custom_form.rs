/// This component was created by Blake Wilson for a group project by students of Florida State University
/// It is not intended to be used in any commercial or professional context and is released as is
/// under the terms of the GNU Public License version 3.0, or optionally, any later version
/// much of the code was taken directly from the excellent tutorial from:
/// Brooks Builds https://www.youtube.com/c/BrooksBuilds
/// Handling Form Submit Events - Introduction to Yew.rs
/// https://youtu.be/2JNw-ftN6js
/// https://github.com/brooks-builds/full-stack-todo-rust-course/blob/main/frontend/rust/yew/lessons/handling_form_submit_events/src/components/molecules/custom_form.rs
use crate::component::custom_button::CustomButton;
use crate::component::custom_radio::CustomRadio;
use crate::component::user_input::UserInput;
use serde::Serialize;
use std::ops::Deref;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Default, Clone, Serialize)]
pub struct UserDataStateHandle {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub contact_email: String,
    pub contact_phone: String,
    pub url: String,
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub accepting_patients: bool,
    pub telehealth: bool,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<UserDataStateHandle>,
}

/// TODO: in our custom form, we're repeating basically the same callback several times
/// there is probably a better way of doing this that doesn't involve all the boilerplate
/// and if not, there should be
#[function_component(CustomForm)]
pub fn custom_form(props: &Props) -> Html {
    let state = use_state(|| UserDataStateHandle::default());

    let cloned_state = state.clone();
    let email_changed = Callback::from(move |email| {
        let mut user_data = cloned_state.deref().clone();
        user_data.email = email;
        cloned_state.set(user_data);
    });

    let cloned_state = state.clone();
    let password_changed = Callback::from(move |password| {
        let mut user_data = cloned_state.deref().clone();
        user_data.password = password;
        cloned_state.set(user_data);
    });

    let cloned_state = state.clone();
    let first_changed = Callback::from(move |first_name| {
        let mut user_data = cloned_state.deref().clone();
        user_data.first_name = first_name;
        cloned_state.set(user_data);
    });

    let cloned_state = state.clone();
    let last_changed = Callback::from(move |last_name| {
        let mut user_data = cloned_state.deref().clone();
        user_data.last_name = last_name;
        cloned_state.set(user_data);
    });

    let cloned_state = state.clone();
    let contact_email_changed = Callback::from(move |contact_email| {
        let mut user_data = cloned_state.deref().clone();
        user_data.contact_email = contact_email;
        cloned_state.set(user_data);
    });

    let cloned_state = state.clone();
    let phone_changed = Callback::from(move |contact_phone| {
        let mut user_data = cloned_state.deref().clone();
        user_data.contact_phone = contact_phone;
        cloned_state.set(user_data);
    });

    let cloned_state = state.clone();
    let url_changed = Callback::from(move |url| {
        let mut user_data = cloned_state.deref().clone();
        user_data.url = url;
        cloned_state.set(user_data);
    });

    let cloned_state = state.clone();
    let street_changed = Callback::from(move |street| {
        let mut user_data = cloned_state.deref().clone();
        user_data.street = street;
        cloned_state.set(user_data);
    });

    let cloned_state = state.clone();
    let city_changed = Callback::from(move |city| {
        let mut user_data = cloned_state.deref().clone();
        user_data.city = city;
        cloned_state.set(user_data);
    });

    let cloned_state = state.clone();
    let political_state_changed = Callback::from(move |political_state| {
        let mut user_data = cloned_state.deref().clone();
        user_data.state = political_state;
        cloned_state.set(user_data);
    });

    let cloned_state = state.clone();
    let zipcode_changed = Callback::from(move |zipcode| {
        let mut user_data = cloned_state.deref().clone();
        user_data.zip = zipcode;
        cloned_state.set(user_data);
    });

    let cloned_state = state.clone();
    let accepting_changed = Callback::from(move |event: Event| {
        let value = event.target_unchecked_into::<HtmlInputElement>().value();
        let mut user_data = cloned_state.deref().clone();
        if value == "true" {
            user_data.accepting_patients = true;
        } else {
            user_data.accepting_patients = false;
        }
        cloned_state.set(user_data);
    });

    let cloned_state = state.clone();
    let telehealth_changed = Callback::from(move |event: Event| {
        let value = event.target_unchecked_into::<HtmlInputElement>().value();
        let mut user_data = cloned_state.deref().clone();
        if value == "true" {
            user_data.telehealth = true;
        } else {
            user_data.telehealth = false;
        }
        cloned_state.set(user_data);
    });

    let form_onsubmit = props.onsubmit.clone();
    let cloned_state = state.clone();
    let onsubmit = Callback::from(move |event: FocusEvent| {
        event.prevent_default();
        let data = cloned_state.deref().clone();
        form_onsubmit.emit(data);
    });

    html! {
        <form onsubmit={onsubmit}>
            <UserInput name="email" handle_onchange={email_changed} />
            <UserInput name="password" handle_onchange={password_changed} />
            <UserInput name="first_name" handle_onchange={first_changed} />
            <UserInput name="last_name" handle_onchange={last_changed} />
            <UserInput name="contact_email" handle_onchange={contact_email_changed} />
            <UserInput name="contact_phone" handle_onchange={phone_changed} />
            <UserInput name="url" handle_onchange={url_changed} />
            <UserInput name="street" handle_onchange={street_changed} />
            <UserInput name="city" handle_onchange={city_changed} />
            <UserInput name="state" handle_onchange={political_state_changed} />
            <UserInput name="zipcode" handle_onchange={zipcode_changed} />
            <div>
                <h4>{"Are you currently accepting new clients?"}</h4>
                <input type="radio" id="accepting_true" name="accepting" onchange={accepting_changed.clone()} value="true" />
                <label for="accepting_true">{"Yes"}</label>
                <input type="radio" id="accepting_false" name="accepting" onchange={accepting_changed.clone()} value="false" checked=true />
                <label for="accepting_false">{"No"}</label>
            </div>
            <div>
                <h4>{"Do you offer services remotely via telehealth?"}</h4>
                <input type="radio" id="telehealth_true" name="telehealth" onchange={telehealth_changed.clone()} value="true" />
                <label for="telehealth_true">{"Yes"}</label>
                <input type="radio" id="telehealth_false" name="telehealth" onchange={telehealth_changed.clone()} value="false" checked=true />
                <label for="telehealth_false">{"No"}</label>
            </div>
            <CustomButton label="Publish" />
        </form>
    }
}
