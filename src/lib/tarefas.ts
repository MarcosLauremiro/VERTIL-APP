import { abrirBanco, gravarPrevia, lerPrevia, usarPrevia } from "./banco";

export interface Tarefa {
  id: number;
  texto: string;
  feita: boolean;
}

type Linha = { id: number; texto: string; feita: number };

/** Tarefas abertas, das mais antigas para as mais novas. */
export async function listarAbertas(): Promise<Tarefa[]> {
  if (usarPrevia) {
    return lerPrevia<Tarefa>("tarefas")
      .filter((t) => !t.feita)
      .sort((a, b) => a.id - b.id);
  }
  const db = await abrirBanco();
  const linhas = await db.select<Linha[]>(
    "SELECT id, texto, feita FROM tarefas WHERE feita = 0 ORDER BY id",
  );
  return linhas.map((l) => ({ id: l.id, texto: l.texto, feita: Boolean(l.feita) }));
}

export async function criarTarefa(texto: string): Promise<void> {
  if (usarPrevia) {
    gravarPrevia("tarefas", [...lerPrevia<Tarefa>("tarefas"), { id: Date.now(), texto, feita: false }]);
    return;
  }
  const db = await abrirBanco();
  await db.execute("INSERT INTO tarefas (texto) VALUES ($1)", [texto]);
}

export async function concluirTarefa(id: number): Promise<void> {
  if (usarPrevia) {
    gravarPrevia(
      "tarefas",
      lerPrevia<Tarefa>("tarefas").map((t) => (t.id === id ? { ...t, feita: true } : t)),
    );
    return;
  }
  const db = await abrirBanco();
  await db.execute("UPDATE tarefas SET feita = 1 WHERE id = $1", [id]);
}
