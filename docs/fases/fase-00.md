# Fase 0: base do app

**Status:** concluída no Mac em 20/11/2026. Os testes no Windows e no Linux ficam para depois.

## Entregas

- [x] Projeto Tauri 2 + React + TypeScript com a estrutura de pastas do CLAUDE.md
- [x] Janela transparente, sem borda, sempre no topo, centralizada no topo da tela
- [x] Entalho recolhido (relógio + aura) que abre ao passar o mouse
- [x] Fora do entalho, o mouse atravessa a janela (região clicável + vigia do cursor)
- [x] Atalho global Ctrl+Shift+Espaço (Cmd no Mac) abre o entalho com o campo em foco
- [x] SQLite com migração; o campo salva notas e mostra as 3 mais recentes
- [x] Ícone na bandeja com "Abrir o Vertil" e "Sair do Vertil"
- [x] Sistema visual "hora violeta" (tokens, fontes empacotadas, movimento reduzido respeitado)
- [x] Workflow do GitHub Actions que gera instaladores para Windows, Mac (Apple e Intel) e Linux

## Critérios de pronto (testar à mão)

| Teste | Windows | Mac | Linux |
| --- | --- | --- | --- |
| O entalho aparece no topo central ao abrir o app | | ok | |
| Passar o mouse abre; tirar o mouse fecha | | ok | |
| Clicar ao lado do entalho clica no app de trás | | ok | |
| Atalho abre e foca o campo; Esc fecha | | ok | |
| Nota salva continua lá depois de fechar e abrir o app | | ok | |
| Bandeja: "Sair do Vertil" fecha o app | | ok | |
| Não aparece na barra de tarefas / Dock | | ok | |

## Pendências conhecidas

- **Mac:** a janela fica abaixo da barra de menus. Encostar no entalho físico exige subir o nível da janela (Fase 6).
- **Linux Wayland:** posição, atalho e cursor dependem do ambiente gráfico (Fase 6).
- **Vários monitores:** o entalho vai só para o monitor principal.
