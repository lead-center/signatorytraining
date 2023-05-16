use crate::components::header::Header;

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use crate::api::user_api::api_login_user;
use crate::components::{form_input::FormInput, loading_button::LoadingButton};
use crate::router::{self, Route};
use crate::store::{set_page_loading, set_show_alert, Store};

use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(SignatoryTrainingPage)]
pub fn signatory_training_page() -> Html {
  let navigator = use_navigator().unwrap();

  let module_1_button = {
    let navigator = navigator.clone();
    let onclick = Callback::from(move |_| navigator.push(&Route::SignatoryTrainingPage2));
    html! {
      <button
        type="button"
        class={format!(
          "w-full py-3 font-semibold rounded-lg outline-none border-none flex justify-center bg-ct-yellow-600"
        )}
        {onclick}
        style="float: right"
      >
        {"Module 1"}
      </button>
    }
  };

  html! {
    <>
      <Header />
      <section class="bg-ct-blue-600 min-h-screen grid place-items-center">
          <div class="max-w-4xl min-h-fit mx-auto bg-ct-dark-100 rounded-md h-[20rem] justify-center items-center p-8 space-y-5">
              <p class="text-3xl font-bold">{"Module 1"}</p>
              <p class="text-3xl font-semibold">{"Welcome to the LEAD Center"}</p>
              {"The LEAD (Leadership, Engagement, Advising, & Development) Center is UC Berkeley’s hub for student involvement, leadership development, and co-curricular advising. In this module, you will learn about the LEAD Center and how we can support you as a signatory."}
              <div class="text-right">
                <a href="#">
                  {next_button}
                </a>
              </div>
          </div>
      </section>
    </>
  }
}
