// Importación de módulos necesarios desde varias bibliotecas.
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{JsFuture, spawn_local};
use web_sys::{Request, RequestInit, RequestMode, Response, window};
use serde::Deserialize;
use serde_wasm_bindgen::from_value;
use js_sys::Date;

// Definición de una estructura Rust para mapear los datos JSON de las tarifas obtenidas de la API.
#[derive(Deserialize)]
struct Fees {
    #[serde(rename = "fastestFee")] fastest_fee: u32,
    #[serde(rename = "halfHourFee")] half_hour_fee: u32,
    #[serde(rename = "hourFee")] hour_fee: u32,
    #[serde(rename = "economyFee")] economy_fee: u32,
    #[serde(rename = "minimumFee")] minimum_fee: u32,
}

// Función asíncrona para realizar la solicitud HTTP a la API y actualizar el DOM con los resultados.
async fn fetch_price_and_update_dom() -> Result<(), JsValue> {
    // Configuración inicial para la solicitud HTTP.
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    // Creación de la solicitud HTTP.
    let request = Request::new_with_str_and_init(
        "https://mempool.space/api/v1/fees/recommended",
        &opts,
    )?;

    // Obtiene el objeto window para realizar la solicitud fetch.
    let window = window().unwrap();
    // Realiza la solicitud fetch y espera por la respuesta.
    let response = match JsFuture::from(window.fetch_with_request(&request)).await {
        Ok(res) => res,
        Err(e) => {
            log(&format!("Error fetching data: {:?}", e));
            return Err(e);
        }
    };

    // Formatea la fecha y hora actuales.
    let formatted_date_time = format_date_time();
    
    // Convierte la respuesta a un objeto Response.
    let response: Response = response.dyn_into().unwrap();
    // Obtiene el cuerpo de la respuesta como JSON y espera por el resultado.
    let fastest_fee = JsFuture::from(response.json()?).await?;
    // Deserializa el JSON a la estructura Fees.
    let fees: Fees = from_value(fastest_fee).unwrap();

    // Convierte las tarifas a cadenas de texto para su visualización.
    let fastest_fee = fees.fastest_fee.to_string();  
    let half_hour_fee = fees.half_hour_fee.to_string(); 
    let hour_fee = fees.hour_fee.to_string();  
    let economy_fee = fees.economy_fee.to_string();  
    let minimum_fee = fees.minimum_fee.to_string();  
    
    // Obtiene el documento y el cuerpo del documento para realizar modificaciones en el DOM.
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    // Crea un nuevo elemento <p> y establece su contenido HTML.
    let p = document.create_element("p")?;
    p.set_inner_html(&format!("<b>Fecha: {}</b><br> Fastest Fee: {}<br> Half Hour Fee: {}<br> Hour Fee: {}<br> Economy Fee: {}<br> Minimum Fee: {}<hr><br>", 
                                formatted_date_time, fastest_fee, half_hour_fee, hour_fee, economy_fee, minimum_fee));

    // Añade el nuevo elemento al principio del cuerpo del documento.
    body.prepend_with_node_1(&p)?;

    // Retorna Ok para indicar que la función se ejecutó sin errores.
    Ok(())
}

// Marcador de inicio para WebAssembly, se llama cuando se carga el módulo WASM.
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // Envuelve la función fetch_price_and_update_dom en una closure para ser ejecutada de forma asíncrona.
    let closure = Closure::wrap(Box::new(move || {
        spawn_local(async {
            if let Err(e) = fetch_price_and_update_dom().await {
                log(&format!("Error: {:?}", e));
            }
        });
    }) as Box<dyn Fn()>);

    // Configura un temporizador para llamar a la closure cada 60 segundos.
    window().unwrap().set_interval_with_callback_and_timeout_and_arguments_0(
        closure.as_ref().unchecked_ref(),
        60000, // Intervalo de tiempo en milisegundos.
    )?;
    closure.forget(); // Evita que la closure sea recolectada por el recolector de basura.

    Ok(())
}

// Función externa para permitir el registro en la consola desde Rust.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

// Función para formatear la fecha y hora actuales al formato "hora:minutos - día/mes/año".
fn format_date_time() -> String {
    let date = Date::new_0();

    let hours = date.get_hours();
    let minutes = date.get_minutes();
    let day = date.get_date();
    let month = date.get_month() + 1; // get_month retorna 0-11, por lo que sumamos 1 para tener 1-12.
    let year = date.get_full_year();

    // Formatea manualmente la fecha y hora al formato deseado.
    format!("{:02}:{:02} - {:02}/{:02}/{:04}", hours, minutes, day, month, year)
}
