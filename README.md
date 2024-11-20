# Instrucciones de Uso

## Codificar un archivo en fotogramas
```bash
mkdir fotogramas
rm fotogramas/*
cargo run --bin encode archivo.zip fotogramas
cargo run --bin decode fotogramas archivo_recuperado.zip



```bash
cargo run --bin encode <archivo_original> <carpeta_salida>

```bash
cargo run --bin encode archivo.zip fotogramas

```bash
cargo run --bin decode <carpeta> <archivo_recuperado>

```bash
cargo run --bin decode fotogramas archivo_recuperado.zip


## Codificar un archivo en vídeo

```bash
mkdir fotogramas
rm -r fotogramas/*
cargo run --bin video_generator fotogramas output_video.mp4
cargo run --bin video_decoder output_video.mp4 fotogramas archivo_recuperado.zip


```bash
cargo run --bin video_generator <carpeta_fotogramas> <video_salida>


```bash
cargo run --bin video_generator fotogramas output_video.mp4


```bash
cargo run --bin video_decoder <video_entrada> <carpeta_fotogramas> <archivo_salida>

```bash
cargo run --bin video_decoder output_video.mp4 fotogramas_recuperados archivo_recuperado.zip

