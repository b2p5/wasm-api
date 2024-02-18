# Proyecto de Tarifas de Transacciones de Bitcoin con WebAssembly y Rust

Este proyecto utiliza Rust y WebAssembly para crear una aplicación web que realiza solicitudes a la API de Mempool Space para obtener las tarifas recomendadas de transacciones de Bitcoin. La información se deserializa y se muestra en el DOM de una página web, actualizándose periódicamente.

## Características

- Realiza solicitudes HTTP a la API `https://mempool.space/api/v1/fees/recommended`.
- Deserializa los datos JSON a estructuras Rust.
- Actualiza el DOM con las tarifas de transacciones de Bitcoin y la fecha y hora actuales.
- Utiliza WebAssembly para ejecutar Rust en el navegador.
- Configura intervalos para actualizar la información cada 60 segundos.

## Requisitos Previos

Para utilizar este proyecto, necesitas tener instalado lo siguiente:

- Rust y `cargo`
- `wasm-pack` para compilar a WebAssembly
- `npm` y Node.js para el servidor de desarrollo y dependencias de JavaScript
- Un navegador moderno que soporte WebAssembly

## Configuración del Proyecto

1. **Clona el Repositorio**

    ```bash
    git clone <url-del-repositorio>
    cd <nombre-del-repositorio>
    ```

2. **Construye el Proyecto con `wasm-pack`**

    ```bash
    wasm-pack build --target web
    ```

    Esto compila el código Rust a WebAssembly, generando los archivos necesarios en la carpeta `pkg`.

3. **Instala Dependencias de JavaScript**

    Desde la carpeta que contiene tu archivo `index.html` y tu script de JavaScript que carga el módulo wasm:

    ```bash
    npm install
    ```

4. **Ejecuta el Servidor de Desarrollo**

    Puedes usar cualquier servidor HTTP. Aquí hay un ejemplo usando `http-server` de npm:

    ```bash
    npx http-server .
    ```

    Visita `http://localhost:8080` en tu navegador para ver la aplicación.

    o puedes ejecutar

    ```bash
    python3 -m http.server
    ```
    Visitar `http://0.0.0.0:8000/` en tu navegador para ver la aplicación. 

    También puedes ver el funcionamiento del script en:

    `https://b2p5.github.io/wasm-api/`


## Uso

Una vez que el servidor de desarrollo esté en funcionamiento y la página cargada, deberías ver las tarifas de transacción recomendadas actualizarse automáticamente cada 60 segundos, junto con la fecha y hora actuales.

## Contribuir

Las contribuciones son bienvenidas. Si tienes una sugerencia para mejorar, por favor, abre un issue o envía un pull request.

## Licencia

Este proyecto está licenciado bajo la Licencia MIT - vea el archivo `LICENSE.md` para detalles.

## Agradecimientos

- Gracias a Mempool Space por proporcionar la API utilizada en este proyecto.
- Este proyecto fue inspirado por el deseo de demostrar la interacción entre Rust, WebAssembly y la web moderna.
