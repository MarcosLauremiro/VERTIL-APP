//! Calculadora simples: + - * / ^, parênteses, porcentagem e palavras ("mais", "vezes").

use regex::Regex;
use std::sync::LazyLock;

static PORCENTAGEM_DE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(\d+(?:[.,]\d+)?)\s*%\s*(?:de|do|da|sobre)\s*(\d+(?:[.,]\d+)?)").unwrap()
});

/// Troca palavras por operadores e "15% de 230" por "(15/100*230)".
pub fn normalizar(expressao: &str) -> String {
    let mut s = format!(" {} ", expressao.to_lowercase());
    s = PORCENTAGEM_DE.replace_all(&s, "($1/100*$2)").into_owned();
    for (de, para) in [
        (" dividido por ", " / "),
        (" elevado a ", " ^ "),
        (" mais ", " + "),
        (" menos ", " - "),
        (" vezes ", " * "),
        (" x ", " * "),
        ("×", "*"),
        ("÷", "/"),
    ] {
        s = s.replace(de, para);
    }
    s.replace(',', ".").trim().to_string()
}

/// Avalia a expressão. Retorna `None` se não for uma conta válida ou não tiver operador.
pub fn calcular(expressao: &str) -> Option<f64> {
    let texto = normalizar(expressao);
    if !texto.chars().any(|c| "+-*/^".contains(c)) {
        return None;
    }
    let tokens = tokenizar(&texto)?;
    let mut p = Parser { tokens, pos: 0 };
    let valor = p.soma()?;
    (p.pos == p.tokens.len() && valor.is_finite()).then_some(valor)
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tok {
    Num(f64),
    Op(char),
    Abre,
    Fecha,
}

fn tokenizar(s: &str) -> Option<Vec<Tok>> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            i += 1;
        } else if c.is_ascii_digit() || c == '.' {
            let inicio = i;
            while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                i += 1;
            }
            let numero: String = chars[inicio..i].iter().collect();
            tokens.push(Tok::Num(numero.parse().ok()?));
        } else if "+-*/^".contains(c) {
            tokens.push(Tok::Op(c));
            i += 1;
        } else if c == '(' {
            tokens.push(Tok::Abre);
            i += 1;
        } else if c == ')' {
            tokens.push(Tok::Fecha);
            i += 1;
        } else {
            return None;
        }
    }
    Some(tokens)
}

struct Parser {
    tokens: Vec<Tok>,
    pos: usize,
}

impl Parser {
    fn ver(&self) -> Option<Tok> {
        self.tokens.get(self.pos).copied()
    }

    fn soma(&mut self) -> Option<f64> {
        let mut v = self.produto()?;
        while let Some(Tok::Op(op @ ('+' | '-'))) = self.ver() {
            self.pos += 1;
            let d = self.produto()?;
            v = if op == '+' { v + d } else { v - d };
        }
        Some(v)
    }

    fn produto(&mut self) -> Option<f64> {
        let mut v = self.potencia()?;
        while let Some(Tok::Op(op @ ('*' | '/'))) = self.ver() {
            self.pos += 1;
            let d = self.potencia()?;
            v = if op == '*' { v * d } else { v / d };
        }
        Some(v)
    }

    fn potencia(&mut self) -> Option<f64> {
        let base = self.unario()?;
        if let Some(Tok::Op('^')) = self.ver() {
            self.pos += 1;
            let exp = self.potencia()?;
            return Some(base.powf(exp));
        }
        Some(base)
    }

    fn unario(&mut self) -> Option<f64> {
        if let Some(Tok::Op('-')) = self.ver() {
            self.pos += 1;
            return Some(-self.unario()?);
        }
        self.atomo()
    }

    fn atomo(&mut self) -> Option<f64> {
        match self.ver()? {
            Tok::Num(n) => {
                self.pos += 1;
                Some(n)
            }
            Tok::Abre => {
                self.pos += 1;
                let v = self.soma()?;
                if self.ver()? != Tok::Fecha {
                    return None;
                }
                self.pos += 1;
                Some(v)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn contas() {
        assert_eq!(calcular("2 + 3 * 4"), Some(14.0));
        assert_eq!(calcular("(2 + 3) * 4"), Some(20.0));
        assert_eq!(calcular("15% de 230"), Some(34.5));
        assert_eq!(calcular("10 vezes 3 mais 2"), Some(32.0));
        assert_eq!(calcular("7,5 x 2"), Some(15.0));
        assert_eq!(calcular("2 ^ 10"), Some(1024.0));
        assert_eq!(calcular("100 dividido por 8"), Some(12.5));
        assert_eq!(calcular("-3 + 5"), Some(2.0));
        assert_eq!(calcular("42"), None);
        assert_eq!(calcular("1 / 0"), None);
        assert_eq!(calcular("dois mais"), None);
    }
}
