use std::process::Command;
use std::env;
use image::open;
use std::fs;

const MACRO_PIXEL_SIZE: u32 = 1; // Tamaño del macropíxel (ajustable según encode.rs)
const FRAME_WIDTH: u32 = 640;    // Ancho del fotograma
const FRAME_HEIGHT: u32 = 480;   // Alto del fotograma


fn validate_frame_dimensions(folder: &str) -> Result<(), String> {
    for entry in fs::read_dir(folder).map_err(|e| format!("Error leyendo la carpeta: {}", e))? {
        let path = entry.map_err(|e| format!("Error obteniendo archivo: {}", e))?.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("png") {
            let img = open(&path).map_err(|e| format!("Error abriendo el archivo {}: {}", path.display(), e))?;
            if img.width() != FRAME_WIDTH || img.height() != FRAME_HEIGHT {
                return Err(format!(
                    "El fotograma {} tiene dimensiones incorrectas: {}x{} (esperado: {}x{})",
                    path.display(),
                    img.width(),
                    img.height(),
                    FRAME_WIDTH,
                    FRAME_HEIGHT
                ));
            }
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Uso: {} <carpeta_fotogramas> <video_salida>", args[0]);
        return;
    }

    let frames_folder = &args[1];
    let output_video = &args[2];

    // Validar dimensiones de los fotogramas
    if let Err(e) = validate_frame_dimensions(frames_folder) {
        eprintln!("Error: {}", e);
        return;
    }

    // Comando de ffmpeg para generar el vídeo
    let ffmpeg_command = format!(
        "ffmpeg -framerate 24 -i {}/frame_%04d.png -c:v libx264 -pix_fmt yuv420p {}",
        frames_folder, output_video
    );

    println!("Ejecutando: {}", ffmpeg_command);

    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg(ffmpeg_command)
        .status();

    match status {
        Ok(code) if code.success() => println!("Vídeo generado correctamente: {}", output_video),
        Ok(_) | Err(_) => eprintln!("Error al generar el vídeo. Verifica los fotogramas y ffmpeg."),
    }
}

