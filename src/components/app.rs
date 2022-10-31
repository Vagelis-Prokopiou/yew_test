use yew::prelude::*;
use super::counter::Counter;
use super::header::Header;

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();
    fn create(_ctx: &yew::html::Context<Self>) -> Self { Self }

    fn view(&self, ctx: &yew::html::Context<Self>) -> Html {
        html! {
             <>
                <div class="app">
                    <Header />
                    <div class="container">
                        <div class="sidebar"><p>{ "This is the sidebar" }</p></div>
                        <div class="main-content">
                            <p>{ "This is the main content" }</p>
                            <Counter />
                        </div>
                    </div>
                </div>
            </>
        }
    }
}