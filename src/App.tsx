import { useCallback, useEffect, useLayoutEffect, useRef, useState } from "react";
import { useAssistente } from "./hooks/useAssistente";
import { definirRegiaoClicavel, noTauri, ouvir, plataforma } from "./lib/ambiente";
import { gravarPrevia } from "./lib/banco";
import {
  formatarContagem,
  formatarData,
  formatarHora,
  formatarQuando,
  haQuanto,
  saudacao,
} from "./lib/tempo";

// Folga em volta do entalho recolhido para o mouse "pegar" mais fácil.
const FOLGA_HOVER = 10;
const ATRASO_FECHAR_MS = 280;
const MAX_AGORA = 3;
const MAX_TAREFAS = 4;

const parametros = new URLSearchParams(location.search);

export default function App() {
  const a = useAssistente();
  const agora = new Date(a.agoraMs);

  const [mouseEmCima, setMouseEmCima] = useState(false);
  const [fixado, setFixado] = useState(parametros.has("aberto")); // atalho ou digitando
  const [texto, setTexto] = useState("");
  const [enviando, setEnviando] = useState(false);
  const [salvou, setSalvou] = useState(false);

  const entalhoRef = useRef<HTMLDivElement>(null);
  const conteudoRef = useRef<HTMLDivElement>(null);
  const campoRef = useRef<HTMLInputElement>(null);
  const timerFechar = useRef<number | undefined>(undefined);

  // Alerta tocando mantém o entalho aberto até ser fechado.
  const aberto = mouseEmCima || fixado || texto.length > 0 || a.alertas.length > 0;

  // Prévia no navegador: fundo simulado e, com ?demo, exemplos na tela.
  useEffect(() => {
    if (noTauri) return;
    document.body.classList.add("previa");
    if (parametros.has("demo")) semearDemo(a.semearDemo);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const aoHover = useCallback((dentro: boolean) => {
    window.clearTimeout(timerFechar.current);
    if (dentro) setMouseEmCima(true);
    else timerFechar.current = window.setTimeout(() => setMouseEmCima(false), ATRASO_FECHAR_MS);
  }, []);

  useEffect(() => {
    const desligar = [
      ouvir<boolean>("notch://hover", aoHover),
      ouvir<null>("notch://summon", () => {
        setFixado(true);
        requestAnimationFrame(() => campoRef.current?.focus());
      }),
    ];
    return () => desligar.forEach((p) => p.then((f) => f()));
  }, [aoHover]);

  // Altura do entalho aberto acompanha o conteúdo.
  useLayoutEffect(() => {
    const conteudo = conteudoRef.current;
    const entalho = entalhoRef.current;
    if (!conteudo || !entalho) return;
    const medir = () => entalho.style.setProperty("--altura-aberta", `${conteudo.offsetHeight}px`);
    medir();
    const obs = new ResizeObserver(medir);
    obs.observe(conteudo);
    return () => obs.disconnect();
  }, []);

  // Informa ao núcleo a área clicável sempre que o entalho muda de tamanho.
  useEffect(() => {
    const entalho = entalhoRef.current;
    if (!entalho) return;
    const enviar = () => {
      const r = entalho.getBoundingClientRect();
      const folga = aberto ? 0 : FOLGA_HOVER;
      void definirRegiaoClicavel({ x: r.left - folga, y: 0, width: r.width + folga * 2, height: r.height + folga });
    };
    enviar();
    const obs = new ResizeObserver(enviar);
    obs.observe(entalho);
    return () => obs.disconnect();
  }, [aberto]);

  async function enviar(e: React.FormEvent) {
    e.preventDefault();
    const pedido = texto.trim();
    if (!pedido || enviando) return;
    setEnviando(true);
    setTexto("");
    await a.executar(pedido);
    setEnviando(false);
    setSalvou(true);
    window.setTimeout(() => setSalvou(false), 750);
  }

  function aoTeclar(e: React.KeyboardEvent<HTMLInputElement>) {
    if (e.key === "Escape") {
      setTexto("");
      setFixado(false);
      setMouseEmCima(false);
      campoRef.current?.blur();
    }
  }

  // Timer mais próximo de acabar: aparece no entalho recolhido e pinta o horizonte de âmbar.
  const timerAtivo = a.timers[0];
  const progresso = timerAtivo
    ? Math.min(1, (a.agoraMs - timerAtivo.inicioMs) / (timerAtivo.fimMs - timerAtivo.inicioMs))
    : 0;

  const itensAgora = [
    ...a.timers.map((t) => ({ tipo: "timer" as const, id: t.id, ordem: t.fimMs, t })),
    ...a.lembretes.map((l) => ({ tipo: "lembrete" as const, id: l.id, ordem: l.quandoMs, l })),
  ]
    .sort((x, y) => x.ordem - y.ordem)
    .slice(0, MAX_AGORA);

  const mac = plataforma() === "mac";
  const tab = aberto ? 0 : -1;

  return (
    <main className="palco">
      <div
        ref={entalhoRef}
        className="entalho"
        data-aberto={aberto}
        data-salvou={salvou}
        data-contando={Boolean(timerAtivo)}
        style={{ "--progresso": progresso } as React.CSSProperties}
        onMouseEnter={() => aoHover(true)}
        onMouseLeave={() => aoHover(false)}
      >
        <div className="recolhido" aria-hidden={aberto}>
          {timerAtivo ? (
            <>
              <span className="rotulo-mini">{timerAtivo.rotulo ?? "Timer"}</span>
              <span className="contagem-mini" aria-label="Tempo restante">
                {formatarContagem(timerAtivo.fimMs - a.agoraMs)}
              </span>
            </>
          ) : (
            <>
              <span className="relogio-mini">{formatarHora(agora)}</span>
              <span className="aura" />
            </>
          )}
        </div>

        <div className="aberto" ref={conteudoRef} aria-hidden={!aberto}>
          <header className="cabecalho">
            <span className="relogio">{formatarHora(agora)}</span>
            <div className="saudacao">
              <strong>{saudacao(agora)}</strong>
              <span>{formatarData(agora)}</span>
            </div>
          </header>

          {a.alertas.map((alerta) => (
            <div key={alerta.id} className="alerta" role="alert">
              <div>
                <strong>{alerta.titulo}</strong>
                <span>{alerta.detalhe}</span>
              </div>
              <button type="button" className="botao" onClick={() => a.fecharAlerta(alerta.id)} tabIndex={tab}>
                Fechar
              </button>
            </div>
          ))}

          <div className="pedido">
            <form className="comando" onSubmit={enviar}>
              <input
                ref={campoRef}
                value={texto}
                onChange={(e) => setTexto(e.target.value)}
                onKeyDown={aoTeclar}
                onFocus={() => setFixado(true)}
                onBlur={() => setFixado(false)}
                placeholder="Peça algo: timer de 10 min, me lembra às 16h..."
                aria-label="Pedido para o Vertil"
                tabIndex={tab}
              />
              <span className="teclas" aria-label={`Atalho: ${mac ? "Command" : "Ctrl"}, Shift, Espaço`}>
                <kbd>{mac ? "⌘" : "Ctrl"}</kbd>
                <kbd>⇧</kbd>
                <kbd>Espaço</kbd>
              </span>
            </form>
            <p className="resposta" data-erro={a.resposta?.erro ?? false} aria-live="polite">
              {a.resposta?.texto ?? ""}
            </p>
          </div>

          {itensAgora.length > 0 && (
            <section className="lista">
              <h2>Agora</h2>
              <ul>
                {itensAgora.map((item) =>
                  item.tipo === "timer" ? (
                    <li key={`t${item.id}`}>
                      <span className="texto">{item.t.rotulo ?? "Timer"}</span>
                      <span className="valor">
                        faltam <b className="contando">{formatarContagem(item.t.fimMs - a.agoraMs)}</b>
                      </span>
                      <button type="button" className="remover" onClick={() => a.cancelarTimer(item.id)} aria-label="Cancelar timer" tabIndex={tab}>
                        ×
                      </button>
                    </li>
                  ) : (
                    <li key={`l${item.id}`}>
                      <span className="texto">{item.l.texto}</span>
                      <span className="valor">{formatarQuando(item.l.quandoMs, agora)}</span>
                      <button type="button" className="remover" onClick={() => a.dispensarLembrete(item.id)} aria-label="Remover lembrete" tabIndex={tab}>
                        ×
                      </button>
                    </li>
                  ),
                )}
              </ul>
            </section>
          )}

          {a.tarefas.length > 0 && (
            <section className="lista">
              <h2>Tarefas</h2>
              <ul>
                {a.tarefas.slice(0, MAX_TAREFAS).map((t) => (
                  <li key={t.id}>
                    <button type="button" className="marcar" onClick={() => a.concluir(t.id)} aria-label={`Concluir: ${t.texto}`} tabIndex={tab} />
                    <span className="texto">{t.texto}</span>
                  </li>
                ))}
              </ul>
              {a.tarefas.length > MAX_TAREFAS && (
                <p className="mais">Mais {a.tarefas.length - MAX_TAREFAS} na lista.</p>
              )}
            </section>
          )}

          {a.notas.length > 0 && (
            <section className="lista">
              <h2>Notas</h2>
              <ul>
                {a.notas.map((n) => (
                  <li key={n.id}>
                    <span className="texto">{n.texto}</span>
                    <time className="valor" dateTime={n.criadoEm}>
                      {haQuanto(n.criadoEm, agora)}
                    </time>
                  </li>
                ))}
              </ul>
            </section>
          )}

          {itensAgora.length === 0 && a.tarefas.length === 0 && a.notas.length === 0 && (
            <p className="vazio">Tente: “timer de 25 minutos”, “me lembra de ligar pro cliente em 30 minutos” ou “tarefa revisar proposta”.</p>
          )}
        </div>

        <span className="horizonte" />
      </div>
    </main>
  );
}

function semearDemo(semear: ReturnType<typeof useAssistente>["semearDemo"]) {
  const agora = Date.now();
  gravarPrevia("tarefas", [
    { id: 1, texto: "Revisar proposta da clínica Sorriso", feita: false },
    { id: 2, texto: "Enviar orçamento do site", feita: false },
  ]);
  gravarPrevia("lembretes", [
    { id: 3, texto: "Ligar para o cliente", quandoMs: agora + 42 * 60000, avisado: false },
  ]);
  gravarPrevia("notas", [{ id: 4, texto: "Preciso comprar café", criadoEm: new Date(agora - 12 * 60000).toISOString() }]);
  const alertas = parametros.has("alerta")
    ? [{ id: "demo", titulo: "Pizza", detalhe: "O timer de 10 min terminou." }]
    : [];
  semear({
    timers: [{ id: 9, rotulo: "Foco", inicioMs: agora - 8 * 60000, fimMs: agora + 17 * 60000 }],
    alertas,
    resposta: { texto: "Foco: timer de 25 min começou." },
  });
  // Recarrega a lista lendo o que acabamos de gravar.
  window.setTimeout(() => window.dispatchEvent(new Event("vertil:recarregar")), 0);
}
