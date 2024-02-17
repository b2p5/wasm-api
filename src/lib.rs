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
        //"http://numbersapi.com/random",
        "https://mempool.space/api/v1/fees/recommended",
        &opts,
    )?;

    let window = window().unwrap();
    let response = JsFuture::from(window.fetch_with_request(&request)).await?;
    let response: Response = response.dyn_into().unwrap();
    let text = JsFuture::from(response.text()?).await?.as_string().unwrap();  
    let fastest_fee = text.split("fastestFee").collect::<Vec<&str>>()[1];
    println!("fastestFee: {}", fastest_fee);
    let half_hour_fee = text.split("halfHourFee").collect::<Vec<&str>>()[1];
    println!("halfHourFee: {}", half_hour_fee);
    let hour_fee = text.split("hourFee").collect::<Vec<&str>>()[1];
    let economy_fee = text.split("economyFee").collect::<Vec<&str>>()[1];
    let minimum_fee = text.split("minimumFee").collect::<Vec<&str>>()[1];
    println!("minimumFee: {}", minimum_fee);


    let document = window.document().unwrap();
    let body = document.body().unwrap();

    // Verifica si el elemento <p> ya existe
    let existing_p = document.get_element_by_id("fees_info");

    // Si existe, actualiza su contenido. Si no, crea uno nuevo.
    let p = if let Some(p) = existing_p {
        p
    } else {
        let p = document.create_element("p")?;
        p.set_id("fees_info"); // Asigna un ID único
        body.prepend_with_node_1(&p)?;
        p
    };

    // Actualiza el contenido del elemento <p>
    p.set_inner_html(&format!(
        "fastest Fee: {}\n halfHour Fee: {}\n hour Fee: {}\n economy Fee: {}\n minimum Fee: {}\n\n",
        fastest_fee, half_hour_fee, hour_fee, economy_fee, minimum_fee
    ));

    // let document = window.document().unwrap();
    // let body = document.body().unwrap();
    // let p = document.create_element("p")?;
    // p.set_inner_html(&format!("fastest Fee:{}\n halfHour Fee:  {}\n hour Fee: {}\n economy Fee: {}\n minimum Fee: {}\n\n", fastest_fee, half_hour_fee, hour_fee, economy_fee, minimum_fee));
    
    // body.prepend_with_node_1(&p)?;
    
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
        30000, // Intervalo de tiempo en milisegundos
    )?;
    closure.forget(); // Evita que la closure sea recolectada por el garbage collector

    Ok(())
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}




