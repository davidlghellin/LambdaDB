//! Utilidades para trabajar con Apache Arrow
//!
//! Este módulo contiene helpers para crear y manipular RecordBatches.

use arrow::array::{ArrayRef, Int64Array, StringArray, RecordBatch};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::error::ArrowError;
use std::sync::Arc;

pub type Result<T> = std::result::Result<T, ArrowError>;

/// Crea un RecordBatch simple con datos de ejemplo (una tabla de usuarios)
///
/// Esto demuestra:
/// - Crear un Schema (definir columnas y tipos)
/// - Crear Arrays tipados (Int64Array, StringArray)
/// - Combinarlos en un RecordBatch
pub fn create_sample_batch() -> Result<RecordBatch> {
    // Schema (estructura)        →  Arrays (datos)           →  RecordBatch (tabla)
    // ┌─────────────────────┐      ┌──────────────────┐       ┌───────────────────┐
    // │ id:   Int64, !null  │      │ [1, 2, 3]        │       │ Schema + Columnas │
    // │ name: Utf8,  !null  │  +   │ ["Alice","Bob"…] │   =   │ = Tabla completa  │
    // │ age:  Int64, null   │      │ [30, 25, NULL]   │       │                   │
    // └─────────────────────┘      └──────────────────┘       └───────────────────┘

    // 1. Definir el schema (estructura de la tabla)
    let schema = Schema::new(vec![
        Field::new("id", DataType::Int64, false),        // columna "id", tipo Int64, no nullable
        Field::new("name", DataType::Utf8, false),       // columna "name", tipo String, no nullable
        Field::new("age", DataType::Int64, true),        // columna "age", tipo Int64, nullable
    ]);

    // 2. Crear los arrays (columnas con datos)
    let ids: ArrayRef = Arc::new(Int64Array::from(vec![1, 2, 3]));
    let names: ArrayRef = Arc::new(StringArray::from(vec!["Alice", "Bob", "Charlie"]));
    let ages: ArrayRef = Arc::new(Int64Array::from(vec![Some(30), Some(25), None])); // None = NULL

    // 3. Crear el RecordBatch (combina schema + datos)
    RecordBatch::try_new(Arc::new(schema), vec![ids, names, ages])
}

/// Imprime un RecordBatch de forma legible
pub fn print_batch(batch: &RecordBatch) {
    println!("Schema: {:?}", batch.schema());
    println!("Num rows: {}", batch.num_rows());
    println!("Num columns: {}", batch.num_columns());

    // Imprimir cada columna
    for (i, field) in batch.schema().fields().iter().enumerate() {
        println!("  Column '{}': {:?}", field.name(), batch.column(i));
    }
}
