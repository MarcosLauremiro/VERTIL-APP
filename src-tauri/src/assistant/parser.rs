//! Entendimento local: transforma frases simples em comandos, sem IA e sem internet.
//! Frases que não se encaixam viram `Desconhecido` (na Fase 1b, vão para a IA).

use chrono::{DateTime, Duration, Local, NaiveTime, TimeZone};
use regex::{Captures, Regex};
use std::sync::LazyLock;

use super::calculo;
use super::numeros::{segundos_da_duracao, DURACAO};
use super::Intencao;

fn re(padrao: &str) -> Regex {
    Regex::new(padrao).expect("regex do interpretador")
}

static ESPACOS: LazyLock<Regex> = LazyLock::new(|| re(r"\s+"));
static CHAMAMENTO: LazyLock<Regex> =
    LazyLock::new(|| re(r"(?i)^(?:ei|oi|olá|ola)?,?\s*vertil[,:]?\s*"));
static CONECTORES: LazyLock<Regex> = LazyLock::new(|| {
    re(r"(?i)^(?:\s*\b(?:de|do|da|que|para|pra|pro|o|a|me|se|um|uma|e)\b\s*)+|(?:\s*\b(?:de|do|da|para|pra|pro|e|às|as)\b\s*)+$")
});

static LEMBRETE: LazyLock<Regex> = LazyLock::new(|| {
    re(r"(?i)^(?:(?:me|nos)\s+)?(?:lembr[ae](?:-me)?|lembrar|lembrete|avis[ae](?:-me)?)\b(?P<resto>.*)$")
});
static QUANDO_RELATIVO: LazyLock<Regex> =
    LazyLock::new(|| re(r"(?i)\b(?:em|daqui\s+a|daqui|dentro\s+de)\s+(?P<dur>.+?)(?:\s|$)"));
static QUANDO_ABSOLUTO: LazyLock<Regex> = LazyLock::new(|| {
    re(r"(?i)(?:\b(?P<dia>amanhã|amanha|hoje)\s*)?(?:\b(?:às|as|a|ao)\s+)?(?P<h>\d{1,2})(?:\s*(?:h|:)\s*(?P<m>\d{2})?|\s+horas?)\b|\b(?P<dia2>amanhã|amanha)\b")
});

static TIMER: LazyLock<Regex> = LazyLock::new(|| {
    re(r"(?i)^(?:(?:coloca|coloque|põe|poe|cria|crie|inicia|inicie|começa|comeca|liga|ligue|faz|faça|faca)\s+(?:um\s+)?)?(?:timer|temporizador|cronômetro|cronometro)\b(?P<resto>.*)$")
});

static CRIAR_TAREFA: LazyLock<Regex> = LazyLock::new(|| {
    re(r"(?i)^(?:(?:adiciona|adicione|adicionar|cria|crie|criar|nova|novo|coloca|coloque)\s+(?:uma\s+)?)?tarefa\s*(?:de|para|pra|:|-)?\s+(?P<texto>.+)$")
});
static LISTAR_TAREFAS: LazyLock<Regex> = LazyLock::new(|| {
    re(r"(?i)^(?:(?:quais|ver|veja|mostra|mostre|mostrar|listar|lista|liste)\s+)?(?:(?:são|sao|as|minhas)\s+)*(?:minhas\s+)?tarefas(?:\s+de\s+hoje)?\??$")
});

static NOTA: LazyLock<Regex> = LazyLock::new(|| {
    re(r"(?i)^(?:anota|anote|anotar|nota)(?:\s+(?:que|aí|ai|isso))?\s*[:\-]?\s+(?P<texto>.+)$")
});

static CALCULO: LazyLock<Regex> = LazyLock::new(|| {
    re(r"(?i)^(?:quanto\s+(?:é|e|da|dá|fica)|calcula|calcule|calcular|conta)\s+(?P<expr>.+?)\??$")
});
static SO_CONTA: LazyLock<Regex> = LazyLock::new(|| re(r"^[\d\s.,+\-*/x×÷^()%]+$"));

static ABRIR: LazyLock<Regex> = LazyLock::new(|| {
    re(r"(?i)^(?:abre|abrir|abra|abri|acessa|acesse|entra\s+n[oa])\s+(?:o\s+|a\s+)?(?P<site>site\s+(?:do\s+|da\s+|de\s+)?)?(?P<alvo>.+)$")
});

