use clap::Parser;
use std::fs::File;
use std::io::Write;
use chrono::Local;
use std::{fs, io};
use std::path::{Path, PathBuf};
use zip::read::ZipArchive;
use unrar::Archive as RarArchive;
use sevenz_rust::decompress_file;
use winapi::um::fileapi::GetLogicalDrives;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// 要解压的压缩包名（可多个）：WindowsClient 或 Windows
    #[arg(short = 'p', long)]
    package: Vec<String>,

    /// 目标解压路径（可选，仅对一个压缩包生效，需和 -p 一起使用指定压缩包名）
    #[arg(short = 'd', long)]
    directory: Option<String>,
}

// 全局压缩包名
const VALID_NAMES: [&str; 2] = ["WindowsClient", "Windows"];

// 支持的压缩包扩展名
const VALID_EXTS: [&str; 3] = ["zip", "rar", "7z"];

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

            let file_stem = path.file_stem().and_then(|s| s.to_str());
            let extension = path.extension().and_then(|e| e.to_str());

            if let (Some(name), Some(ext)) = (file_stem, extension) {
                // 将文件名和扩展名转换为小写
                let name_lc = name.to_lowercase();
                let ext_lc = ext.to_lowercase();

                if VALID_NAMES.iter().any(|n| n.eq_ignore_ascii_case(&name_lc)) &&
                    VALID_EXTS.contains(&ext_lc.as_str())
                {
                    result.push((name.to_string(), path.to_string_lossy().to_string()));
                }
            }
        }
    }

    result
}
fn get_existing_drives() -> Vec<char> {
    let mask = unsafe { GetLogicalDrives() };
    let mut drives = vec![];

    for i in 4..26 { // 从 D (第4个字母) 开始
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
    let args = Args::parse();

    if args.directory.is_some() && args.package.len() != 1 {
        write_error("-d 参数必须与 -p 参数一起使用，且只能指定一个压缩包");
        return;
    }

    let mut archives = find_archives_in_current_dir();
    if archives.is_empty() {
        let name_list = VALID_NAMES.join("、");
        let ext_list = VALID_EXTS.join("/");
        write_error(&format!("当前目录下没有任何名为 {} 的压缩包（{}）", name_list, ext_list));
        return;
    }

    if !args.package.is_empty() {
        // 只保留用户指定的压缩包
        archives.retain(|(name, _)| {
            args.package.iter().any(|p| p.eq_ignore_ascii_case(name))
        });

        if archives.is_empty() {
            let name_list = if args.package.len() > 1 {
                args.package.join("、")
            } else {
                args.package[0].clone()
            };
            let ext_list = VALID_EXTS.join("/");
            write_error(&format!("当前目录下没有任何名为 {} 的压缩包（{}），请检查指定的参数", name_list, ext_list));
            return;
        }
    }

    for (file_stem, archive_path) in archives {
        let target_dirs = if let Some(ref custom_path) = args.directory {
            vec![PathBuf::from(custom_path)]
        } else {
            get_target_dirs(&file_stem)
        };

        if target_dirs.is_empty() {
            let expected_path = match file_stem.as_str() {
                "WindowsClient" => r"三角洲行动",
                "Windows" => r"无畏契约",
                _ => "未知",
            };
            write_error(&format!("未找到 {} 的目标目录用于解压，请使用 -p 指定当前目录下的压缩包名称，-d 指定解压目录", expected_path));
            continue;
        }

        let ext = Path::new(&archive_path)
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase());

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