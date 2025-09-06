use std::fs;
use std::process::Command;
use csv::Writer;
use serde::Serialize;
use rand::{thread_rng, Rng};
use tokio::task::JoinSet;
use std::sync::Arc;
use std::env;

#[derive(Serialize)]
struct ImageQualityStats {
    run_number: u32,
    quality: u32,
    total_size: u64,
    average_size: f64,
    min_size: u64,
    max_size: u64,
    median_size: f64,
    compression_ratio: f64,
}

fn print_help() {
    println!("画像品質比較ベンチマーク（テスト版）");
    println!("");
    println!("使用方法:");
    println!("  {} [画像枚数] [ラウンド数]", env::args().next().unwrap_or_else(|| "test_program".to_string()));
    println!("");
    println!("引数:");
    println!("  画像枚数    各ラウンドで生成する画像の枚数 (デフォルト: 5)");
    println!("  ラウンド数  ベンチマークの実行回数 (デフォルト: 1)");
    println!("");
    println!("オプション:");
    println!("  -h, --help  このヘルプメッセージを表示");
    println!("");
    println!("例:");
    println!("  {}           # デフォルト: 5枚、1ラウンド", env::args().next().unwrap_or_else(|| "test_program".to_string()));
    println!("  {} 10        # 10枚、1ラウンド", env::args().next().unwrap_or_else(|| "test_program".to_string()));
    println!("  {} 20 2      # 20枚、2ラウンド", env::args().next().unwrap_or_else(|| "test_program".to_string()));
    println!("");
    println!("品質設定: PNG→WebP変換で100%、80%、60%の3段階で品質比較（テスト版）");
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
        5  // テスト版のデフォルト
    };
    
    let rounds = if args.len() > 2 {
        args[2].parse::<u32>()
            .map_err(|_| "ラウンド数は正の整数で指定してください")?
    } else {
        1  // テスト版のデフォルト
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
    
    println!("画像品質比較ベンチマーク（テスト版）開始: {}枚の画像で{}ラウンド実行", image_count, rounds);
    println!("品質設定: 100%, 80%, 60%の3段階で比較");
    
    let mut csv_writer = Writer::from_path("image_comparison_quality_test_results.csv")?;
    
    // テスト用品質設定（3段階）
    let quality_levels = [100, 80, 60];
    
    for run in 1..=rounds {
        println!("実行回数: {}/{}（テスト）", run, rounds);
        
        let output_dir = format!("test_images_run_{}", run);
        fs::create_dir_all(&output_dir)?;
        
        // ランダムなPNG画像を生成（4並列）
        generate_random_png_images_parallel(&output_dir, image_count).await?;
        println!("PNG画像{}枚を生成しました", image_count);
        
        // 各品質レベルでWebP変換（4並列）
        for &quality in &quality_levels {
            convert_png_to_webp_parallel(&output_dir, quality, image_count).await?;
            println!("品質{}%でWebP変換しました", quality);
            
            // 各品質の統計を計算
            let png_stats = calculate_png_stats(&output_dir, run, image_count)?;
            let webp_stats = calculate_webp_quality_stats(&output_dir, quality, run, image_count, png_stats.total_size)?;
            
            // CSV出力（WebPの結果のみ）
            csv_writer.serialize(&webp_stats)?;
        }
        
        // 画像ファイルを削除
        cleanup_images(&output_dir, image_count, &quality_levels)?;
        fs::remove_dir(&output_dir)?;
        println!("実行 {} 完了（画像ファイル削除済み）", run);
    }
    
    csv_writer.flush()?;
    
    // 実験終了後の最終クリーンアップ
    cleanup_remaining_files(rounds)?;
    println!("全ての実行が完了しました。結果はimage_comparison_quality_test_results.csvに保存されました。");
    
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

async fn convert_png_to_webp_parallel(output_dir: &str, quality: u32, count: u32) -> Result<(), Box<dyn std::error::Error>> {
    let mut join_set = JoinSet::new();
    let output_dir = Arc::new(output_dir.to_string());
    
    let chunk_size = count / 4;
    for thread_id in 0..4 {
        let start = thread_id * chunk_size;
        let end = if thread_id == 3 { count } else { (thread_id + 1) * chunk_size };
        let output_dir_clone = Arc::clone(&output_dir);
        
        join_set.spawn(async move {
            for i in start..end {
                let input_path = format!("{}/image_{:03}.png", output_dir_clone.as_str(), i);
                let output_path = format!("{}/image_{:03}_q{}.webp", output_dir_clone.as_str(), i, quality);
                
                let status = Command::new("convert")
                    .args(&[
                        &input_path,
                        "-quality", &quality.to_string(),
                        &output_path
                    ])
                    .status()
                    .map_err(|e| format!("変換コマンド実行エラー: {}", e))?;
                    
                if !status.success() {
                    return Err(format!("画像変換に失敗しました: {} -> {}", input_path, output_path));
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

fn calculate_png_stats(output_dir: &str, run_number: u32, image_count: u32) -> Result<ImageQualityStats, Box<dyn std::error::Error>> {
    let mut sizes: Vec<u64> = Vec::new();
    
    for i in 0..image_count {
        let file_path = format!("{}/image_{:03}.png", output_dir, i);
        let metadata = fs::metadata(&file_path)?;
        sizes.push(metadata.len());
    }
    
    sizes.sort();
    
    let total_size: u64 = sizes.iter().sum();
    let average_size = total_size as f64 / sizes.len() as f64;
    let min_size = *sizes.first().unwrap();
    let max_size = *sizes.last().unwrap();
    
    let median_size = if sizes.len() % 2 == 0 {
        let mid = sizes.len() / 2;
        (sizes[mid - 1] + sizes[mid]) as f64 / 2.0
    } else {
        sizes[sizes.len() / 2] as f64
    };
    
    Ok(ImageQualityStats {
        run_number,
        quality: 100,
        total_size,
        average_size,
        min_size,
        max_size,
        median_size,
        compression_ratio: 1.0,
    })
}

fn calculate_webp_quality_stats(output_dir: &str, quality: u32, run_number: u32, image_count: u32, png_total_size: u64) -> Result<ImageQualityStats, Box<dyn std::error::Error>> {
    let mut sizes: Vec<u64> = Vec::new();
    
    for i in 0..image_count {
        let file_path = format!("{}/image_{:03}_q{}.webp", output_dir, i, quality);
        let metadata = fs::metadata(&file_path)?;
        sizes.push(metadata.len());
    }
    
    sizes.sort();
    
    let total_size: u64 = sizes.iter().sum();
    let average_size = total_size as f64 / sizes.len() as f64;
    let min_size = *sizes.first().unwrap();
    let max_size = *sizes.last().unwrap();
    
    let median_size = if sizes.len() % 2 == 0 {
        let mid = sizes.len() / 2;
        (sizes[mid - 1] + sizes[mid]) as f64 / 2.0
    } else {
        sizes[sizes.len() / 2] as f64
    };
    
    let compression_ratio = total_size as f64 / png_total_size as f64;
    
    Ok(ImageQualityStats {
        run_number,
        quality,
        total_size,
        average_size,
        min_size,
        max_size,
        median_size,
        compression_ratio,
    })
}

fn cleanup_images(output_dir: &str, image_count: u32, quality_levels: &[u32]) -> Result<(), Box<dyn std::error::Error>> {
    // PNG files
    for i in 0..image_count {
        let file_path = format!("{}/image_{:03}.png", output_dir, i);
        if fs::metadata(&file_path).is_ok() {
            fs::remove_file(&file_path)?;
        }
    }
    
    // WebP files
    for &quality in quality_levels {
        for i in 0..image_count {
            let file_path = format!("{}/image_{:03}_q{}.webp", output_dir, i, quality);
            if fs::metadata(&file_path).is_ok() {
                fs::remove_file(&file_path)?;
            }
        }
    }
    
    Ok(())
}

fn cleanup_remaining_files(rounds: u32) -> Result<(), Box<dyn std::error::Error>> {
    for run in 1..=rounds {
        let dir_name = format!("test_images_run_{}", run);
        if fs::metadata(&dir_name).is_ok() {
            let entries = fs::read_dir(&dir_name)?;
            for entry in entries {
                let entry = entry?;
                fs::remove_file(entry.path())?;
            }
            fs::remove_dir(&dir_name)?;
        }
    }
    
    if fs::metadata("test.png").is_ok() {
        fs::remove_file("test.png")?;
    }
    
    Ok(())
}