use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Instant;
use std::env;
use csv::Writer;
use serde::Serialize;

#[derive(Serialize)]
struct BenchmarkResult {
    run_number: u32,
    target_path: String,
    file_count: u32,
    total_time_ms: u128,
    files_per_second: f64,
}

fn print_help() {
    println!("ファイル書き込みベンチマーク");
    println!("");
    println!("使用方法:");
    println!("  {} <書き込み先パス> [ファイル数] [実行回数]", env::args().next().unwrap_or_else(|| "program".to_string()));
    println!("");
    println!("引数:");
    println!("  書き込み先パス  ファイルを作成するディレクトリパス (必須)");
    println!("  ファイル数      各実行で作成するファイル数 (デフォルト: 100000)");
    println!("  実行回数        ベンチマークの実行回数 (デフォルト: 10)");
    println!("");
    println!("オプション:");
    println!("  -h, --help  このヘルプメッセージを表示");
    println!("");
    println!("例:");
    println!("  {} /tmp/benchmark", env::args().next().unwrap_or_else(|| "program".to_string()));
    println!("  {} F:/benchmark 50000", env::args().next().unwrap_or_else(|| "program".to_string()));
    println!("  {} /mnt/f/benchmark 100000 10", env::args().next().unwrap_or_else(|| "program".to_string()));
}

fn parse_args() -> Result<(PathBuf, u32, u32), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // ヘルプの表示
    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        print_help();
        std::process::exit(0);
    }

    // 書き込み先パスは必須
    if args.len() < 2 {
        print_help();
        return Err("書き込み先パスを指定してください".into());
    }

    let target_path = PathBuf::from(&args[1]);

    let file_count = if args.len() > 2 {
        args[2].parse::<u32>()
            .map_err(|_| "ファイル数は正の整数で指定してください")?
    } else {
        100000
    };

    let runs = if args.len() > 3 {
        args[3].parse::<u32>()
            .map_err(|_| "実行回数は正の整数で指定してください")?
    } else {
        10
    };

    if file_count == 0 {
        return Err("ファイル数は1以上で指定してください".into());
    }

    if runs == 0 {
        return Err("実行回数は1以上で指定してください".into());
    }

    Ok((target_path, file_count, runs))
}

fn create_benchmark_files(base_path: &Path, file_count: u32) -> Result<(), Box<dyn std::error::Error>> {
    // 1バイトのデータ
    let data = [0u8; 1];

    for i in 0..file_count {
        let file_path = base_path.join(format!("file_{:08}.dat", i));
        let mut file = File::create(&file_path)?;
        file.write_all(&data)?;

        // 進捗表示（10000ファイルごと）
        if (i + 1) % 10000 == 0 {
            println!("  作成済み: {}/{} ファイル", i + 1, file_count);
        }
    }

    Ok(())
}

fn cleanup_benchmark_files(base_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if base_path.exists() {
        fs::remove_dir_all(base_path)?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (target_path, file_count, runs) = parse_args()?;

    println!("ファイル書き込みベンチマーク開始");
    println!("書き込み先: {}", target_path.display());
    println!("ファイル数: {}ファイル", file_count);
    println!("実行回数: {}回", runs);
    println!("");

    // 親ディレクトリが存在するか確認
    if let Some(parent) = target_path.parent() {
        if !parent.exists() {
            return Err(format!("親ディレクトリが存在しません: {}", parent.display()).into());
        }
    }

    let mut csv_writer = Writer::from_path("file_write_benchmark_results.csv")?;

    for run in 1..=runs {
        println!("実行回数: {}/{}", run, runs);

        // ベンチマーク用ディレクトリを作成
        let benchmark_dir = target_path.join(format!("run_{}", run));
        fs::create_dir_all(&benchmark_dir)?;

        // ベンチマーク実行
        let start = Instant::now();
        create_benchmark_files(&benchmark_dir, file_count)?;
        let elapsed = start.elapsed();

        let elapsed_ms = elapsed.as_millis();
        let files_per_second = (file_count as f64) / elapsed.as_secs_f64();

        println!("  完了: {}ファイル作成", file_count);
        println!("  処理時間: {}ms ({:.2}秒)", elapsed_ms, elapsed.as_secs_f64());
        println!("  処理速度: {:.2}ファイル/秒", files_per_second);

        // 結果を記録
        let result = BenchmarkResult {
            run_number: run,
            target_path: target_path.display().to_string(),
            file_count,
            total_time_ms: elapsed_ms,
            files_per_second,
        };
        csv_writer.serialize(result)?;

        // クリーンアップ
        println!("  クリーンアップ中...");
        cleanup_benchmark_files(&benchmark_dir)?;
        println!("");
    }

    csv_writer.flush()?;
    println!("ベンチマーク完了！");
    println!("結果はfile_write_benchmark_results.csvに保存されました");

    Ok(())
}
