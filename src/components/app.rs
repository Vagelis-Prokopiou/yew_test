use yew::prelude::*;
use super::header::Header;
use super::main_content::MainContent;

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();
    fn create(_ctx: &yew::html::Context<Self>) -> Self { Self }

    fn view(&self, _ctx: &yew::html::Context<Self>) -> Html {
        html! {
             <>
                <div class="app">
                    <Header />
                    <div class="container">
                        <div class="sidebar"><p>{ "This is the sidebar" }</p></div>
                        <div class="main-content">
                            <MainContent />
                        </div>
                    </div>
                </div>
            </>
        }
    }
}