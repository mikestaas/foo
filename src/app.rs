use boa_engine::Context;
use monaco::{
    api::{CodeEditorOptions, TextModel},
    sys::editor::BuiltinTheme,
    yew::CodeEditor,
};
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::suspense::use_future;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_change: Callback<String>,
}

const FOO: &str = "const hello = name => `hello ${name}!`;\nhello('foo');\n";

#[function_component]
pub fn EditorWithTheme(props: &Props) -> HtmlResult {
    let theme_handle = use_future(|| async { invoke("get_theme").await.as_string().unwrap() })?;
    let theme = if *theme_handle == "dark" {
        BuiltinTheme::VsDark
    } else {
        BuiltinTheme::Vs
    };
    let options = use_memo(
        |_| {
            CodeEditorOptions::default()
                .with_theme(theme.to_value().into())
                .with_automatic_layout(true)
                .to_sys_options()
        },
        (),
    );
    let model = use_memo(
        |_| TextModel::create(FOO, Some("javascript"), None).unwrap(),
        (),
    );
    let cb = use_callback(
        move |value, on_change| on_change.emit(value),
        props.on_change.clone(),
    );
    use_effect_with_deps(
        move |model| {
            let model2 = model.clone();
            let disposable = model.on_did_change_content(move |_| {
                cb.emit(model2.get_value());
            });

            move || drop(disposable)
        },
        model.clone(),
    );

    Ok(html! {
        <CodeEditor options={(*options).clone()} model={(*model).clone()} classes="editor" />
    })
}

fn eval(js_code: String) -> String {
    let mut context = Context::default();
    let js_val;
    match context.eval(js_code) {
        Ok(res) => js_val = res,
        Err(e) => js_val = e,
    };

    js_val.to_string(&mut context).unwrap().to_string()
}

#[function_component]
pub fn App() -> Html {
    let result = use_state(|| eval(FOO.into()));
    let on_change = use_callback(
        move |value, result| result.set(eval(value)),
        result.clone(),
    );

    html! {
        <Suspense>
            <EditorWithTheme on_change={on_change} />
            <pre class="result">{&*result}</pre>
        </Suspense>
    }
}
