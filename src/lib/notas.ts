// Notas salvas no SQLite local (arquivo vertil.db na pasta de dados do app).
// No navegador, usa o localStorage para a prévia funcionar.

import { noTauri } from "./ambiente";

export interface Nota {
  id: number;
  texto: string;
  criadoEm: string; // ISO 8601, UTC
}

const BANCO = "sqlite:vertil.db";
const CHAVE_PREVIA = "vertil.notas";

type Banco = Awaited<ReturnType<typeof import("@tauri-apps/plugin-sql").default.load>>;
let banco: Promise<Banco> | null = null;

function abrir(): Promise<Banco> {
  if (!banco) {
    banco = import("@tauri-apps/plugin-sql").then((m) => m.default.load(BANCO));
  }
  return banco;
}

function lerPrevia(): Nota[] {
  try {
    return JSON.parse(localStorage.getItem(CHAVE_PREVIA) ?? "[]") as Nota[];
  } catch {
    return [];
  }
}

export async function listarRecentes(limite = 3): Promise<Nota[]> {
  if (!noTauri) return lerPrevia().slice(0, limite);
  const db = await abrir();
  const linhas = await db.select<{ id: number; texto: string; criado_em: string }[]>(
    "SELECT id, texto, criado_em FROM notas ORDER BY id DESC LIMIT $1",
    [limite],
  );
  // O SQLite grava "AAAA-MM-DD HH:MM:SS" em UTC; convertemos para ISO.
  return linhas.map((l) => ({
    id: l.id,
    texto: l.texto,
    criadoEm: l.criado_em.replace(" ", "T") + "Z",
  }));
}

export async function salvarNota(texto: string): Promise<void> {
  const limpo = texto.trim();
  if (!limpo) return;
  if (!noTauri) {
    const notas = lerPrevia();
    const nova: Nota = { id: Date.now(), texto: limpo, criadoEm: new Date().toISOString() };
    try {
      localStorage.setItem(CHAVE_PREVIA, JSON.stringify([nova, ...notas].slice(0, 50)));
    } catch {
      /* armazenamento indisponível na prévia: ignora */
    }
    return;
  }
  const db = await abrir();
  await db.execute("INSERT INTO notas (texto) VALUES ($1)", [limpo]);
}
