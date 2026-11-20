# Vertil App

App de desktop (Windows, Mac, Linux) que junta três coisas no topo da tela:
um **assistente com IA e voz**, um **modo foco** com bloqueio de sites e apps,
e uma **biblioteca de wallpapers animados**. Venda primeiro pelo site (Cakto),
depois pela Steam. Público: quem trabalha no computador o dia todo, no Brasil primeiro.

Leia antes de mudar algo grande: `docs/especificacao.md` (o produto),
`docs/design.md` (o visual) e a fase atual em `docs/fases/`.

## Stack

- Tauri 2: núcleo em Rust (`src-tauri/`) + interface em React e TypeScript (`src/`).
- SQLite local via `tauri-plugin-sql` (arquivo `vertil.db`). Migrações em `src-tauri/src/lib.rs`.
- Atalho global via `tauri-plugin-global-shortcut`.
- Fontes empacotadas com `@fontsource` (o app precisa funcionar offline).

## Regras do projeto

1. **Código de sistema operacional só em `src-tauri/src/platform/`** (`windows.rs`, `macos.rs`, `linux.rs`),
   todos com as mesmas funções. O resto do app nunca usa `#[cfg(target_os)]` diretamente.
2. **Cada função nova nasce no Windows** e depois é portada. Se não funcionar bem num sistema,
   fica desligada nele com aviso ao usuário.
3. **Ações destrutivas sempre pedem confirmação** (apagar, desligar, fechar programas, mexer em arquivos)
   e, quando possível, podem ser desfeitas.
4. **Migrações do banco nunca são editadas** depois de lançadas: crie uma nova versão.
5. **Comandos do assistente** (a partir da Fase 1) seguem um formato único: nome, descrição para a IA,
   parâmetros, se pede confirmação. Ficam em `src-tauri/src/assistant/`.
6. **Visual segue `docs/design.md`**: só os tokens de `src/styles/tokens.css`, nada de cor solta,
   sem degradês decorativos, âmbar só para estados em andamento, sem CAIXA ALTA em rótulos.
7. **Textos da interface em português**, frases curtas, sem exclamação. Prepare para tradução
   (inglês na Fase 7): nada de texto montado com concatenação de pedaços.
8. Chaves de API e dados do usuário ficam só no computador dele. Nunca enviar dados sem o usuário pedir.

## Antes de dizer que terminou

- `npm run build` (checa TypeScript e gera a interface) sem erros.
- `cd src-tauri && cargo check` e `cargo clippy` sem erros.
- Liste o que precisa ser testado à mão no Windows, no Mac e no Linux.
- Atualize este arquivo se uma regra ou estrutura mudou.

## Comandos

- `npm install` — dependências da interface.
- `npm run dev` — só a interface no navegador (prévia com fundo simulado; `?aberto` força o entalho aberto).
- `npm run tauri dev` — o app nativo completo.
- `npm run tauri build` — gera o instalador do sistema atual.

## Estrutura

```
src/                     interface React
  App.tsx                entalho: estados recolhido e aberto
  lib/ambiente.ts        ponte com o núcleo (funciona também fora do Tauri)
  lib/notas.ts           notas no SQLite
  styles/tokens.css      cores, fontes, raios, movimento
src-tauri/
  src/lib.rs             inicialização, atalho, bandeja, migrações
  src/notch.rs           posição da janela, região clicável, vigia do cursor
  src/platform/          código específico de cada sistema
  tauri.conf.json        janela transparente sem borda, sempre no topo
docs/                    especificação, design e fases
.github/workflows/       builds dos 3 sistemas
```
