// build.rs
use std::io::Result;

fn main() -> Result<()> {
    // Создаем выходную директорию
    std::fs::create_dir_all("src/proto/generated").ok();
    println!("cargo:rerun-if-changed=proto/");

    // Компилируем все .proto файлы
    prost_build::Config::new()
        .out_dir("src/proto/generated") // Генерируем код в src/proto
        .compile_protos(
            &[
                "proto/PublicMiniTickersV3Api.proto",
                "proto/PushDataV3ApiWrapper.proto",
            ],
            &["proto/"], // путь где искать .proto файлы
        )?;

    Ok(())
}