/// Sites conhecidos que o usuário costuma chamar pelo nome.
const SITES: &[(&str, &str)] = &[
    ("youtube", "https://www.youtube.com"),
    ("gmail", "https://mail.google.com"),
    ("google", "https://www.google.com"),
    ("google agenda", "https://calendar.google.com"),
    ("agenda", "https://calendar.google.com"),
    ("drive", "https://drive.google.com"),
    ("whatsapp web", "https://web.whatsapp.com"),
    ("instagram", "https://www.instagram.com"),
    ("netflix", "https://www.netflix.com"),
    ("chatgpt", "https://chatgpt.com"),
    ("claude", "https://claude.ai"),
    ("github", "https://github.com"),
    ("linkedin", "https://www.linkedin.com"),
    ("notion", "https://www.notion.so"),
];

/// Interpreta uma frase. `agora` é passado de fora para os testes serem previsíveis.
pub fn interpretar(texto_bruto: &str, agora: DateTime<Local>) -> Intencao {
    let texto = limpar(texto_bruto);
    if texto.is_empty() {
        return Intencao::Desconhecido { texto };
    }

    if let Some(c) = LEMBRETE.captures(&texto) {
        return lembrete(&c["resto"], agora);
    }
    if let Some(c) = TIMER.captures(&texto) {
        return timer(&c["resto"]);
    }
    if LISTAR_TAREFAS.is_match(&texto) {
        return Intencao::ListarTarefas;
    }
    if let Some(c) = CRIAR_TAREFA.captures(&texto) {
        return Intencao::CriarTarefa { texto: capitalizar(&c["texto"]) };
    }
    if let Some(c) = NOTA.captures(&texto) {
        return Intencao::Nota { texto: capitalizar(&c["texto"]) };
    }
    if let Some(c) = ABRIR.captures(&texto) {
        return abrir(&c);
    }
    let expressao = CALCULO
        .captures(&texto)
        .map(|c| c["expr"].to_string())
        .or_else(|| SO_CONTA.is_match(&texto).then(|| texto.clone()));
    if let Some(expressao) = expressao {
        if let Some(resultado) = calculo::calcular(&expressao) {
            return Intencao::Calculo { expressao: expressao.trim().to_string(), resultado };
        }
    }

    Intencao::Desconhecido { texto }
}

fn limpar(texto: &str) -> String {
    let t = ESPACOS.replace_all(texto.trim(), " ");
    let t = t.trim_end_matches(['.', '!']);
    // "Ei, Vertil," no começo é só chamamento.
    let t = CHAMAMENTO.replace(t, "");
    t.trim().to_string()
}

fn capitalizar(texto: &str) -> String {
    let t = texto.trim().trim_end_matches(['.', ',', '?']);
    let mut letras = t.chars();
    match letras.next() {
        Some(primeira) => primeira.to_uppercase().collect::<String>() + letras.as_str(),
        None => String::new(),
    }
}

/// Tira conectores soltos no começo e no fim ("de", "para", "que"...).
fn aparar_conectores(texto: &str) -> String {
    let mut atual = texto.trim().to_string();
    loop {
        let novo = CONECTORES.replace_all(&atual, "").trim().to_string();
        if novo == atual {
            return novo;
        }
        atual = novo;
    }
}

fn timer(resto: &str) -> Intencao {
    let Some(c) = DURACAO.captures(resto) else {
        return Intencao::FaltaInfo {
            mensagem: "De quanto tempo? Por exemplo: timer de 10 minutos.".into(),
        };
    };
    let Some(segundos) = segundos_da_duracao(&c) else {
        return Intencao::FaltaInfo { mensagem: "Não entendi o tempo do timer.".into() };
    };
    let m = c.get(0).expect("captura inteira");
    let sobra = format!("{} {}", &resto[..m.start()], &resto[m.end()..]);
    let rotulo = aparar_conectores(&sobra);
    Intencao::Timer {
        segundos,
        rotulo: (!rotulo.is_empty()).then(|| capitalizar(&rotulo)),
    }
}

