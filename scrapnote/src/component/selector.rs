use wasm_bindgen::prelude::*;

use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use yew::events::KeyboardEvent;
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

use common::{GetFilesResponse, Item};

#[wasm_bindgen(module = "/js/list.js")]
extern "C" {
    fn scroll_list();
}

pub struct Selector {
    link: ComponentLink<Self>,
    props: Props,
    input_ref: NodeRef,
    fetch_task: Option<FetchTask>,
    state: State,
}

struct State {
    is_composing: bool,
    input: String,
    items: Vec<Item>,
    list_index: usize,
}

impl State {
    fn new() -> Self {
        Self {
            is_composing: false,
            input: String::new(),
            items: Vec::new(),
            list_index: 0,
        }
    }

    fn on_input(&mut self, input: String) {
        self.input = input;
    }

    fn on_composing_state_changed(&mut self, is_composing: bool) {
        self.is_composing = is_composing;
    }

    fn on_file_updated(&mut self, files: Vec<Item>) {
        self.items = files;
        if self.items.len() - 1 < self.list_index {
            self.list_index = self.items.len() - 1;
        }
    }
}

pub enum Msg {
    None(bool),
    Input(String),
    MoveCaretToFirst,
    MoveCaretToEnd,
    MoveSelectionUp,
    MoveSelectionDown,
    Enter,
    UpdateFiles(Vec<Item>),
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub on_file_select: Callback<String>,
    pub on_command_select: Callback<String>,
    pub focus: bool,
}

impl Component for Selector {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            input_ref: NodeRef::default(),
            fetch_task: None,
            state: State::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::None(is_composing) => {
                self.state.on_composing_state_changed(is_composing);
            }
            Msg::Input(input) => {
                if !(self.state.is_composing && input.is_empty()) {
                    self.query_files(&input);
                    self.state.on_input(input);
                }
            }
            Msg::MoveCaretToFirst => {
                self.move_caret(CaretPosition::Start);
            }
            Msg::MoveCaretToEnd => {
                self.move_caret(CaretPosition::End);
            }
            Msg::MoveSelectionUp => {
                self.state.list_index = self.state.list_index.saturating_sub(1);
                return true;
            }
            Msg::MoveSelectionDown => {
                if self.state.list_index + 1 < self.state.items.len() {
                    self.state.list_index += 1;
                }
                return true;
            }
            Msg::Enter => {
                match self.state.items.get(self.state.list_index) {
                    Some(f) => self.props.on_file_select.emit(f.name.to_owned()),
                    None => self.props.on_file_select.emit(self.state.input.to_owned()),
                };
            }
            Msg::UpdateFiles(files) => {
                self.state.on_file_updated(files);
                return true;
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        if self.props.focus {
            self.focus_input();
        }
        false
    }

    fn view(&self) -> Html {
        html! {
            <div id="command-wrapper">
            { self.view_input() }
            <div id="list">
              { for self.state.items.iter().enumerate().map(|(idx, i)| self.view_list_item(idx, i)) }
            </div>
            </div>
        }
    }

    #[allow(unused_unsafe)]
    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.focus_input();
            self.query_files("");
        }
        unsafe {
            scroll_list();
        }
    }
}

impl Selector {
    fn view_input(&self) -> Html {
        fn should_prevent_default(e: &KeyboardEvent) -> bool {
            e.ctrl_key() && e.key() == "a"
                || e.ctrl_key() && e.key() == "e"
                || e.ctrl_key() && e.key() == "p"
                || e.ctrl_key() && e.key() == "n"
        }
        html! {
              <input
                id="command"
                type="text"
                ref=self.input_ref.clone()
                oninput=self.link.callback(|e: InputData| Msg::Input(e.value))
                onkeydown=self.link.callback(|e: KeyboardEvent| {
                    if should_prevent_default(&e) {
                        Event::from(e.clone()).prevent_default();
                    }
                    let key: &str = &e.key();
                    match (e.ctrl_key(), key) {
                        (true, "a") => Msg::MoveCaretToFirst,
                        (true, "e") => Msg::MoveCaretToEnd,
                        (true, "p") => Msg::MoveSelectionUp,
                        (true, "n") => Msg::MoveSelectionDown,
                        (_, "Enter") => Msg::Enter,
                        (_, _) => Msg::None(e.is_composing())
                    }
                })
              />
        }
    }

    fn view_list_item(&self, index: usize, item: &Item) -> Html {
        let is_selected = self.state.list_index == index;
        html! {
            <div class=if is_selected { Some("selected") } else { None }>
              <div>{ &item.name }</div>
            </div>
        }
    }

    fn focus_input(&self) {
        self.input_ref
            .cast::<yew::web_sys::HtmlElement>()
            .expect("cast input_ref to HtmlInputElement")
            .focus()
            .expect("focus input");
    }

    fn move_caret(&self, target_caret_position: CaretPosition) {
        let input_element = self
            .input_ref
            .cast::<yew::web_sys::HtmlInputElement>()
            .expect("cast input_ref to HtmlInputElement");
        let pos = match target_caret_position {
            CaretPosition::Start => 0,
            CaretPosition::End => input_element.value().len() as u32,
        };
        input_element
            .set_selection_range(pos, pos)
            .expect(&format!("set selection range to ({pos}, {pos})", pos = pos));
    }

    fn query_files(&mut self, input: &str) {
        let port = yew::utils::window().location().port().unwrap();
        let query = if input.is_empty() {
            "".to_string()
        } else {
            format!(
                "?key={}",
                utf8_percent_encode(input, NON_ALPHANUMERIC).to_string()
            )
        };
        log::info!("http://127.0.0.1:{}/api/files{}", port, query);
        let request = Request::get(format!("http://127.0.0.1:{}/api/files{}", port, query))
            .body(yew::format::Nothing)
            .expect("build request to query files");
        log::info!("request: {:?}", request);

        let callback = self.link.callback(
            |response: Response<Json<Result<GetFilesResponse, anyhow::Error>>>| {
                let Json(data) = response.into_body();
                Msg::UpdateFiles(data.unwrap().files)
            },
        );
        drop(self.fetch_task.take());
        self.fetch_task = Some(FetchService::fetch(request, callback).expect("fetch files"));
    }
}

enum CaretPosition {
    Start,
    End,
}
