use crate::components::header::Header;

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use crate::api::user_api::api_login_user;
use crate::components::{form_input::FormInput, loading_button::LoadingButton};
use crate::router::{self, Route};
use crate::store::{set_page_loading, set_show_alert, Store};

use gloo::net::http::Request;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::suspense::use_future;
use yew_router::prelude::*;
use yewdux::prelude::*;

// pub async fn get_article(name: &str) -> String {
//     let res = Request::get("/articles/first.md").send().await.unwrap();

//     let body = res.text().await.unwrap().to_string();

//     body.into()
// }

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

    let article = use_future(|| async {Request::get("/articles/first.md").send().await?.text().await});


    // let res = use_future(|| async { Request::new(URL).send().await?.text().await })?;
    let result_html = if let Ok(article) = article {
        match *article {
            Ok(ref res) => html! { res },
            Err(ref failure) => failure.to_string().into(),
        }
    } else {
        String::from("Err").into()
    };


    html! {
      <>
        <Header />
        <section class="bg-ct-blue-600 min-h-screen grid place-items-center">
            <div class="max-w-4xl min-h-fit mx-auto bg-ct-dark-100 rounded-md justify-center items-center p-8 space-y-5">
            <p class="text-3xl font-semibold underline">{"Mission & Values"}</p>
                {result_html}
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