fn lembrete(resto: &str, agora: DateTime<Local>) -> Intencao {
    // 1) Tempo relativo: "em 30 minutos", "daqui a uma hora".
    if let Some(rel) = QUANDO_RELATIVO.captures_iter(resto).find_map(|c| {
        let inicio = c.get(0)?.start();
        let trecho_dur = &resto[c.name("dur")?.start()..];
        let d = DURACAO.captures(trecho_dur)?;
        let dm = d.get(0)?;
        // A duração precisa vir logo depois de "em"/"daqui a".
        if dm.start() > 1 {
            return None;
        }
        let fim = c.name("dur")?.start() + dm.end();
        Some((inicio, fim, segundos_da_duracao(&d)?))
    }) {
        let (inicio, fim, segundos) = rel;
        let texto = aparar_conectores(&format!("{} {}", &resto[..inicio], &resto[fim..]));
        let quando = agora + Duration::seconds(segundos as i64);
        return montar_lembrete(&texto, quando);
    }

    // 2) Horário: "às 16h30", "amanhã às 9h", "amanhã".
    if let Some(c) = QUANDO_ABSOLUTO.captures(resto) {
        if let Some(quando) = horario(&c, agora) {
            let m = c.get(0).expect("captura inteira");
            let texto =
                aparar_conectores(&format!("{} {}", &resto[..m.start()], &resto[m.end()..]));
            return montar_lembrete(&texto, quando);
        }
    }

    Intencao::FaltaInfo {
        mensagem: "Para quando? Diga, por exemplo: em 30 minutos ou às 16h.".into(),
    }
}

fn horario(c: &Captures, agora: DateTime<Local>) -> Option<DateTime<Local>> {
    let amanha = c
        .name("dia")
        .or(c.name("dia2"))
        .is_some_and(|d| d.as_str().to_lowercase().starts_with("amanh"));
    let (hora, minuto) = match c.name("h") {
        Some(h) => (h.as_str().parse::<u32>().ok()?, c.name("m").map_or(Ok(0), |m| m.as_str().parse()).ok()?),
        None => (9, 0), // "amanhã" sem hora: 9h
    };
    let hora_dia = NaiveTime::from_hms_opt(hora, minuto, 0)?;
    let mut data = agora.date_naive();
    if amanha {
        data = data.succ_opt()?;
    }
    let mut quando = Local.from_local_datetime(&data.and_time(hora_dia)).earliest()?;
    if !amanha && quando <= agora {
        quando += Duration::days(1);
    }
    Some(quando)
}

fn montar_lembrete(texto: &str, quando: DateTime<Local>) -> Intencao {
    if texto.is_empty() {
        return Intencao::FaltaInfo { mensagem: "Do que devo lembrar?".into() };
    }
    Intencao::Lembrete { texto: capitalizar(texto), quando_ms: quando.timestamp_millis() }
}

fn abrir(c: &Captures) -> Intencao {
    let alvo = c["alvo"].trim().trim_end_matches(['.', '?']).to_string();
    let alvo_min = alvo.to_lowercase();
    if let Some((_, url)) = SITES.iter().find(|(nome, _)| *nome == alvo_min) {
        return Intencao::AbrirSite { url: (*url).to_string() };
    }
    let parece_endereco = alvo_min.contains('.') && !alvo_min.contains(' ');
    if c.name("site").is_some() || parece_endereco {
        let endereco = if parece_endereco { alvo_min } else { format!("{}.com", alvo_min.replace(' ', "")) };
        let url = if endereco.starts_with("http://") || endereco.starts_with("https://") {
            endereco
        } else {
            format!("https://{endereco}")
        };
        return Intencao::AbrirSite { url };
    }
    Intencao::AbrirApp { nome: capitalizar(&alvo) }
}

#[cfg(test)]
mod testes {
    use super::*;
    use chrono::{Datelike, Timelike};

    fn agora() -> DateTime<Local> {
        Local.with_ymd_and_hms(2026, 11, 20, 15, 0, 0).unwrap()
    }

    fn i(texto: &str) -> Intencao {
        interpretar(texto, agora())
    }

