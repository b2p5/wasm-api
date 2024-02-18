# Proyecto de Tarifas de Transacciones de Bitcoin con WebAssembly y Rust

En esta carpeta "docs" se encuentra todo lo necesario para llamar al script. 

El fichero index.html, desde el cual se llama a un fichero .css (en capteta /css) y al fichero .js en carpeta (/pack) el cual llama a un fichero en binario extensión .wasm que es el que contiene toda la lógica del proyecto debidamente compilada  con:


```bash
    wasm-pack build --target web
```


Se puede ejcutar este programma con: [https://b2p5.github.io/wasm-api/](https://b2p5.github.io/wasm-api/)



## Características

- Realiza solicitudes HTTP a la API `https://mempool.space/api/v1/fees/recommended`.
- Deserializa los datos JSON a estructuras Rust.
- Actualiza el DOM con las tarifas de transacciones de Bitcoin y la fecha y hora actuales.
- Utiliza WebAssembly para ejecutar Rust en el navegador.
- Configura intervalos para actualizar la información cada 60 segundos.


## Uso

Una vez que el servidor de desarrollo esté en funcionamiento y la página cargada, deberías ver las tarifas de transacción recomendadas actualizarse automáticamente cada 60 segundos, junto con la fecha y hora actuales.

## Contribuir

Las contribuciones son bienvenidas. Si tienes una sugerencia para mejorar, por favor, abre un issue o envía un pull request.

## Licencia

Este proyecto está licenciado bajo la Licencia MIT - vea el archivo `LICENSE.md` para detalles.

## Agradecimientos

- Gracias a Mempool Space por proporcionar la API utilizada en este proyecto.
- Este proyecto fue inspirado por el deseo de demostrar la interacción entre Rust, WebAssembly y la web moderna.
