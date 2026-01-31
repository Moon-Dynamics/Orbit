use serde::{Serialize, Deserialize};

// 1. Opciones de Bases de Datos Industriales
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Database {
    None,
    PostgreSQL, // El estándar de oro open source
    TimescaleDB, // Vital para industria: guarda datos de sensores (series de tiempo)
    MongoDB,    // Para logs no estructurados
    Redis,      // Para caché ultra rápida
}

// 2. Opciones de UI (Lo que ya tenías)
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum UiFramework {
    None,
    Avalonia,
    WPF,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ProjectType {
    RustCLI,
    Hybrid(UiFramework),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrbitConfig {
    pub project_name: String,
    pub project_type: ProjectType,
    pub database: Database, // <--- ¡NUEVO CAMPO!
    pub version: String,
}

impl OrbitConfig {
    // Actualizamos el constructor para pedir la DB
    pub fn new(name: &str, p_type: ProjectType, db: Database) -> Self {
        Self {
            project_name: name.to_string(),
            project_type: p_type,
            database: db,
            version: "0.1.0".to_string(),
        }
    }
}

