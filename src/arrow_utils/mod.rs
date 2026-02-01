//! Utilidades para trabajar con Apache Arrow
//!
//! Este módulo contiene helpers para crear y manipular RecordBatches.

use arrow::array::RecordBatch;
use arrow::error::ArrowError;

pub type Result<T> = std::result::Result<T, ArrowError>;

// TODO: Fase 1 - Implementar funciones básicas de Arrow
