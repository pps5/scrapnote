#![recursion_limit = "1024"]
mod component;
use component::editor::Editor;
use component::selector::Selector;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct ScrapNote {
    link: ComponentLink<Self>,
    focus: Focus,
    editing: Option<String>,
}

enum Msg {
    FileSelect(String),
    CommandSelect(String),
    FocusCommand,
}

impl Component for ScrapNote {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            focus: Focus::Command,
            editing: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FileSelect(f) => {
                log::info!("selected: {}", f);
                self.editing = Some(f);
                self.focus = Focus::Editor;
            }
            Msg::CommandSelect(_) => {}
            Msg::FocusCommand => {
                self.focus = Focus::Command;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div id="container">
                <Selector
                  on_file_select=self.link.callback(|f| Msg::FileSelect(f))
                  on_command_select=self.link.callback(|c| Msg::CommandSelect(c))
                  focus=self.focus == Focus::Command
                />
                { self.view_editor() }
            </div>
        }
    }
}

impl ScrapNote {
    fn view_editor(&self) -> Html {
        let on_unfocus = self.link.callback(|_| Msg::FocusCommand);
        match &self.editing {
            Some(s) => html! {
                <Editor
                  file_name=s
                  focus=self.focus == Focus::Editor
                  on_unfocus=on_unfocus
                />
            },
            None => html! {
                <Editor
                  focus=self.focus == Focus::Editor
                  on_unfocus=on_unfocus
                />
            },
        }
    }
}

#[derive(PartialEq)]
enum Focus {
    Command,
    Editor,
}

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    App::<ScrapNote>::new().mount_to_body();
}
