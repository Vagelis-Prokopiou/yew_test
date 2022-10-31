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
                <div class="main-content">
                    <Header />
                    <Counter />
                </div>
            </>
        }
    }
}