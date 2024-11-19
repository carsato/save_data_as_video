use std::fs::{self, File, create_dir_all};
use std::io::{Read, Write};
use image::{ImageBuffer, RgbImage, Rgb};

const MACRO_PIXEL_SIZE: u32 = 10; // Tamaño de los bloques de macropíxeles (10x10)

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Uso: {} <codificar|decodificar> [archivo|video]", args[0]);
        return;
    }

    let modo = &args[1];
    match modo.as_str() {
        "codificar" => {
            if args.len() < 3 {
                eprintln!("Falta el archivo a codificar.");
                return;
            }
            let archivo = &args[2];
            codificar(archivo, "video_output.mp4", "fotogramas_codificados");
        }
        "decodificar" => {
            if args.len() < 3 {
                eprintln!("Falta el video a decodificar.");
                return;
            }
            let video = &args[2];
            decodificar(video, "archivo_recuperado.txt", "fotogramas_decodificados");
        }
        _ => {
            eprintln!("Modo no reconocido. Usa 'codificar' o 'decodificar'.");
        }
    }
}

fn codificar(input_file: &str, output_video: &str, frames_folder: &str) {
    let data = read_file_as_binary(input_file);

    // Crear carpeta para los fotogramas
    create_dir_all(frames_folder).expect("No se pudo crear la carpeta de fotogramas.");

    encode_data_to_frames(&data, frames_folder);

    create_video_from_frames(frames_folder, output_video);
    println!("Archivo codificado en el video: {}", output_video);
}

fn decodificar(input_video: &str, output_file: &str, frames_folder: &str) {
    // Crear carpeta para los fotogramas extraídos
    create_dir_all(frames_folder).expect("No se pudo crear la carpeta de fotogramas.");

    extract_frames_from_video(input_video, frames_folder);

    let data = decode_data_from_frames(frames_folder);

    write_bytes_to_file(output_file, &data);
    println!("Datos recuperados en el archivo: {}", output_file);
}

fn read_file_as_binary(file_path: &str) -> Vec<u8> {
    let mut file = File::open(file_path).expect("No se pudo abrir el archivo");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("No se pudo leer el archivo");

    // Convertir cada byte en bits individuales (0 o 1)
    let mut bits = Vec::new();
    for byte in buffer {
        for bit in (0..8).rev() {
            bits.push((byte >> bit) & 1); // Extraer cada bit
        }
    }
    bits
}

fn encode_data_to_frames(data: &[u8], output_folder: &str) {
    let frame_width = 640;
    let frame_height = 480;
    let macropixels_per_row = frame_width / MACRO_PIXEL_SIZE;
    let macropixels_per_col = frame_height / MACRO_PIXEL_SIZE;

    let mut bit_index = 0;

    for frame_number in 0.. {
        if bit_index >= data.len() {
            break;
        }

        let mut img: RgbImage = ImageBuffer::new(frame_width, frame_height);

        for row in 0..macropixels_per_col {
            for col in 0..macropixels_per_row {
                if bit_index >= data.len() {
                    break;
                }

                // Determinar color del macropíxel
                let color = if data[bit_index] == 1 {
                    Rgb([255, 255, 255]) // Blanco
                } else {
                    Rgb([0, 0, 0]) // Negro
                };

                // Dibujar el macropíxel
                for y in 0..MACRO_PIXEL_SIZE {
                    for x in 0..MACRO_PIXEL_SIZE {
                        let px = col * MACRO_PIXEL_SIZE + x;
                        let py = row * MACRO_PIXEL_SIZE + y;
                        img.put_pixel(px, py, color);
                    }
                }

                bit_index += 1;
            }
        }

        let frame_path = format!("{}/frame_{:04}.png", output_folder, frame_number);
        img.save(frame_path).expect("No se pudo guardar el fotograma.");
    }
}

fn create_video_from_frames(frames_folder: &str, output_video: &str) {
    let ffmpeg_command = format!(
        "ffmpeg -framerate 24 -i {}/frame_%04d.png -c:v libx264 -pix_fmt yuv420p {}",
        frames_folder, output_video
    );
    std::process::Command::new("sh")
        .arg("-c")
        .arg(ffmpeg_command)
        .status()
        .expect("Error al ejecutar ffmpeg");
}

fn extract_frames_from_video(input_video: &str, frames_folder: &str) {
    let ffmpeg_command = format!(
        "ffmpeg -i {} {}/frame_%04d.png",
        input_video, frames_folder
    );
    std::process::Command::new("sh")
        .arg("-c")
        .arg(ffmpeg_command)
        .status()
        .expect("Error al ejecutar ffmpeg");
}

fn decode_data_from_frames(frames_folder: &str) -> Vec<u8> {
    let mut bits = Vec::new();

    for entry in fs::read_dir(frames_folder).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("png") {
            let img = open(path).unwrap().into_rgb8();
            for (_, _, pixel) in img.enumerate_pixels() {
                if pixel[0] == 255 {
                    bits.push(1); // Blanco
                } else {
                    bits.push(0); // Negro
                }
            }
        }
    }

    // Convertir bits de vuelta a bytes
    let mut bytes = Vec::new();
    for chunk in bits.chunks(8) {
        let mut byte = 0u8;
        for (i, bit) in chunk.iter().enumerate() {
            byte |= bit << (7 - i);
        }
        bytes.push(byte);
    }
    bytes
}

fn write_bytes_to_file(file_path: &str, data: &[u8]) {
    let mut file = File::create(file_path).expect("No se pudo crear el archivo");
    file.write_all(data).expect("No se pudo escribir en el archivo");
}

