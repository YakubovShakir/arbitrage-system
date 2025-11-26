pub mod generated {
    // Включаем сгенерированный код
    include!("generated/_.rs");
}

// Реэкспортируем основные типы для удобства
pub use generated::*;
