use std::fs::File;
use std::io;
use std::path::Path;
use zip::read::ZipArchive;
use unrar::Archive as RarArchive;
use sevenz_rust::decompress_file;

fn extract_zip(archive_path: &str, output_dir: &str) -> io::Result<()> {
    let file = File::open(archive_path)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = Path::new(output_dir).join(file.name());

        if file.name().ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}

fn extract_rar(archive_path: &str, output_dir: &str) -> io::Result<()> {
    RarArchive::new(archive_path.to_string())
        .extract_to(output_dir.to_string())
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
        .process()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    Ok(())
}


fn extract_7z(archive_path: &str, output_dir: &str) -> io::Result<()> {
    decompress_file(archive_path, output_dir).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("用法: {} <压缩文件路径> <输出目录>", args[0]);
        std::process::exit(1);
    }
    let archive_path = &args[1];
    let output_dir = &args[2];

    let ext = Path::new(archive_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase());

    match ext.as_deref() {
        Some("zip") => {
            if let Err(e) = extract_zip(archive_path, output_dir) {
                eprintln!("解压ZIP文件时出错: {}", e);
            }
        }
        Some("rar") => {
            if let Err(e) = extract_rar(archive_path, output_dir) {
                eprintln!("解压RAR文件时出错: {}", e);
            }
        }
        Some("7z") => {
            if let Err(e) = extract_7z(archive_path, output_dir) {
                eprintln!("解压7Z文件时出错: {}", e);
            }
        }
        _ => eprintln!("不支持的文件格式"),
    }
}
