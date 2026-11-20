//! Números e durações escritos em português ("10 minutos", "meia hora", "dez min").

use regex::Regex;
use std::sync::LazyLock;

/// Números por extenso aceitos. A ordem importa: os mais longos primeiro na regex.
const PALAVRAS: &[(&str, f64)] = &[
    ("quarenta e cinco", 45.0),
    ("vinte e cinco", 25.0),
    ("um e meio", 1.5),
    ("uma e meia", 1.5),
    ("cinquenta", 50.0),
    ("quarenta", 40.0),
    ("sessenta", 60.0),
    ("noventa", 90.0),
    ("quatro", 4.0),
    ("quinze", 15.0),
    ("trinta", 30.0),
    ("vinte", 20.0),
    ("cinco", 5.0),
    ("duas", 2.0),
    ("dois", 2.0),
    ("três", 3.0),
    ("tres", 3.0),
    ("seis", 6.0),
    ("sete", 7.0),
    ("oito", 8.0),
    ("nove", 9.0),
    ("onze", 11.0),
    ("doze", 12.0),
    ("dez", 10.0),
    ("uma", 1.0),
    ("um", 1.0),
];

pub fn valor_numero(texto: &str) -> Option<f64> {
    let t = texto.trim().to_lowercase();
    if let Ok(n) = t.replace(',', ".").parse::<f64>() {
        return Some(n);
    }
    PALAVRAS.iter().find(|(p, _)| *p == t).map(|(_, v)| *v)
}

fn alternativa_numeros() -> String {
    let palavras: Vec<&str> = PALAVRAS.iter().map(|(p, _)| *p).collect();
    format!(r"\d+(?:[.,]\d+)?|{}", palavras.join("|"))
}

/// Encontra uma duração no texto. Ex.: "10 minutos", "meia hora", "1h30", "dez min".
pub static DURACAO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        r"(?i)\b(?:(?P<meia>meia\s+hora)|(?P<h>\d+)\s*h\s*(?P<hm>\d{{1,2}})\b|(?P<n>{})\s*(?P<u>segundos?|seg|s|minutos?|min|m|horas?|h)\b(?P<emeia>\s+e\s+meia)?)",
        alternativa_numeros()
    ))
    .expect("regex de duração")
});

/// Converte o trecho encontrado por `DURACAO` em segundos.
pub fn segundos_da_duracao(c: &regex::Captures) -> Option<u64> {
    if c.name("meia").is_some() {
        return Some(30 * 60);
    }
    if let (Some(h), Some(m)) = (c.name("h"), c.name("hm")) {
        let h: u64 = h.as_str().parse().ok()?;
        let m: u64 = m.as_str().parse().ok()?;
        return Some(h * 3600 + m * 60);
    }
    let n = valor_numero(c.name("n")?.as_str())?;
    let unidade = c.name("u")?.as_str().to_lowercase();
    let por_unidade = if unidade.starts_with('h') {
        3600.0
    } else if unidade.starts_with("mi") || unidade == "m" {
        60.0
    } else {
        1.0
    };
    let extra = if c.name("emeia").is_some() { por_unidade / 2.0 } else { 0.0 };
    let total = n * por_unidade + extra;
    (total >= 1.0).then_some(total.round() as u64)
}

#[cfg(test)]
mod testes {
    use super::*;

    fn seg(texto: &str) -> Option<u64> {
        DURACAO.captures(texto).and_then(|c| segundos_da_duracao(&c))
    }

    #[test]
    fn duracoes() {
        assert_eq!(seg("10 minutos"), Some(600));
        assert_eq!(seg("10min"), Some(600));
        assert_eq!(seg("dez minutos"), Some(600));
        assert_eq!(seg("meia hora"), Some(1800));
        assert_eq!(seg("uma hora e meia"), Some(5400));
        assert_eq!(seg("1h30"), Some(5400));
        assert_eq!(seg("2 horas"), Some(7200));
        assert_eq!(seg("30 segundos"), Some(30));
        assert_eq!(seg("1,5 h"), Some(5400));
        assert_eq!(seg("pizza"), None);
    }
}
