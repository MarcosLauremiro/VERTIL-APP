// Acesso ao SQLite local (vertil.db). No navegador, cada tabela vira uma lista no localStorage
// para a prévia funcionar sem o app nativo.

import { noTauri } from "./ambiente";

const BANCO = "sqlite:vertil.db";

type Banco = Awaited<ReturnType<typeof import("@tauri-apps/plugin-sql").default.load>>;
let banco: Promise<Banco> | null = null;

export function abrirBanco(): Promise<Banco> {
  if (!banco) {
    banco = import("@tauri-apps/plugin-sql").then((m) => m.default.load(BANCO));
  }
  return banco;
}

export const usarPrevia = !noTauri;

/** O SQLite grava "AAAA-MM-DD HH:MM:SS" em UTC; converte para ISO. */
export const isoDoSqlite = (valor: string) => valor.replace(" ", "T") + "Z";

export function lerPrevia<T>(tabela: string): T[] {
  try {
    return JSON.parse(localStorage.getItem(`vertil.${tabela}`) ?? "[]") as T[];
  } catch {
    return [];
  }
}

export function gravarPrevia<T>(tabela: string, linhas: T[]): void {
  try {
    localStorage.setItem(`vertil.${tabela}`, JSON.stringify(linhas.slice(0, 100)));
  } catch {
    /* armazenamento indisponível na prévia: ignora */
  }
}
