import { abrirBanco, gravarPrevia, lerPrevia, usarPrevia } from "./banco";

export interface Lembrete {
  id: number;
  texto: string;
  quandoMs: number;
  avisado: boolean;
}

type Linha = { id: number; texto: string; quando_ms: number; avisado: number };
const deLinha = (l: Linha): Lembrete => ({
  id: l.id,
  texto: l.texto,
  quandoMs: Number(l.quando_ms),
  avisado: Boolean(l.avisado),
});

/** Lembretes ainda não avisados, do mais próximo para o mais distante. */
export async function listarPendentes(): Promise<Lembrete[]> {
  if (usarPrevia) {
    return lerPrevia<Lembrete>("lembretes")
      .filter((l) => !l.avisado)
      .sort((a, b) => a.quandoMs - b.quandoMs);
  }
  const db = await abrirBanco();
  const linhas = await db.select<Linha[]>(
    "SELECT id, texto, quando_ms, avisado FROM lembretes WHERE avisado = 0 ORDER BY quando_ms",
  );
  return linhas.map(deLinha);
}

export async function criarLembrete(texto: string, quandoMs: number): Promise<void> {
  if (usarPrevia) {
    const novo: Lembrete = { id: Date.now(), texto, quandoMs, avisado: false };
    gravarPrevia("lembretes", [novo, ...lerPrevia<Lembrete>("lembretes")]);
    return;
  }
  const db = await abrirBanco();
  await db.execute("INSERT INTO lembretes (texto, quando_ms) VALUES ($1, $2)", [texto, quandoMs]);
}

/** Marca como avisado (o lembrete tocou ou o usuário o dispensou). */
export async function marcarAvisado(id: number): Promise<void> {
  if (usarPrevia) {
    gravarPrevia(
      "lembretes",
      lerPrevia<Lembrete>("lembretes").map((l) => (l.id === id ? { ...l, avisado: true } : l)),
    );
    return;
  }
  const db = await abrirBanco();
  await db.execute("UPDATE lembretes SET avisado = 1 WHERE id = $1", [id]);
}
