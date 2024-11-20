use std::fs::{File, create_dir_all};
use std::io::Read;
use image::{ImageBuffer, RgbImage, Rgb};

const MACRO_PIXEL_SIZE: u32 = 10; // Tamaño de los bloques de píxeles (10x10)
const FRAME_WIDTH: u32 = 640; // Ancho del fotograma
const FRAME_HEIGHT: u32 = 480; // Alto del fotograma

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Uso: {} <archivo_entrada> <carpeta_salida>", args[0]);
        return;
    }

    let input_file = &args[1];
    let output_folder = &args[2];

    // Leer el archivo y convertirlo a bits
    let data = read_file_as_binary(input_file);

    // Crear fotogramas con los datos binarios
    encode_data_to_frames(&data, output_folder);
    println!("Fotogramas generados en la carpeta: {}", output_folder);
}

fn read_file_as_binary(file_path: &str) -> Vec<u8> {
    let mut file = File::open(file_path).expect("No se pudo abrir el archivo");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("No se pudo leer el archivo");

    let mut bits = Vec::new();

    // Agregar el tamaño del archivo como los primeros 4 bytes (32 bits)
    let size = buffer.len() as u32;
    bits.extend(size.to_be_bytes().iter().flat_map(|&byte| {
        (0..8).rev().map(move |bit| (byte >> bit) & 1)
    }));

    // Convertir cada byte a bits individuales
    for byte in buffer {
        for bit in (0..8).rev() {
            bits.push((byte >> bit) & 1);
        }
    }

    bits
}

fn encode_data_to_frames(data: &[u8], output_folder: &str) {
    create_dir_all(output_folder).expect("No se pudo crear la carpeta de salida");

    let macropixels_per_row = FRAME_WIDTH / MACRO_PIXEL_SIZE;
    let macropixels_per_col = FRAME_HEIGHT / MACRO_PIXEL_SIZE;
    let mut bit_index = 0;

    for frame_number in 0.. {
        if bit_index >= data.len() {
            break;
        }

        let mut img: RgbImage = ImageBuffer::new(FRAME_WIDTH, FRAME_HEIGHT);

        for row in 0..macropixels_per_col {
            for col in 0..macropixels_per_row {
                if bit_index >= data.len() {
                    break;
                }

                // Determinar el color del macropíxel (blanco o negro)
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

        // Guardar el fotograma como imagen
        let frame_path = format!("{}/frame_{:04}.png", output_folder, frame_number);
        img.save(frame_path).expect("No se pudo guardar el fotograma");
    }
}

