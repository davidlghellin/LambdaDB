use lambda_db::arrow_utils;

fn main() {
    println!("LambdaDB - Mini Query Engine");
    println!("=============================\n");

    // Crear un RecordBatch de ejemplo
    println!("📊 Creando RecordBatch de ejemplo...\n");

    match arrow_utils::create_sample_batch() {
        Ok(batch) => {
            arrow_utils::print_batch(&batch);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
