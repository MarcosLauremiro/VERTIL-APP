// Ponte entre a interface e o núcleo Rust.
// Fora do Tauri (rodando `npm run dev` no navegador), tudo vira operação vazia
// para que a interface possa ser revisada sem o app nativo.

import type { UnlistenFn } from "@tauri-apps/api/event";

export const noTauri =
  typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

export type Plataforma = "mac" | "windows" | "linux";

export function plataforma(): Plataforma {
  const ua = navigator.userAgent.toLowerCase();
  if (ua.includes("mac")) return "mac";
  if (ua.includes("win")) return "windows";
  return "linux";
}

export async function ouvir<T>(
  evento: string,
  aoReceber: (dado: T) => void,
): Promise<UnlistenFn> {
  if (!noTauri) return () => {};
  const { listen } = await import("@tauri-apps/api/event");
  return listen<T>(evento, (e) => aoReceber(e.payload));
}

export interface Regiao {
  x: number;
  y: number;
  width: number;
  height: number;
}

/** Informa ao núcleo qual área da janela é clicável. O resto deixa o mouse passar. */
export async function definirRegiaoClicavel(regiao: Regiao): Promise<void> {
  if (!noTauri) return;
  const { invoke } = await import("@tauri-apps/api/core");
  await invoke("set_hit_region", { region: regiao });
}
