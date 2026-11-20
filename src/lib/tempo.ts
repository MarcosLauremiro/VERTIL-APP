const hora = new Intl.DateTimeFormat("pt-BR", { hour: "2-digit", minute: "2-digit" });
const data = new Intl.DateTimeFormat("pt-BR", { weekday: "long", day: "numeric", month: "long" });
const diaCurto = new Intl.DateTimeFormat("pt-BR", { weekday: "short", day: "2-digit", month: "2-digit" });
const numero = new Intl.NumberFormat("pt-BR", { maximumFractionDigits: 4 });

export const formatarHora = (d: Date | number) => hora.format(d);
export const formatarData = (d: Date) => data.format(d);
export const formatarNumero = (n: number) => numero.format(n);

export function saudacao(d: Date): string {
  const h = d.getHours();
  if (h < 5) return "Boa noite";
  if (h < 12) return "Bom dia";
  if (h < 18) return "Boa tarde";
  return "Boa noite";
}

export function haQuanto(iso: string, agora: Date): string {
  const minutos = Math.floor((agora.getTime() - new Date(iso).getTime()) / 60000);
  if (minutos < 1) return "agora";
  if (minutos < 60) return `há ${minutos} min`;
  const horas = Math.floor(minutos / 60);
  if (horas < 24) return `há ${horas} h`;
  const dias = Math.floor(horas / 24);
  return dias === 1 ? "ontem" : `há ${dias} dias`;
}

/** Contagem regressiva: "9:41" ou "1:02:03". */
export function formatarContagem(ms: number): string {
  const total = Math.max(0, Math.ceil(ms / 1000));
  const h = Math.floor(total / 3600);
  const m = Math.floor((total % 3600) / 60);
  const s = total % 60;
  const ss = String(s).padStart(2, "0");
  return h > 0 ? `${h}:${String(m).padStart(2, "0")}:${ss}` : `${m}:${ss}`;
}

/** Duração legível: "10 min", "1 h 30 min", "45 s". */
export function formatarDuracao(segundos: number): string {
  const h = Math.floor(segundos / 3600);
  const m = Math.floor((segundos % 3600) / 60);
  const s = segundos % 60;
  const partes = [h && `${h} h`, m && `${m} min`, !h && !m && `${s} s`].filter(Boolean);
  return partes.join(" ");
}

/** Quando um lembrete vai tocar: "16:30", "amanhã, 09:00" ou "sáb., 22/11, 09:00". */
export function formatarQuando(ms: number, agora: Date): string {
  const alvo = new Date(ms);
  const mesmoDia = (a: Date, b: Date) => a.toDateString() === b.toDateString();
  const amanha = new Date(agora);
  amanha.setDate(agora.getDate() + 1);
  if (mesmoDia(alvo, agora)) return formatarHora(alvo);
  if (mesmoDia(alvo, amanha)) return `amanhã, ${formatarHora(alvo)}`;
  return `${diaCurto.format(alvo)}, ${formatarHora(alvo)}`;
}
