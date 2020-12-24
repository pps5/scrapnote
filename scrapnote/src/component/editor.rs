use wasm_bindgen::prelude::*;

use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

use common::{GetFileContentResponse, SaveFileContentRequest};

#[wasm_bindgen(module = "/js/ace.js")]
extern "C" {
    fn get_value() -> String;
    fn set_value(value: String);
    fn focus();
    fn set_editable(editable: bool);
}

pub struct Editor {
    link: ComponentLink<Self>,
    props: Props,
    fetch_task: Option<FetchTask>,
    save_task: Option<FetchTask>,
    editor_ref: NodeRef,
}

pub enum Msg {
    Save,
    Unfocus,
    ContentLoaded(String),
    None,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or(None)]
    pub file_name: Option<String>,
    pub focus: bool,
    pub on_unfocus: Callback<()>,
}

impl Component for Editor {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            fetch_task: None,
            save_task: None,
            editor_ref: NodeRef::default(),
        }
    }

    #[allow(unused_unsafe)]
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Save => match &self.props.file_name.to_owned() {
                Some(f) => unsafe { self.save_content(f, get_value()) },
                None => self.link.send_message(Msg::Unfocus),
            },
            Msg::Unfocus => {
                self.props.on_unfocus.emit(());
            }
            Msg::ContentLoaded(c) => unsafe {
                set_value(c);
                set_editable(true);
                if self.props.focus {
                    focus();
                }
            },
            Msg::None => {}
        }
        false
    }

    #[allow(unused_unsafe)]
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if let Some(ref f) = props.file_name {
            self.get_contents(&f);
        }
        self.props = props;
        false
    }

    fn view(&self) -> Html {
        html! {
            <div
              ref=self.editor_ref.clone()
              id="editor"
              onkeypress=self.link.callback(|e: KeyboardEvent| {
                  match e.key().as_ref() {
                    "Escape" => Msg::Save,
                    _ => Msg::None
                  }
              })
            />
        }
    }
}

impl Editor {
    fn get_contents(&mut self, file_name: &str) {
        let port = yew::utils::window().location().port();
        let request = Request::get(format!(
            "http://127.0.0.1:{}/api/file/{}",
            port.unwrap(),
            file_name
        ))
        .body(yew::format::Nothing)
        .unwrap();

        let callback = self.link.callback(
            |response: Response<Json<Result<GetFileContentResponse, anyhow::Error>>>| {
                let Json(data) = response.into_body();
                if let Ok(data) = data {
                    Msg::ContentLoaded(data.content)
                } else {
                    Msg::None
                }
            },
        );
        drop(self.fetch_task.take());
        self.fetch_task =
            Some(FetchService::fetch(request, callback).expect("fetch file contents"));
    }

    fn save_content(&mut self, file_name: &str, content: String) {
        let port = yew::utils::window().location().port();
        let body = SaveFileContentRequest { content };
        let request = Request::post(format!(
            "http://127.0.0.1:{}/api/file/{}",
            port.unwrap(),
            file_name
        ))
        .body(Json(&body))
        .unwrap();

        let callback = self
            .link
            .callback(|_: Response<Result<String, anyhow::Error>>| Msg::Unfocus);
        drop(self.save_task.take());
        self.save_task = Some(FetchService::fetch(request, callback).expect("save content"));
    }
}
