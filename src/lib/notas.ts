import { abrirBanco, gravarPrevia, isoDoSqlite, lerPrevia, usarPrevia } from "./banco";

export interface Nota {
  id: number;
  texto: string;
  criadoEm: string; // ISO 8601
}

export async function listarNotas(limite = 3): Promise<Nota[]> {
  if (usarPrevia) return lerPrevia<Nota>("notas").slice(0, limite);
  const db = await abrirBanco();
  const linhas = await db.select<{ id: number; texto: string; criado_em: string }[]>(
    "SELECT id, texto, criado_em FROM notas ORDER BY id DESC LIMIT $1",
    [limite],
  );
  return linhas.map((l) => ({ id: l.id, texto: l.texto, criadoEm: isoDoSqlite(l.criado_em) }));
}

export async function salvarNota(texto: string): Promise<void> {
  const limpo = texto.trim();
  if (!limpo) return;
  if (usarPrevia) {
    const nova: Nota = { id: Date.now(), texto: limpo, criadoEm: new Date().toISOString() };
    gravarPrevia("notas", [nova, ...lerPrevia<Nota>("notas")]);
    return;
  }
  const db = await abrirBanco();
  await db.execute("INSERT INTO notas (texto) VALUES ($1)", [limpo]);
}
