use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{JsFuture, spawn_local};
use web_sys::{Request, RequestInit, RequestMode, Response, window};

// Esta función maneja la solicitud a la API y la actualización del DOM
async fn fetch_price_and_update_dom() -> Result<(), JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(
        "http://numbersapi.com/random",
        &opts,
    )?;

    let window = window().unwrap();
    let response = JsFuture::from(window.fetch_with_request(&request)).await?;
    let response: Response = response.dyn_into().unwrap();
    let text = JsFuture::from(response.text()?).await?.as_string().unwrap();  
    let text_1 = text.split(" is").collect::<Vec<&str>>()[0];
    let text_2 = text.split(" is").collect::<Vec<&str>>()[1];

    let document = window.document().unwrap();
    let body = document.body().unwrap();
    let p = document.create_element("p")?;
    p.set_inner_html(&format!("{} = {}", text_1, text_2));
    
    body.prepend_with_node_1(&p)?;
    
    Ok(())

}


#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let closure = Closure::wrap(Box::new(move || {
        spawn_local(async {
            if let Err(e) = fetch_price_and_update_dom().await {
                log(&format!("Error: {:?}", e));

            }
        });
    }) as Box<dyn Fn()>);

    window().unwrap().set_interval_with_callback_and_timeout_and_arguments_0(
        closure.as_ref().unchecked_ref(),
        5000, // Intervalo de tiempo en milisegundos
    )?;
    closure.forget(); // Evita que la closure sea recolectada por el garbage collector

    Ok(())
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}




