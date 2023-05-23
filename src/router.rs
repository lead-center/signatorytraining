use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    home_page::HomePage, login_page::LoginPage, profile_page::ProfilePage, quiz_page::QuizPage,
    register_page::RegisterPage, signatory_training_page::SignatoryTrainingPage,
    signatory_training_page_2::SignatoryTrainingPage2,
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    HomePage,
    #[at("/register")]
    RegisterPage,
    #[at("/login")]
    LoginPage,
    #[at("/profile")]
    ProfilePage,
    #[at("/signatorytraining")]
    SignatoryTrainingPage,
    #[at("/signatorytraining2")]
    SignatoryTrainingPage2,
    #[at("/quiz")]
    QuizPage,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::HomePage => html! {<HomePage/> },
        Route::RegisterPage => html! {<RegisterPage/> },
        Route::LoginPage => html! {<LoginPage/> },
        Route::ProfilePage => html! {<ProfilePage/> },
        Route::SignatoryTrainingPage => html! {<SignatoryTrainingPage/> },
        Route::SignatoryTrainingPage2 => html! {<SignatoryTrainingPage2/> },
        Route::QuizPage => html! {<QuizPage/> },
    }
}
