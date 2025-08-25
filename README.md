# Agente de Monitoreo en Rust

![Rust Version](https://img.shields.io/badge/rust-1.79%2B-orange.svg)
![Licencia](https://img.shields.io/badge/licencia-MIT-blue.svg)

Componente de recolección de métricas para el proyecto de monitoreo de servidores de la facultad.

## 📖 Descripción General

Este repositorio contiene el código fuente del **Agente de Monitoreo**, una aplicación de consola escrita en Rust. Su principal responsabilidad es recolectar métricas del sistema (uso de CPU, RAM, etc.) de manera eficiente y enviarlas periódicamente a un servidor backend central para su almacenamiento, procesamiento y visualización en un dashboard.

Este agente está diseñado para ser ligero, de bajo consumo y portable, pudiendo compilarse para diferentes sistemas operativos.

## ✨ Características

* **Recolección de Métricas:** Obtiene información vital del sistema.
    * [✔️] Uso de CPU (%)
    * [✔️] Uso de Memoria RAM (MB)
    * [✔️] Timestamp de la recolección
* **Comunicación Eficiente:** Envía los datos recolectados a un endpoint del backend vía HTTP POST en formato JSON.
* **Configurable:** Permite ajustar parámetros clave a través de variables de entorno para adaptarse a diferentes entornos (desarrollo, producción).
* **Compilación Nativa y con Docker:** Ofrece un `Dockerfile` para crear builds consistentes y reproducibles, facilitando el despliegue.

## 🛠️ Prerrequisitos

Para compilar y ejecutar este proyecto localmente, necesitarás tener instalado:

* [Rust y Cargo](https://www.rust-lang.org/tools/install) (versión 1.79 o superior)
* [Docker](https://www.docker.com/get-started) (Opcional, para la compilación en contenedores)

## 🚀 Instalación y Puesta en Marcha

1.  **Clona el repositorio:**
    ```bash
    git clone [https://github.com/tu-usuario/tu-repositorio.git](https://github.com/tu-usuario/tu-repositorio.git)
    cd tu-repositorio
    ```

2.  **Instala las dependencias:**
    Cargo se encargará de descargar e instalar todas las dependencias definidas en `Cargo.toml` la primera vez que compiles o ejecutes el proyecto.

## ⚙️ Configuración

El agente se configura mediante variables de entorno. Puedes crear un archivo `.env` (y añadirlo a tu `.gitignore`) o exportarlas directamente en tu terminal.

| Variable                  | Descripción                                               | Por Defecto                 |
| ------------------------- | --------------------------------------------------------- | --------------------------- |
| `BACKEND_URL`             | La URL base del servidor backend al que se enviarán las métricas. | `http://localhost:8000`     |
| `METRICS_INTERVAL_SECS`   | El intervalo en segundos entre cada envío de métricas.   | `10`                        |
| `AGENT_ID`                | Un identificador único para el servidor donde corre el agente. | `servidor-ejemplo-01` |

**Ejemplo de ejecución con variables de entorno:**
```bash
export BACKEND_URL="[http://api.tuservidor.com](http://api.tuservidor.com)"
export METRICS_INTERVAL_SECS=30
cargo run
```

##  Running the application

### Ejecución en Modo Desarrollo

Para correr el agente en tu máquina local, utiliza:

```bash
cargo run
```

Esto compilará el proyecto en modo debug y lo ejecutará. Verás en la consola los logs de recolección y envío de métricas.

### Ejecución de Tests

Para verificar que toda la lógica funciona como se espera, puedes correr la suite de tests unitarios y de integración:

```bash
cargo test
```

## 📦 Compilación para Producción

El objetivo final es generar un binario ejecutable y optimizado para desplegar en los servidores a monitorear.

### 1. Compilación Nativa

Este es el método más directo. Generará un binario en la carpeta `target/release/`.

```bash
cargo build --release
```

El ejecutable se encontrará en: `target/release/agent`. Este es el archivo que debes copiar al servidor que quieres monitorear.

### 2. Compilación con Docker (Recomendado)

Usar Docker garantiza que el agente se compile siempre en el mismo entorno, evitando problemas de dependencias del sistema.

**Paso 1: Construir la imagen Docker**
Este comando usará el `Dockerfile` para compilar el agente.
```bash
docker build -t monitoring-agent .
```

**Paso 2: Extraer el binario de la imagen**
Una vez construida la imagen, puedes copiar el binario ejecutable a tu máquina local para su distribución.
```bash
docker create --name temp_agent monitoring-agent
docker cp temp_agent:/app/agent ./agent_binary
docker rm temp_agent
```

Ahora tendrás un archivo llamado `agent_binary` en tu directorio actual, listo para ser desplegado.

##  diagrama
### Diagrama de Arquitectura

Para una mejor comprensión de cómo interactúa el Agente con el resto del sistema, se puede consultar el siguiente diagrama:

![Diagrama de Clases del Proyecto](/agent/documents/diagrama de clase.png)
