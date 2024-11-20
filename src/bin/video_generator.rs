use std::process::Command;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Uso: {} <carpeta_fotogramas> <video_salida>", args[0]);
        return;
    }

    let frames_folder = &args[1];
    let output_video = &args[2];

    // Comando de ffmpeg para generar el video
    let ffmpeg_command = format!(
        "ffmpeg -framerate 24 -i {}/frame_%04d.png -c:v libx264 -pix_fmt yuv420p {}",
        frames_folder, output_video
    );

    println!("Ejecutando: {}", ffmpeg_command);

    // Ejecutar el comando con sh
    let status = Command::new("sh")
        .arg("-c")
        .arg(ffmpeg_command)
        .status();

    match status {
        Ok(code) if code.success() => println!("Vídeo generado correctamente: {}", output_video),
        Ok(_) | Err(_) => eprintln!("Error al generar el vídeo. Verifica los fotogramas y ffmpeg."),
    }
}

