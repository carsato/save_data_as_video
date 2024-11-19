use std::fs::{File, create_dir_all};
use std::io::{Read, Write};
use image::{ImageBuffer, RgbImage, open};

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
            codificar(archivo, "video_output.mp4");
        }
        "decodificar" => {
            if args.len() < 3 {
                eprintln!("Falta el video a decodificar.");
                return;
            }
            let video = &args[2];
            decodificar(video, "archivo_recuperado.txt");
        }
        _ => {
            eprintln!("Modo no reconocido. Usa 'codificar' o 'decodificar'.");
        }
    }
}

fn codificar(input_file: &str, output_video: &str) {
    let data = read_file_as_bytes(input_file);
    let frames_folder = "frames";
    create_dir_all(frames_folder).unwrap();
    encode_data_to_frames(&data, frames_folder);
    create_video_from_frames(frames_folder, output_video);
    println!("Archivo codificado en el video: {}", output_video);
}

fn decodificar(input_video: &str, output_file: &str) {
    let frames_folder = "extracted_frames";
    create_dir_all(frames_folder).unwrap();
    extract_frames_from_video(input_video, frames_folder);
    let data = decode_data_from_frames(frames_folder);
    write_bytes_to_file(output_file, &data);
    println!("Datos recuperados en el archivo: {}", output_file);
}

// Funciones comunes

fn read_file_as_bytes(file_path: &str) -> Vec<u8> {
    let mut file = File::open(file_path).expect("No se pudo abrir el archivo");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("No se pudo leer el archivo");
    buffer
}

fn encode_data_to_frames(data: &[u8], output_folder: &str) {
    let mut data_with_size = Vec::new();
    let size = data.len() as u32;
    data_with_size.extend(size.to_be_bytes()); // Tamaño del archivo
    data_with_size.extend(data);

    let mut byte_index = 0;
    for frame_number in 0.. {
        if byte_index >= data_with_size.len() {
            break;
        }

        let mut img: RgbImage = ImageBuffer::new(640, 480);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            if byte_index >= data_with_size.len() {
                break;
            }

            let byte = data_with_size[byte_index];
            *pixel = image::Rgb([byte, byte, byte]); // Codificar el byte en el píxel
            byte_index += 1;
        }

        let frame_path = format!("{}/frame_{:04}.png", output_folder, frame_number);
        img.save(frame_path).unwrap();
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
    let mut data = Vec::new();
    for entry in std::fs::read_dir(frames_folder).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("png") {
            let img = open(path).unwrap().into_rgb8();
            for pixel in img.pixels() {
                data.push(pixel[0]); // Leer el canal rojo del píxel
            }
        }
    }

    let size = u32::from_be_bytes([data[0], data[1], data[2], data[3]]) as usize;
    data[4..4 + size].to_vec() // Extraer solo los datos originales
}

fn write_bytes_to_file(file_path: &str, data: &[u8]) {
    let mut file = File::create(file_path).expect("No se pudo crear el archivo");
    file.write_all(data).expect("No se pudo escribir en el archivo");
}
