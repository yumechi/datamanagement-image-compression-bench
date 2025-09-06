use std::fs;
use std::process::Command;
use csv::Writer;
use serde::Serialize;
use rand::{thread_rng, Rng};
use tokio::task::JoinSet;
use std::sync::Arc;
use std::env;

#[derive(Serialize)]
struct ImageStats {
    run_number: u32,
    format: String,
    total_size: u64,
    average_size: f64,
    min_size: u64,
    max_size: u64,
    median_size: f64,
}

fn print_help() {
    println!("画像フォーマット比較ベンチマーク");
    println!("");
    println!("使用方法:");
    println!("  {} [画像枚数] [ラウンド数]", env::args().next().unwrap_or_else(|| "program".to_string()));
    println!("");
    println!("引数:");
    println!("  画像枚数    各ラウンドで生成する画像の枚数 (デフォルト: 100)");
    println!("  ラウンド数  ベンチマークの実行回数 (デフォルト: 10)");
    println!("");
    println!("オプション:");
    println!("  -h, --help  このヘルプメッセージを表示");
    println!("");
    println!("例:");
    println!("  {}           # デフォルト: 100枚、10ラウンド", env::args().next().unwrap_or_else(|| "program".to_string()));
    println!("  {} 50        # 50枚、10ラウンド", env::args().next().unwrap_or_else(|| "program".to_string()));
    println!("  {} 200 5     # 200枚、5ラウンド", env::args().next().unwrap_or_else(|| "program".to_string()));
}

fn parse_args() -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    // ヘルプの表示
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
    
    let rounds = if args.len() > 2 {
        args[2].parse::<u32>()
            .map_err(|_| "ラウンド数は正の整数で指定してください")?
    } else {
        10
    };
    
    if image_count == 0 {
        return Err("画像枚数は1以上で指定してください".into());
    }
    
    if rounds == 0 {
        return Err("ラウンド数は1以上で指定してください".into());
    }
    
    Ok((image_count, rounds))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (image_count, rounds) = parse_args()?;
    
    println!("画像フォーマット比較ベンチマーク開始: {}枚の画像で{}ラウンド実行", image_count, rounds);
    
    let mut csv_writer = Writer::from_path("image_format_comparison_results.csv")?;
    
    for run in 1..=rounds {
        println!("実行回数: {}/{}", run, rounds);
        
        let output_dir = format!("images_run_{}", run);
        fs::create_dir_all(&output_dir)?;
        
        // ランダムなPNG画像を生成（4並列）
        generate_random_png_images_parallel(&output_dir, image_count).await?;
        println!("PNG画像{}枚を生成しました", image_count);
        
        // PNG -> JPG/WebP変換（4並列）
        convert_images_parallel(&output_dir, "png", "jpg", image_count).await?;
        println!("JPG画像に変換しました");
        
        convert_images_parallel(&output_dir, "png", "webp", image_count).await?;
        println!("WebP画像に変換しました");
        
        // 各形式の統計を計算
        let png_stats = calculate_stats(&output_dir, "png", run, image_count)?;
        let jpg_stats = calculate_stats(&output_dir, "jpg", run, image_count)?;
        let webp_stats = calculate_stats(&output_dir, "webp", run, image_count)?;
        
        // CSV出力
        csv_writer.serialize(&png_stats)?;
        csv_writer.serialize(&jpg_stats)?;
        csv_writer.serialize(&webp_stats)?;
        
        // 画像ファイルを削除
        cleanup_images(&output_dir, image_count)?;
        fs::remove_dir(&output_dir)?;
        println!("実行 {} 完了（画像ファイル削除済み）", run);
    }
    
    csv_writer.flush()?;
    
    // 実験終了後の最終クリーンアップ
    cleanup_remaining_files(rounds)?;
    println!("全ての実行が完了しました。結果はimage_format_comparison_results.csvに保存されました。");
    
    Ok(())
}

