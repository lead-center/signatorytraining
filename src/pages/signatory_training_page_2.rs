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

#[function_component(SignatoryTrainingPage2)]
pub fn signatory_training_page_2() -> Html {
  let navigator = use_navigator().unwrap();

  let prev_button = {
    let navigator = navigator.clone();
    let onclick = Callback::from(move |_| navigator.push(&Route::SignatoryTrainingPage));
    html! {
      <button
        type="button"
        class={format!(
          "w-full py-3 font-semibold rounded-lg outline-none border-none flex justify-center bg-ct-yellow-600"
        )}
        {onclick}
      >
        {"Previous"}
      </button>
    }
  };

  let next_button = {
    let navigator = navigator.clone();
    let onclick = Callback::from(move |_| navigator.push(&Route::QuizPage));
    html! {
      <button
        type="button"
        class={format!(
          "w-full py-3 font-semibold rounded-lg outline-none border-none flex justify-center bg-ct-yellow-600"
        )}
        {onclick}
      >
        {"Start Quiz"}
      </button>
    }
  };

  html! {
    <>
      <Header />
      <section class="bg-ct-blue-600 min-h-screen grid place-items-center">
          <div class="max-w-4xl min-h-fit mx-auto bg-ct-dark-100 rounded-md h-[20rem] justify-center items-center p-8 space-y-5">
          <p class="text-3xl font-semibold underline">{"Mission & Values"}</p>
              {"The LEAD Center’s mission is to support student-centered learning and development by cultivating student leadership. We advise and engage individuals, student organizations, student government, and various campus communities to enhance the co-curricular experience at UC Berkeley.

              Our values are centered on Inclusion, Equity, Sustainability, Excellence, Community, Growth, and Empowerment.
              
              We believe leadership is the continuous process of building authentic relationships, collectively taking bold action, and practicing intentional reflection to create positive change. Our mission is to guide the learning journey of students as they explore leadership and social change in their communities. Our definition of leadership informs our leadership approach."}
              <div class="text-left">
                <a href="#">
                  {prev_button}
                </a>
              </div>
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
