// Aviso sonoro curto (duas notas), gerado na hora: não precisa de arquivo de áudio.

let contexto: AudioContext | null = null;

export function tocarAviso(): void {
  try {
    contexto ??= new AudioContext();
    const ctx = contexto;
    const notas = [880, 1174.66]; // lá e ré: subida suave
    notas.forEach((frequencia, i) => {
      const inicio = ctx.currentTime + i * 0.18;
      const osc = ctx.createOscillator();
      const ganho = ctx.createGain();
      osc.type = "sine";
      osc.frequency.value = frequencia;
      ganho.gain.setValueAtTime(0, inicio);
      ganho.gain.linearRampToValueAtTime(0.18, inicio + 0.02);
      ganho.gain.exponentialRampToValueAtTime(0.0001, inicio + 0.5);
      osc.connect(ganho).connect(ctx.destination);
      osc.start(inicio);
      osc.stop(inicio + 0.55);
    });
  } catch {
    /* sem áudio disponível: o alerta visual continua */
  }
}