async fn generate_random_png_images_parallel(output_dir: &str, count: u32) -> Result<(), Box<dyn std::error::Error>> {
    let mut join_set = JoinSet::new();
    let output_dir = Arc::new(output_dir.to_string());
    
    // 4並列でタスクを分割
    let chunk_size = count / 4;
    for thread_id in 0..4 {
        let start = thread_id * chunk_size;
        let end = if thread_id == 3 { count } else { (thread_id + 1) * chunk_size };
        let output_dir_clone = Arc::clone(&output_dir);
        
        join_set.spawn(async move {
            let mut rng = thread_rng();
            for i in start..end {
                // ランダムな色でノイズ画像を生成
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
    
    // 全てのタスクの完了を待つ
    while let Some(result) = join_set.join_next().await {
        match result {
            Ok(Ok(())) => {},
            Ok(Err(e)) => return Err(e.into()),
            Err(e) => return Err(format!("並列実行エラー: {}", e).into()),
        }
    }
    
    Ok(())
}

async fn convert_images_parallel(output_dir: &str, from_format: &str, to_format: &str, count: u32) -> Result<(), Box<dyn std::error::Error>> {
    let mut join_set = JoinSet::new();
    let output_dir = Arc::new(output_dir.to_string());
    let from_format = Arc::new(from_format.to_string());
    let to_format = Arc::new(to_format.to_string());
    
    // 4並列でタスクを分割
    let chunk_size = count / 4;
    for thread_id in 0..4 {
        let start = thread_id * chunk_size;
        let end = if thread_id == 3 { count } else { (thread_id + 1) * chunk_size };
        let output_dir_clone = Arc::clone(&output_dir);
        let from_format_clone = Arc::clone(&from_format);
        let to_format_clone = Arc::clone(&to_format);
        
        join_set.spawn(async move {
            for i in start..end {
                let input_path = format!("{}/image_{:03}.{}", output_dir_clone.as_str(), i, from_format_clone.as_str());
                let output_path = format!("{}/image_{:03}.{}", output_dir_clone.as_str(), i, to_format_clone.as_str());
                
                let mut args = vec![input_path.as_str()];
                
                // 劣化なしの設定
                match to_format_clone.as_str() {
                    "jpg" => {
                        args.extend_from_slice(&["-quality", "100"]);
                    },
                    "webp" => {
                        args.extend_from_slice(&["-quality", "100"]);
                    },
                    _ => {}
                }
                
                args.push(output_path.as_str());
                
                let status = Command::new("convert")
                    .args(&args)
                    .status()
                    .map_err(|e| format!("変換コマンド実行エラー: {}", e))?;
                    
                if !status.success() {
                    return Err(format!("画像変換に失敗しました: {} -> {}", input_path, output_path));
                }
            }
            Ok::<(), String>(())
        });
    }
    
    // 全てのタスクの完了を待つ
    while let Some(result) = join_set.join_next().await {
        match result {
            Ok(Ok(())) => {},
            Ok(Err(e)) => return Err(e.into()),
            Err(e) => return Err(format!("並列実行エラー: {}", e).into()),
        }
    }
    
    Ok(())
}

fn calculate_stats(output_dir: &str, format: &str, run_number: u32, image_count: u32) -> Result<ImageStats, Box<dyn std::error::Error>> {
    let mut sizes: Vec<u64> = Vec::new();
    
    for i in 0..image_count {
        let file_path = format!("{}/image_{:03}.{}", output_dir, i, format);
        let metadata = fs::metadata(&file_path)?;
        sizes.push(metadata.len());
    }
    
    sizes.sort();
    
    let total_size: u64 = sizes.iter().sum();
    let average_size = total_size as f64 / sizes.len() as f64;
    let min_size = *sizes.first().unwrap();
    let max_size = *sizes.last().unwrap();
    
    // 中央値計算
    let median_size = if sizes.len() % 2 == 0 {
        let mid = sizes.len() / 2;
        (sizes[mid - 1] + sizes[mid]) as f64 / 2.0
    } else {
        sizes[sizes.len() / 2] as f64
    };
    
    Ok(ImageStats {
        run_number,
        format: format.to_uppercase(),
        total_size,
        average_size,
        min_size,
        max_size,
        median_size,
    })
}

fn cleanup_images(output_dir: &str, image_count: u32) -> Result<(), Box<dyn std::error::Error>> {
    let formats = ["png", "jpg", "webp"];
    
    for format in &formats {
        for i in 0..image_count {
            let file_path = format!("{}/image_{:03}.{}", output_dir, i, format);
            if fs::metadata(&file_path).is_ok() {
                fs::remove_file(&file_path)?;
            }
        }
    }
    
    Ok(())
}

fn cleanup_remaining_files(rounds: u32) -> Result<(), Box<dyn std::error::Error>> {
    // 残存する可能性のある画像ディレクトリをクリーンアップ
    for run in 1..=rounds {
        let dir_name = format!("images_run_{}", run);
        if fs::metadata(&dir_name).is_ok() {
            // ディレクトリ内のファイルを全て削除
            let entries = fs::read_dir(&dir_name)?;
            for entry in entries {
                let entry = entry?;
                fs::remove_file(entry.path())?;
            }
            // 空のディレクトリを削除
            fs::remove_dir(&dir_name)?;
        }
    }
    
    // その他の一時ファイル削除
    if fs::metadata("test.png").is_ok() {
        fs::remove_file("test.png")?;
    }
    
    Ok(())
}
