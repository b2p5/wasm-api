use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{JsFuture, spawn_local};
use web_sys::{Request, RequestInit, RequestMode, Response, window};
use serde::Deserialize;
use serde_wasm_bindgen::from_value;

#[derive(Deserialize)]
struct Fees {
    #[serde(rename = "fastestFee")]
    fastest_fee: u32,
    #[serde(rename = "halfHourFee")]
    half_hour_fee: u32,
    #[serde(rename = "hourFee")]
    hour_fee: u32,
    #[serde(rename = "economyFee")]
    economy_fee: u32,
    #[serde(rename = "minimumFee")]
    minimum_fee: u32,
}


// Esta función maneja la solicitud a la API y la actualización del DOM
async fn fetch_price_and_update_dom() -> Result<(), JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(
        "https://mempool.space/api/v1/fees/recommended",
        &opts,
    )?;

    let window = window().unwrap();
    let response = match JsFuture::from(window.fetch_with_request(&request)).await {
        Ok(res) => res,
        Err(e) => {
            log(&format!("Error fetching data: {:?}", e));
            return Err(e);
        }
    };

    // Hora actual en formato hora:minutos:segundos - dia/mes/año
    let date = js_sys::Date::new_0();
    let date = date.to_locale_string("es-ES", &JsValue::UNDEFINED).as_string().unwrap();
    
    let response: Response = response.dyn_into().unwrap();
    let fasstest_fee = JsFuture::from(response.json()?).await?;
    let fees: Fees = from_value(fasstest_fee).unwrap();

    let fastest_fee = fees.fastest_fee.to_string();  
    let half_hour_fee = fees.half_hour_fee.to_string(); 
    let hour_fee = fees.hour_fee.to_string();  
    let economy_fee = fees.economy_fee.to_string();  
    let minimum_fee = fees.minimum_fee.to_string();  
    
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    let p = document.create_element("p")?;
    p.set_inner_html(&format!("<b>Fecha:{}</b><br> fastest Fee:{}<br> halfHour Fee:  {}<br> hour Fee: {}<br> economy Fee: {}<br> minimum Fee: {}<hr><br>", 
                                date, fastest_fee, half_hour_fee, hour_fee, economy_fee, minimum_fee));


    body.prepend_with_node_1(&p)?;

    //log(&format!("fastestFee: {:?}", fasstest_fee));

    
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
        60000, // Intervalo de tiempo en milisegundos
    )?;
    closure.forget(); // Evita que la closure sea recolectada por el garbage collector

    Ok(())
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}




