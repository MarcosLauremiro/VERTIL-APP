// Ponte com o interpretador do núcleo (src-tauri/src/assistant).

import { noTauri } from "./ambiente";

/** Espelho do enum `Intencao` do Rust (campo `comando` diz qual é). */
export type Intencao =
  | { comando: "timer"; segundos: number; rotulo: string | null }
  | { comando: "lembrete"; texto: string; quando_ms: number }
  | { comando: "criar_tarefa"; texto: string }
  | { comando: "listar_tarefas" }
  | { comando: "nota"; texto: string }
  | { comando: "calculo"; expressao: string; resultado: number }
  | { comando: "abrir_site"; url: string }
  | { comando: "abrir_app"; nome: string }
  | { comando: "falta_info"; mensagem: string }
  | { comando: "desconhecido"; texto: string };

async function chamar<T>(nome: string, args: Record<string, unknown>): Promise<T> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<T>(nome, args);
}

export async function interpretar(texto: string): Promise<Intencao> {
  // Na prévia do navegador não há núcleo Rust: tudo vira nota.
  if (!noTauri) return { comando: "desconhecido", texto };
  return chamar<Intencao>("interpretar", { texto });
}

export async function abrirSite(url: string): Promise<void> {
  if (!noTauri) return;
  await chamar("abrir_site", { url });
}

export async function abrirApp(nome: string): Promise<void> {
  if (!noTauri) return;
  await chamar("abrir_app", { nome });
}
