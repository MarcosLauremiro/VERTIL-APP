import { useCallback, useEffect, useLayoutEffect, useRef, useState } from "react";
import { definirRegiaoClicavel, noTauri, ouvir, plataforma } from "./lib/ambiente";
import { listarRecentes, salvarNota, type Nota } from "./lib/notas";
import { formatarData, formatarHora, haQuanto, saudacao } from "./lib/tempo";

// Folga em volta do entalho recolhido para o mouse "pegar" mais fácil.
const FOLGA_HOVER = 10;
const ATRASO_FECHAR_MS = 280;

function useAgora(intervaloMs = 15000): Date {
  const [agora, setAgora] = useState(() => new Date());
  useEffect(() => {
    const id = window.setInterval(() => setAgora(new Date()), intervaloMs);
    return () => window.clearInterval(id);
  }, [intervaloMs]);
  return agora;
}

export default function App() {
  const agora = useAgora();
  const forcarAberto = new URLSearchParams(location.search).has("aberto");

  const [mouseEmCima, setMouseEmCima] = useState(false);
  const [fixado, setFixado] = useState(forcarAberto); // aberto pelo atalho ou enquanto digita
  const [texto, setTexto] = useState("");
  const [notas, setNotas] = useState<Nota[]>([]);
  const [salvou, setSalvou] = useState(false);

  const entalhoRef = useRef<HTMLDivElement>(null);
  const conteudoRef = useRef<HTMLDivElement>(null);
  const campoRef = useRef<HTMLInputElement>(null);
  const timerFechar = useRef<number | undefined>(undefined);

  const aberto = mouseEmCima || fixado || texto.length > 0;

  const recarregar = useCallback(async () => {
    try {
      setNotas(await listarRecentes());
    } catch (erro) {
      console.error("Não foi possível ler as notas", erro);
    }
  }, []);

  useEffect(() => {
    if (!noTauri) document.body.classList.add("previa");
    void recarregar();
  }, [recarregar]);

  // Hover: o núcleo avisa quando o cursor entra ou sai da área clicável.
  const aoHover = useCallback((dentro: boolean) => {
    window.clearTimeout(timerFechar.current);
    if (dentro) {
      setMouseEmCima(true);
    } else {
      timerFechar.current = window.setTimeout(() => setMouseEmCima(false), ATRASO_FECHAR_MS);
    }
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
      void definirRegiaoClicavel({
        x: r.left - folga,
        y: 0,
        width: r.width + folga * 2,
        height: r.height + folga,
      });
    };
    enviar();
    const obs = new ResizeObserver(enviar);
    obs.observe(entalho);
    return () => obs.disconnect();
  }, [aberto]);

  async function enviar(e: React.FormEvent) {
    e.preventDefault();
    if (!texto.trim()) return;
    await salvarNota(texto);
    setTexto("");
    setSalvou(true);
    window.setTimeout(() => setSalvou(false), 750);
    await recarregar();
  }

  function aoTeclar(e: React.KeyboardEvent<HTMLInputElement>) {
    if (e.key === "Escape") {
      setTexto("");
      setFixado(false);
      setMouseEmCima(false);
      campoRef.current?.blur();
    }
  }

  const mac = plataforma() === "mac";

  return (
    <main className="palco">
      <div
        ref={entalhoRef}
        className="entalho"
        data-aberto={aberto}
        data-salvou={salvou}
        onMouseEnter={() => aoHover(true)}
        onMouseLeave={() => aoHover(false)}
      >
        <div className="recolhido" aria-hidden={aberto}>
          <span className="relogio-mini">{formatarHora(agora)}</span>
          <span className="aura" />
        </div>

        <div className="aberto" ref={conteudoRef} aria-hidden={!aberto}>
          <header className="cabecalho">
            <span className="relogio">{formatarHora(agora)}</span>
            <div className="saudacao">
              <strong>{saudacao(agora)}</strong>
              <span>{formatarData(agora)}</span>
            </div>
          </header>

          <form className="comando" onSubmit={enviar}>
            <input
              ref={campoRef}
              value={texto}
              onChange={(e) => setTexto(e.target.value)}
              onKeyDown={aoTeclar}
              onFocus={() => setFixado(true)}
              onBlur={() => setFixado(false)}
              placeholder="Anote algo e aperte Enter"
              aria-label="Anotar"
              tabIndex={aberto ? 0 : -1}
            />
            <span className="teclas" aria-label={`Atalho: ${mac ? "Command" : "Ctrl"}, Shift, Espaço`}>
              <kbd>{mac ? "⌘" : "Ctrl"}</kbd>
              <kbd>⇧</kbd>
              <kbd>Espaço</kbd>
            </span>
          </form>

          <section className="notas" aria-live="polite">
            {notas.length === 0 ? (
              <p className="vazio">Nada anotado ainda. Escreva acima e aperte Enter.</p>
            ) : (
              <>
                <h2>Anotações recentes</h2>
                <ul>
                  {notas.map((n) => (
                    <li key={n.id}>
                      <span>{n.texto}</span>
                      <time dateTime={n.criadoEm}>{haQuanto(n.criadoEm, agora)}</time>
                    </li>
                  ))}
                </ul>
              </>
            )}
          </section>
        </div>

        <span className="horizonte" />
      </div>
    </main>
  );
}
