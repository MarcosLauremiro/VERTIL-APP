# Vertil App

Assistente, modo foco e wallpapers animados no topo da tela. Windows, Mac e Linux.

## Rodar no seu computador

1. Instale o [Node.js](https://nodejs.org) (versão LTS) e o [Rust](https://rustup.rs).
2. Dependências do sistema para o Tauri: veja [os pré-requisitos](https://v2.tauri.app/start/prerequisites/)
   (no Mac basta o Xcode Command Line Tools: `xcode-select --install`;
   no Windows, o Microsoft C++ Build Tools e o WebView2, que já vem no Windows 11).
3. Na pasta do projeto:

```bash
npm install
npm run tauri dev
```

A primeira compilação do Rust leva alguns minutos; as seguintes são rápidas.

Só a interface, no navegador: `npm run dev` e abra `http://localhost:1420` (ou `http://localhost:1420/?aberto`).

## Usar

- Passe o mouse no entalho no topo da tela para abrir.
- **Ctrl+Shift+Espaço** (Windows/Linux) ou **Cmd+Shift+Espaço** (Mac) abre com o campo pronto para digitar.
- Escreva e aperte Enter para salvar uma nota. Esc fecha.
- Para sair, use o ícone do Vertil na bandeja (Windows) ou na barra de menus (Mac).

## Gerar instaladores

Envie o projeto para o GitHub: o workflow em `.github/workflows/build.yml` gera os instaladores
dos 3 sistemas a cada push na `main`. Crie uma tag `v0.1.0` para gerar um rascunho de release.

Documentação: `CLAUDE.md`, `docs/especificacao.md`, `docs/design.md`, `docs/fases/`.
