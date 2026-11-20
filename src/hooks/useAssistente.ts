import { useCallback, useEffect, useRef, useState } from "react";
import { abrirApp, abrirSite, interpretar, type Intencao } from "../lib/assistente";
import { criarLembrete, listarPendentes, marcarAvisado, type Lembrete } from "../lib/lembretes";
import { listarNotas, salvarNota, type Nota } from "../lib/notas";
import { concluirTarefa, criarTarefa, listarAbertas, type Tarefa } from "../lib/tarefas";
import { formatarDuracao, formatarHora, formatarNumero, formatarQuando } from "../lib/tempo";
import { tocarAviso } from "../lib/som";

export interface Timer {
  id: number;
  rotulo: string | null;
  inicioMs: number;
  fimMs: number;
}

export interface Alerta {
  id: string;
  titulo: string;
  detalhe: string;
}

export interface Resposta {
  texto: string;
  erro?: boolean;
}

/**
 * Estado do assistente: timers (em memória), lembretes, tarefas e notas (SQLite),
 * a última resposta e os alertas que estão tocando.
 */
export function useAssistente() {
  const [agoraMs, setAgoraMs] = useState(() => Date.now());
  const [timers, setTimers] = useState<Timer[]>([]);
  const [lembretes, setLembretes] = useState<Lembrete[]>([]);
  const [tarefas, setTarefas] = useState<Tarefa[]>([]);
  const [notas, setNotas] = useState<Nota[]>([]);
  const [resposta, setResposta] = useState<Resposta | null>(null);
  const [alertas, setAlertas] = useState<Alerta[]>([]);
  const proximoId = useRef(1);

  const recarregar = useCallback(async () => {
    try {
      const [l, t, n] = await Promise.all([listarPendentes(), listarAbertas(), listarNotas(2)]);
      setLembretes(l);
      setTarefas(t);
      setNotas(n);
    } catch (erro) {
      console.error("Não foi possível ler o banco local", erro);
    }
  }, []);

  useEffect(() => {
    void recarregar();
    // Outras partes da interface (ex.: prévia ?demo) podem pedir para reler o banco.
    const aoPedir = () => void recarregar();
    window.addEventListener("vertil:recarregar", aoPedir);
    return () => window.removeEventListener("vertil:recarregar", aoPedir);
  }, [recarregar]);

  const alertar = useCallback((alerta: Alerta) => {
    setAlertas((atuais) => (atuais.some((a) => a.id === alerta.id) ? atuais : [...atuais, alerta]));
    tocarAviso();
  }, []);

  // Relógio de 1 segundo: atualiza contagens e dispara timers e lembretes vencidos.
  useEffect(() => {
    const id = window.setInterval(() => setAgoraMs(Date.now()), 1000);
    return () => window.clearInterval(id);
  }, []);

  useEffect(() => {
    const vencidos = timers.filter((t) => t.fimMs <= agoraMs);
    if (vencidos.length) {
      setTimers((atuais) => atuais.filter((t) => t.fimMs > agoraMs));
      vencidos.forEach((t) =>
        alertar({
          id: `timer-${t.id}`,
          titulo: t.rotulo ?? "Timer",
          detalhe: `O timer de ${formatarDuracao(Math.round((t.fimMs - t.inicioMs) / 1000))} terminou.`,
        }),
      );
    }
    const devidos = lembretes.filter((l) => l.quandoMs <= agoraMs);
    if (devidos.length) {
      setLembretes((atuais) => atuais.filter((l) => l.quandoMs > agoraMs));
      devidos.forEach((l) => {
        alertar({ id: `lembrete-${l.id}`, titulo: l.texto, detalhe: `Lembrete das ${formatarHora(l.quandoMs)}.` });
        void marcarAvisado(l.id);
      });
    }
  }, [agoraMs, timers, lembretes, alertar]);

  const executar = useCallback(
    async (texto: string) => {
      let intencao: Intencao;
      try {
        intencao = await interpretar(texto);
      } catch (erro) {
        setResposta({ texto: `Não consegui entender agora: ${String(erro)}`, erro: true });
        return;
      }

      try {
        setResposta(await aplicar(intencao));
      } catch (erro) {
        setResposta({ texto: String(erro), erro: true });
      }
      await recarregar();
    },
    [recarregar],
  );

  async function aplicar(i: Intencao): Promise<Resposta> {
    switch (i.comando) {
      case "timer": {
        const inicio = Date.now();
        const timer = { id: proximoId.current++, rotulo: i.rotulo, inicioMs: inicio, fimMs: inicio + i.segundos * 1000 };
        setTimers((atuais) => [...atuais, timer].sort((a, b) => a.fimMs - b.fimMs));
        const duracao = formatarDuracao(i.segundos);
        return { texto: i.rotulo ? `${i.rotulo}: timer de ${duracao} começou.` : `Timer de ${duracao} começou.` };
      }
      case "lembrete":
        await criarLembrete(i.texto, i.quando_ms);
        return { texto: `Vou lembrar ${quandoNaFrase(i.quando_ms)}: ${i.texto}.` };
      case "criar_tarefa":
        await criarTarefa(i.texto);
        return { texto: "Tarefa adicionada." };
      case "listar_tarefas": {
        const abertas = await listarAbertas();
        if (abertas.length === 0) return { texto: "Nenhuma tarefa aberta." };
        return { texto: abertas.length === 1 ? "Você tem 1 tarefa aberta." : `Você tem ${abertas.length} tarefas abertas.` };
      }
      case "nota":
        await salvarNota(i.texto);
        return { texto: "Nota salva." };
      case "calculo":
        return { texto: `${i.expressao} = ${formatarNumero(i.resultado)}` };
      case "abrir_site":
        await abrirSite(i.url);
        return { texto: `Abrindo ${new URL(i.url).hostname.replace(/^www\./, "")}.` };
      case "abrir_app":
        await abrirApp(i.nome);
        return { texto: `Abrindo ${i.nome}.` };
      case "falta_info":
        return { texto: i.mensagem };
      case "desconhecido":
        await salvarNota(i.texto);
        return { texto: "Ainda não entendo esse pedido. Salvei como nota." };
    }
  }

  function quandoNaFrase(ms: number): string {
    const quando = formatarQuando(ms, new Date());
    return quando.startsWith("amanhã")
      ? `amanhã às ${formatarHora(ms)}`
      : quando.includes(",")
        ? `em ${quando}`
        : `às ${quando}`;
  }

  const cancelarTimer = useCallback((id: number) => {
    setTimers((atuais) => atuais.filter((t) => t.id !== id));
    setResposta({ texto: "Timer cancelado." });
  }, []);

  const dispensarLembrete = useCallback(
    async (id: number) => {
      await marcarAvisado(id);
      setResposta({ texto: "Lembrete removido." });
      await recarregar();
    },
    [recarregar],
  );

  const concluir = useCallback(
    async (id: number) => {
      await concluirTarefa(id);
      setResposta({ texto: "Tarefa concluída." });
      await recarregar();
    },
    [recarregar],
  );

  const fecharAlerta = useCallback((id: string) => {
    setAlertas((atuais) => atuais.filter((a) => a.id !== id));
  }, []);

  return {
    agoraMs,
    timers,
    lembretes,
    tarefas,
    notas,
    resposta,
    alertas,
    executar,
    cancelarTimer,
    dispensarLembrete,
    concluir,
    fecharAlerta,
    /** Só para a prévia no navegador (?demo): preenche a tela com exemplos. */
    semearDemo: (setar: { timers: Timer[]; alertas: Alerta[]; resposta: Resposta }) => {
      setTimers(setar.timers);
      setAlertas(setar.alertas);
      setResposta(setar.resposta);
    },
  };
}
