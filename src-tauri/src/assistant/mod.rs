//! Assistente: entende o pedido e executa as ações que dependem do sistema.
//!
//! Divisão de responsabilidades:
//! - Rust (aqui): entender a frase (`parser`), descrever os comandos para a IA (`definicoes`)
//!   e executar ações do sistema operacional (abrir site, abrir app).
//! - Interface (`src/lib/assistente.ts`): guardar timers, lembretes, tarefas e notas e mostrar
//!   o resultado no entalho.

mod calculo;
mod numeros;
mod parser;

use chrono::Local;
use serde::Serialize;
use serde_json::{json, Value};
use tauri::AppHandle;
use tauri_plugin_opener::OpenerExt;

/// Resultado de interpretar uma frase. O campo `comando` diz qual é.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "comando", rename_all = "snake_case")]
pub enum Intencao {
    Timer { segundos: u64, rotulo: Option<String> },
    Lembrete { texto: String, quando_ms: i64 },
    CriarTarefa { texto: String },
    ListarTarefas,
    Nota { texto: String },
    Calculo { expressao: String, resultado: f64 },
    AbrirSite { url: String },
    AbrirApp { nome: String },
    /// O pedido foi entendido, mas falta um dado (ex.: lembrete sem horário).
    FaltaInfo { mensagem: String },
    /// Não é um comando conhecido. Na Fase 1b, vai para a IA.
    Desconhecido { texto: String },
}

/// Descrição de um comando para a IA (formato de "ferramenta" das APIs). Usado na Fase 1b.
#[derive(Debug, Serialize)]
pub struct Definicao {
    pub nome: &'static str,
    pub descricao: &'static str,
    pub parametros: Value,
    pub pede_confirmacao: bool,
}

pub fn definicoes() -> Vec<Definicao> {
    vec![
        Definicao {
            nome: "timer",
            descricao: "Inicia um timer regressivo, opcionalmente com um nome.",
            parametros: json!({"type":"object","properties":{"segundos":{"type":"integer","minimum":1},"rotulo":{"type":"string"}},"required":["segundos"]}),
            pede_confirmacao: false,
        },
        Definicao {
            nome: "lembrete",
            descricao: "Cria um lembrete para um momento específico.",
            parametros: json!({"type":"object","properties":{"texto":{"type":"string"},"quando_iso":{"type":"string","description":"Data e hora local em ISO 8601"}},"required":["texto","quando_iso"]}),
            pede_confirmacao: false,
        },
        Definicao {
            nome: "criar_tarefa",
            descricao: "Adiciona uma tarefa à lista do usuário.",
            parametros: json!({"type":"object","properties":{"texto":{"type":"string"}},"required":["texto"]}),
            pede_confirmacao: false,
        },
        Definicao {
            nome: "nota",
            descricao: "Salva uma anotação curta.",
            parametros: json!({"type":"object","properties":{"texto":{"type":"string"}},"required":["texto"]}),
            pede_confirmacao: false,
        },
        Definicao {
            nome: "abrir_site",
            descricao: "Abre um endereço no navegador padrão.",
            parametros: json!({"type":"object","properties":{"url":{"type":"string","format":"uri"}},"required":["url"]}),
            pede_confirmacao: false,
        },
        Definicao {
            nome: "abrir_app",
            descricao: "Abre um aplicativo instalado pelo nome.",
            parametros: json!({"type":"object","properties":{"nome":{"type":"string"}},"required":["nome"]}),
            pede_confirmacao: false,
        },
    ]
}

#[tauri::command]
pub fn interpretar(texto: String) -> Intencao {
    parser::interpretar(&texto, Local::now())
}

#[tauri::command]
pub fn listar_comandos() -> Vec<Definicao> {
    definicoes()
}

#[tauri::command]
pub fn abrir_site(app: AppHandle, url: String) -> Result<(), String> {
    let url = url.trim();
    if !(url.starts_with("https://") || url.starts_with("http://")) {
        return Err("Só abro endereços que começam com http:// ou https://.".into());
    }
    app.opener()
        .open_url(url, None::<&str>)
        .map_err(|e| format!("Não consegui abrir o site: {e}"))
}

#[tauri::command]
pub fn abrir_app(nome: String) -> Result<(), String> {
    let nome = nome.trim();
    // Só letras, números, espaço, ponto, hífen e sublinhado: o nome vai para o sistema.
    let valido = !nome.is_empty()
        && nome.len() <= 60
        && nome.chars().all(|c| c.is_alphanumeric() || " .-_".contains(c));
    if !valido {
        return Err("Esse nome de app não é válido.".into());
    }
    crate::platform::abrir_app(nome)
}
