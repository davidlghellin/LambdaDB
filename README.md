# LambdaDB

Mini query engine para aprender Apache Arrow y los conceptos detrás de DataFusion.

## Objetivo

Construir un motor de consultas SQL simplificado desde cero para entender:

- Cómo funciona el formato columnar de Arrow
- El pipeline: SQL → Plan Lógico → Plan Físico → Ejecución
- Optimización de queries
- Procesamiento vectorizado

## Arquitectura

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Parser    │ ──▶ │Plan Lógico  │ ──▶ │Plan Físico  │ ──▶ │  Ejecución  │
│    SQL      │     │   (AST)     │     │ (Operadores)│     │   (Arrow)   │
└─────────────┘     └─────────────┘     └─────────────┘     └─────────────┘
```

## Fases del proyecto

### Fase 1: Fundamentos Arrow
- [ ] Crear RecordBatches manualmente
- [ ] Leer/escribir archivos CSV a Arrow
- [ ] Operaciones básicas: filtrar, proyectar

### Fase 2: Parser SQL
- [ ] Tokenizer básico
- [ ] Parser para SELECT, FROM, WHERE
- [ ] AST (Abstract Syntax Tree)

### Fase 3: Plan Lógico
- [ ] Nodos: Scan, Filter, Projection
- [ ] Construcción del árbol desde el AST
- [ ] Representación del schema

### Fase 4: Plan Físico
- [ ] Operadores físicos
- [ ] Modelo de ejecución pull-based (iteradores)
- [ ] Procesamiento por batches

### Fase 5: Optimizaciones
- [ ] Predicate pushdown
- [ ] Projection pushdown
- [ ] Eliminación de columnas no usadas

### Fase 6: Extras (opcional)
- [ ] JOIN básico (hash join)
- [ ] Agregaciones (GROUP BY)
- [ ] Lectura de Parquet

## Estructura del proyecto

```
src/
├── main.rs           # CLI y punto de entrada
├── lib.rs            # Exporta módulos
├── arrow_utils/      # Helpers para trabajar con Arrow
├── parser/           # Tokenizer y parser SQL
├── logical_plan/     # Plan lógico y nodos
├── physical_plan/    # Operadores físicos
├── execution/        # Motor de ejecución
└── optimizer/        # Reglas de optimización
```

## Comandos

```bash
# Compilar
cargo build

# Ejecutar tests
cargo test

# Ejecutar
cargo run

# Ejemplo futuro
echo "SELECT name, age FROM users WHERE age > 25" | cargo run
```

## Dependencias principales

- `arrow` - Formato columnar en memoria
- `parquet` - Lectura/escritura de Parquet (más adelante)

## Referencias

- [Arrow Columnar Format](https://arrow.apache.org/docs/format/Columnar.html)
- [DataFusion repo](https://github.com/apache/arrow-datafusion)
- [How Query Engines Work](https://howqueryengineswork.com/)
