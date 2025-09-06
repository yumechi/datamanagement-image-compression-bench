use std::fs;
use std::process::Command;
use csv::Writer;
use serde::Serialize;
use rand::{thread_rng, Rng};
use tokio::task::JoinSet;
use std::sync::Arc;
use std::env;
use std::time::Instant;

#[derive(Serialize)]
struct CompressionStats {
    run_number: u32,
    format: String,
    original_size: u64,
    compressed_size: u64,
    compression_ratio: f64,
    compression_time_ms: u64,
    compression_speed_mbps: f64,
}

fn print_help() {
    println!("データ圧縮フォーマット比較ベンチマーク");
    println!("");
    println!("使用方法:");
    println!("  {} [画像枚数] [圧縮回数]", env::args().next().unwrap_or_else(|| "program".to_string()));
    println!("");
    println!("引数:");
    println!("  画像枚数    圧縮対象の画像枚数 (デフォルト: 100)");
    println!("  圧縮回数    各フォーマットでの圧縮実行回数 (デフォルト: 100)");
    println!("");
    println!("オプション:");
    println!("  -h, --help  このヘルプメッセージを表示");
    println!("");
    println!("例:");
    println!("  {}           # デフォルト: 100枚、100回圧縮", env::args().next().unwrap_or_else(|| "program".to_string()));
    println!("  {} 50        # 50枚、100回圧縮", env::args().next().unwrap_or_else(|| "program".to_string()));
    println!("  {} 200 50    # 200枚、50回圧縮", env::args().next().unwrap_or_else(|| "program".to_string()));
    println!("");
    println!("対応フォーマット: ZIP, TAR.GZ, ZSTD, XZ, 7Z");
}

fn parse_args() -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        print_help();
        std::process::exit(0);
    }
    
    let image_count = if args.len() > 1 {
        args[1].parse::<u32>()
            .map_err(|_| "画像枚数は正の整数で指定してください")?
    } else {
        100
    };
    
    let compression_runs = if args.len() > 2 {
        args[2].parse::<u32>()
            .map_err(|_| "圧縮回数は正の整数で指定してください")?
    } else {
        100
    };
    
    if image_count == 0 {
        return Err("画像枚数は1以上で指定してください".into());
    }
    
    if compression_runs == 0 {
        return Err("圧縮回数は1以上で指定してください".into());
    }
    
    Ok((image_count, compression_runs))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (image_count, compression_runs) = parse_args()?;
    
    println!("データ圧縮フォーマット比較ベンチマーク開始");
    println!("画像枚数: {}枚、各フォーマット{}回圧縮実行", image_count, compression_runs);
    
    // 必要なコマンドの確認
    check_required_commands()?;
    
    let mut csv_writer = Writer::from_path("compression_format_comparison_results.csv")?;
    
    // 圧縮フォーマット
    let formats = ["zip", "tar.gz", "zstd", "xz", "7z"];
    
    // ベンチマーク用ディレクトリ作成
    let benchmark_dir = "benchmark_images";
    fs::create_dir_all(benchmark_dir)?;
    
    // 画像ファイル生成（4並列）
    println!("ベンチマーク用画像{}枚を生成中...", image_count);
    generate_random_png_images_parallel(benchmark_dir, image_count).await?;
    let original_size = calculate_directory_size(benchmark_dir)?;
    println!("画像生成完了: 総サイズ {:.2} MB", original_size as f64 / 1024.0 / 1024.0);
    
    // 各フォーマットで圧縮テスト
    for format in &formats {
        println!("\\n{}フォーマット圧縮テスト開始...", format.to_uppercase());
        
        for run in 1..=compression_runs {
            if run % 10 == 0 || run == 1 {
                println!("  {}: {}/{} 実行中", format.to_uppercase(), run, compression_runs);
            }
            
            let start_time = Instant::now();
            let compressed_size = compress_directory(benchmark_dir, format, run).await?;
            let compression_time = start_time.elapsed();
            
            let compression_ratio = compressed_size as f64 / original_size as f64;
            let compression_speed = (original_size as f64 / 1024.0 / 1024.0) / compression_time.as_secs_f64();
            
            let stats = CompressionStats {
                run_number: run,
                format: format.to_uppercase().to_string(),
                original_size,
                compressed_size,
                compression_ratio,
                compression_time_ms: compression_time.as_millis() as u64,
                compression_speed_mbps: compression_speed,
            };
            
            csv_writer.serialize(&stats)?;
            
            // 圧縮ファイルを削除（ディスク容量節約）
            cleanup_compressed_file(format, run)?;
        }
        
        println!("  {}フォーマット完了", format.to_uppercase());
    }
    
    csv_writer.flush()?;
    
    // クリーンアップ
    fs::remove_dir_all(benchmark_dir)?;
    cleanup_remaining_files(&formats)?;
    
    println!("\\n全ての圧縮テストが完了しました。");
    println!("結果はcompression_format_comparison_results.csvに保存されました。");
    
    Ok(())
}