    #[test]
    fn timers() {
        assert_eq!(i("timer de 10 minutos para a pizza"), Intencao::Timer { segundos: 600, rotulo: Some("Pizza".into()) });
        assert_eq!(i("Ei, Vertil, timer de 5 min"), Intencao::Timer { segundos: 300, rotulo: None });
        assert_eq!(i("coloca um timer de meia hora pro bolo"), Intencao::Timer { segundos: 1800, rotulo: Some("Bolo".into()) });
        assert_eq!(i("timer 25 minutos foco"), Intencao::Timer { segundos: 1500, rotulo: Some("Foco".into()) });
        assert!(matches!(i("timer"), Intencao::FaltaInfo { .. }));
    }

    #[test]
    fn lembretes_relativos() {
        let esperado = agora() + Duration::minutes(30);
        assert_eq!(
            i("me lembra de ligar pro cliente em 30 minutos"),
            Intencao::Lembrete { texto: "Ligar pro cliente".into(), quando_ms: esperado.timestamp_millis() }
        );
        assert_eq!(
            i("me lembra em 1 hora de tirar o bolo"),
            Intencao::Lembrete { texto: "Tirar o bolo".into(), quando_ms: (agora() + Duration::hours(1)).timestamp_millis() }
        );
    }

    #[test]
    fn lembretes_com_horario() {
        let Intencao::Lembrete { texto, quando_ms } = i("lembrete às 16h30 reunião com a equipe") else { panic!() };
        assert_eq!(texto, "Reunião com a equipe");
        let q = Local.timestamp_millis_opt(quando_ms).unwrap();
        assert_eq!((q.hour(), q.minute(), q.day()), (16, 30, 20));

        let Intencao::Lembrete { texto, quando_ms } = i("me lembra amanhã às 9h de pagar o boleto") else { panic!() };
        assert_eq!(texto, "Pagar o boleto");
        let q = Local.timestamp_millis_opt(quando_ms).unwrap();
        assert_eq!((q.hour(), q.minute(), q.day()), (9, 0, 21));

        // 10h já passou hoje (são 15h): vai para amanhã.
        let Intencao::Lembrete { quando_ms, .. } = i("me lembra de ligar pra Ana às 10h") else { panic!() };
        assert_eq!(Local.timestamp_millis_opt(quando_ms).unwrap().day(), 21);

        assert!(matches!(i("me lembra de comprar pão"), Intencao::FaltaInfo { .. }));

        let Intencao::Lembrete { texto, .. } = i("me lembra às 16h de ligar pro João") else { panic!() };
        assert_eq!(texto, "Ligar pro João");
        let Intencao::Lembrete { texto, .. } = i("lembra de beber água em 20 min") else { panic!() };
        assert_eq!(texto, "Beber água");
    }

    #[test]
    fn tarefas_e_notas() {
        assert_eq!(i("adiciona tarefa revisar proposta da clínica"), Intencao::CriarTarefa { texto: "Revisar proposta da clínica".into() });
        assert_eq!(i("tarefa: enviar orçamento"), Intencao::CriarTarefa { texto: "Enviar orçamento".into() });
        assert_eq!(i("minhas tarefas"), Intencao::ListarTarefas);
        assert_eq!(i("quais são as minhas tarefas?"), Intencao::ListarTarefas);
        assert_eq!(i("anota que preciso comprar café"), Intencao::Nota { texto: "Preciso comprar café".into() });
    }

    #[test]
    fn calculos() {
        assert_eq!(i("quanto é 15% de 230"), Intencao::Calculo { expressao: "15% de 230".into(), resultado: 34.5 });
        assert_eq!(i("12*7+3"), Intencao::Calculo { expressao: "12*7+3".into(), resultado: 87.0 });
        assert!(matches!(i("42"), Intencao::Desconhecido { .. }));
    }

    #[test]
    fn abrir() {
        assert_eq!(i("abre o youtube"), Intencao::AbrirSite { url: "https://www.youtube.com".into() });
        assert_eq!(i("abre g1.com.br"), Intencao::AbrirSite { url: "https://g1.com.br".into() });
        assert_eq!(i("abre o site da vertil"), Intencao::AbrirSite { url: "https://vertil.com".into() });
        assert_eq!(i("abre o spotify"), Intencao::AbrirApp { nome: "Spotify".into() });
    }

    #[test]
    fn desconhecido() {
        assert!(matches!(i("qual a capital da França"), Intencao::Desconhecido { .. }));
    }
}
