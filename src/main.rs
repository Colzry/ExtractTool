use std::fs::File;
use std::io::Write;
use chrono::Local;
use std::{fs, io};
use std::path::{Path, PathBuf};
use zip::read::ZipArchive;
use unrar::Archive as RarArchive;
use sevenz_rust::decompress_file;
use winapi::um::fileapi::GetLogicalDrives;

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

fn find_archives_in_current_dir() -> Vec<(String, String)> {
    let mut result = vec![];
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(file_name) = path.file_stem().and_then(|s| s.to_str()) {
                if file_name.eq_ignore_ascii_case("WindowsClient") || file_name.eq_ignore_ascii_case("Windows") {
                    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                        let ext_lc = ext.to_lowercase();
                        if ext_lc == "zip" || ext_lc == "rar" || ext_lc == "7z" {
                            result.push((file_name.to_string(), path.to_string_lossy().to_string()));
                        }
                    }
                }
            }
        }
    }
    result
}
fn get_existing_drives() -> Vec<char> {
    let mask = unsafe { GetLogicalDrives() };
    let mut drives = vec![];

    for i in 6..26 { // 从 G (第7个字母) 开始
        if (mask >> i) & 1 == 1 {
            let letter = (b'A' + i) as char;
            drives.push(letter);
        }
    }

    drives
}

fn get_target_dirs(file_stem: &str) -> Vec<PathBuf> {
    let relative_path = match file_stem {
        "WindowsClient" => r"\网络游戏\三角洲行动\DeltaForce\Saved\Config",
        "Windows" => r"\网络游戏\无畏契约\live\ShooterGame\Saved\Config",
        _ => return vec![],
    };

    get_existing_drives()
        .into_iter()
        .map(|letter| format!("{}:{}", letter, relative_path))
        .map(PathBuf::from)
        .filter(|path| path.exists())
        .collect()
}

fn write_error(message: &str) {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    if let Ok(mut file) = File::create("ERROR.txt") {
        let _ = writeln!(file, "{} {}", now, message);
    }
}

fn main() {
    let archives = find_archives_in_current_dir();

    if archives.is_empty() {
        write_error("未在当前目录找到名为 WindowsClient 或 Windows 的压缩文件（zip/rar/7z）");
        return;
    }

    for (file_stem, archive_path) in archives {
        let target_dirs = get_target_dirs(&file_stem);

        if target_dirs.is_empty() {
            let expected_path = match file_stem.as_str() {
                "WindowsClient" => r"三角洲行动",
                "Windows" => r"无畏契约",
                _ => "",
            };
            write_error(&format!("未找到 {} 的解压目录，请手动添加参数指定", expected_path));
            continue;
        }

        let ext = Path::new(&archive_path)
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_lowercase());

        for target_dir in target_dirs {
            let result = match ext.as_deref() {
                Some("zip") => extract_zip(&archive_path, target_dir.to_str().unwrap()),
                Some("rar") => extract_rar(&archive_path, target_dir.to_str().unwrap()),
                Some("7z") => extract_7z(&archive_path, target_dir.to_str().unwrap()),
                _ => {
                    write_error(&format!("{} 是不支持的文件格式", archive_path));
                    continue;
                }
            };

            if let Err(e) = result {
                write_error(&format!("解压失败（{} -> {}）：{}", archive_path, target_dir.display(), e));
            }
        }
    }
}