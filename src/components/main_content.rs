use yew::prelude::*;
use crate::js;
use super::counter::Counter;

pub struct MainContent;

impl Component for MainContent {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &yew::html::Context<Self>) -> Self { Self }

    fn view(&self, _ctx: &yew::html::Context<Self>) -> Html {
        let iso_date = js::lib_a::getIsoDate();
        let my_class = js::lib_a::MyClass::new(10);
        let my_class_number = my_class.number();
        my_class.set_number(20);
        let my_class_updated_number = my_class.number();
        let my_class_rendered = my_class.render();

        // lib_uuid module
        let uuid = js::lib_uuid::v4();

        html! {
            <div class="main-content-inner">
                <div style="background-color: red;">
                    <p>
                        {"The following date comes from js/lib_a.js through src/js/lib_a.rs:"}
                    </p>
                    <p>
                        {iso_date}
                    </p>
                </div>

                <div style="background-color: yellow;">
                    <p>
                        {"The following class comes from js/lib_a.js through src/js/lib_a.rs:"}
                    </p>
                    <p>
                        {"my_class_number: "} {my_class_number}
                    </p>
                    <p>
                        {"my_class_updated_number: "} {my_class_updated_number}
                    </p>
                    <p>
                        {"my_class_rendered: "} {my_class_rendered}
                    </p>
                </div>

                <div style="background-color: red;">
                    <p>
                        {"The following class comes from js/lib_uuid.js through src/js/lib_uuid.rs:"}
                    </p>
                    <p>
                        {"uuid: "} {uuid}
                    </p>
                </div>

                <div class="counter-container">
                    <Counter />
                </div>
            </div>
        }
    }
}