async fn generate_random_png_images_parallel(output_dir: &str, count: u32) -> Result<(), Box<dyn std::error::Error>> {
    let mut join_set = JoinSet::new();
    let output_dir = Arc::new(output_dir.to_string());
    
    let chunk_size = count / 4;
    for thread_id in 0..4 {
        let start = thread_id * chunk_size;
        let end = if thread_id == 3 { count } else { (thread_id + 1) * chunk_size };
        let output_dir_clone = Arc::clone(&output_dir);
        
        join_set.spawn(async move {
            let mut rng = thread_rng();
            for i in start..end {
                let r: u8 = rng.r#gen();
                let g: u8 = rng.r#gen();
                let b: u8 = rng.r#gen();
                
                let output_path = format!("{}/image_{:03}.png", output_dir_clone.as_str(), i);
                
                let status = Command::new("convert")
                    .args(&[
                        "-size", "1024x1024",
                        &format!("xc:rgb({},{},{})", r, g, b),
                        "+noise", "Random",
                        &output_path
                    ])
                    .status()
                    .map_err(|e| format!("画像生成コマンド実行エラー: {}", e))?;
                    
                if !status.success() {
                    return Err(format!("画像生成に失敗しました: {}", output_path));
                }
            }
            Ok::<(), String>(())
        });
    }
    
    while let Some(result) = join_set.join_next().await {
        match result {
            Ok(Ok(())) => {},
            Ok(Err(e)) => return Err(e.into()),
            Err(e) => return Err(format!("並列実行エラー: {}", e).into()),
        }
    }
    
    Ok(())
}

async fn compress_directory(dir_path: &str, format: &str, run_number: u32) -> Result<u64, Box<dyn std::error::Error>> {
    let output_file = format!("{}_run_{}", dir_path, run_number);
    
    let status = match format {
        "zip" => {
            Command::new("zip")
                .args(&["-r", "-q", &format!("{}.zip", output_file), dir_path])
                .status()?
        },
        "tar.gz" => {
            Command::new("tar")
                .args(&["-czf", &format!("{}.tar.gz", output_file), "-C", ".", dir_path])
                .status()?
        },
        "zstd" => {
            // まずtarで一時ファイル作成
            let temp_tar = format!("{}.tar", output_file);
            let tar_status = Command::new("tar")
                .args(&["-cf", &temp_tar, "-C", ".", dir_path])
                .status()?;
            
            if !tar_status.success() {
                return Err("tarファイル作成に失敗".into());
            }
            
            // zstdで圧縮
            let status = Command::new("zstd")
                .args(&[&temp_tar, "-o", &format!("{}.tar.zst", output_file)])
                .status()?;
                
            // 一時tarファイル削除
            let _ = fs::remove_file(&temp_tar);
            status
        },
        "xz" => {
            Command::new("tar")
                .args(&["-cJf", &format!("{}.tar.xz", output_file), "-C", ".", dir_path])
                .status()?
        },
        "7z" => {
            Command::new("7z")
                .args(&["a", "-t7z", &format!("{}.7z", output_file), dir_path])
                .stdout(std::process::Stdio::null())
                .status()?
        },
        _ => return Err(format!("未対応のフォーマット: {}", format).into()),
    };
    
    if !status.success() {
        return Err(format!("圧縮に失敗しました: {}", format).into());
    }
    
    // 圧縮ファイルサイズを取得
    let compressed_file = match format {
        "zip" => format!("{}.zip", output_file),
        "tar.gz" => format!("{}.tar.gz", output_file),
        "zstd" => format!("{}.tar.zst", output_file),
        "xz" => format!("{}.tar.xz", output_file),
        "7z" => format!("{}.7z", output_file),
        _ => return Err("Unknown format".into()),
    };
    
    let metadata = fs::metadata(&compressed_file)?;
    Ok(metadata.len())
}

fn calculate_directory_size(dir_path: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let mut total_size = 0;
    let entries = fs::read_dir(dir_path)?;
    
    for entry in entries {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.is_file() {
            total_size += metadata.len();
        }
    }
    
    Ok(total_size)
}

fn cleanup_compressed_file(format: &str, run_number: u32) -> Result<(), Box<dyn std::error::Error>> {
    let base_name = format!("benchmark_images_run_{}", run_number);
    
    let file_path = match format {
        "zip" => format!("{}.zip", base_name),
        "tar.gz" => format!("{}.tar.gz", base_name),
        "zstd" => format!("{}.tar.zst", base_name),
        "xz" => format!("{}.tar.xz", base_name),
        "7z" => format!("{}.7z", base_name),
        _ => return Ok(()),
    };
    
    if fs::metadata(&file_path).is_ok() {
        fs::remove_file(&file_path)?;
    }
    
    Ok(())
}

fn cleanup_remaining_files(formats: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    for format in formats {
        for run in 1..=200 { // 十分な範囲で削除
            let _ = cleanup_compressed_file(format, run);
        }
    }
    
    Ok(())
}

fn check_required_commands() -> Result<(), Box<dyn std::error::Error>> {
    let commands = [
        ("convert", "ImageMagick"),
        ("zip", "zip"),
        ("tar", "tar"),
        ("zstd", "zstandard"),
        ("xz", "xz-utils"),
        ("7z", "p7zip-full"),
    ];
    
    let mut missing_commands = Vec::new();
    
    for (cmd, package) in &commands {
        match Command::new(cmd).arg("--help").output() {
            Ok(_) => {},
            Err(_) => {
                missing_commands.push((*cmd, *package));
            }
        }
    }
    
    if !missing_commands.is_empty() {
        println!("エラー: 以下のコマンドが見つかりません:");
        for (cmd, package) in &missing_commands {
            println!("  {} (パッケージ: {})", cmd, package);
        }
        println!("\\nUbuntu/Debianでのインストール:");
        println!("  sudo apt-get update");
        println!("  sudo apt-get install imagemagick zip tar zstd xz-utils p7zip-full");
        println!("\\nmacOSでのインストール:");
        println!("  brew install imagemagick zstd xz p7zip");
        
        return Err("必要なコマンドが不足しています".into());
    }
    
    println!("全ての必要なコマンドが利用可能です。");
    Ok(())
}