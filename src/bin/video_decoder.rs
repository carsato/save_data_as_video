use std::fs;
use std::fs::File;
use std::io::Write;
use std::env;
use std::process::Command;
use image::open;

const MACRO_PIXEL_SIZE: u32 = 10; // Tamaño del macropíxel (bloque)
const FRAME_WIDTH: u32 = 640;    // Ancho del fotograma
const FRAME_HEIGHT: u32 = 480;   // Alto del fotograma

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Uso: {} <video_entrada> <carpeta_fotogramas> <archivo_salida>", args[0]);
        return;
    }

    let input_video = &args[1];
    let frames_folder = &args[2];
    let output_file = &args[3];

    // Crear carpeta para los fotogramas si no existe
    if let Err(e) = fs::create_dir_all(frames_folder) {
        eprintln!("Error al crear la carpeta de fotogramas: {}", e);
        return;
    }

    // Extraer fotogramas del vídeo
    extract_frames_from_video(input_video, frames_folder);

    // Decodificar los datos de los fotogramas
    match decode_data_from_frames(frames_folder) {
        Ok(data) => {
            // Escribir los datos en un archivo
            write_bytes_to_file(output_file, &data);
            println!("Archivo reconstruido: {}", output_file);
        }
        Err(e) => {
            // Manejar el error
            eprintln!("Error al decodificar los fotogramas: {}", e);
        }
    }
}

fn extract_frames_from_video(input_video: &str, frames_folder: &str) {
    let ffmpeg_command = format!(
        "ffmpeg -i {} -start_number 0 {}/frame_%04d.png",
        input_video, frames_folder
    );

    println!("Ejecutando: {}", ffmpeg_command);

    let status = Command::new("sh")
        .arg("-c")
        .arg(&ffmpeg_command)
        .status();

    if let Err(e) = status {
        eprintln!("Error al extraer fotogramas: {}", e);
        std::process::exit(1);
    }
}

fn decode_data_from_frames(folder: &str) -> Result<Vec<u8>, String> {
    let mut bits = Vec::new();

    // Obtener y ordenar los archivos de la carpeta
    let mut entries: Vec<_> = std::fs::read_dir(folder)
        .map_err(|e| format!("Error leyendo la carpeta: {}", e))?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("png"))
        .collect();

    entries.sort(); // Ordenar los archivos por nombre

    for path in entries {
        let img = open(&path)
            .map_err(|e| format!("Error abriendo el archivo {}: {}", path.display(), e))?
            .into_rgb8();

        // Validar dimensiones del fotograma
        if img.width() != FRAME_WIDTH || img.height() != FRAME_HEIGHT {
            return Err(format!("Dimensiones incorrectas en el fotograma {}", path.display()));
        }

        for row in (0..FRAME_HEIGHT).step_by(MACRO_PIXEL_SIZE as usize) {
            for col in (0..FRAME_WIDTH).step_by(MACRO_PIXEL_SIZE as usize) {
                let pixel = img.get_pixel(col, row);
                if pixel[0] > 128 {
                    bits.push(1); // Blanco
                } else {
                    bits.push(0); // Negro
                }
            }
        }
    }

    // Leer el tamaño original del archivo desde los primeros 32 bits
    let size = {
        let mut size_bits = bits.drain(0..32).collect::<Vec<u8>>();
        let mut size = 0u32;
        for bit in size_bits.drain(..) {
            size = (size << 1) | (bit as u32);
        }
        size as usize
    };

    // Convertir los bits en bytes, respetando el tamaño original
    let mut bytes = Vec::new();
    for chunk in bits.chunks(8).take(size) {
        let mut byte = 0u8;
        for (i, bit) in chunk.iter().enumerate() {
            byte |= bit << (7 - i);
        }
        bytes.push(byte);
    }
    Ok(bytes)
}

fn write_bytes_to_file(file_path: &str, data: &[u8]) {
    let mut file = File::create(file_path).expect("No se pudo crear el archivo");
    file.write_all(data).expect("No se pudo escribir en el archivo");
}

