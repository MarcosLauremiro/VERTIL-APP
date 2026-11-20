# Especificação do Vertil App

Resumo do plano completo (documento "Plano de desenvolvimento — Assistente de desktop + Wallpapers").

## Produto

Um app que fica no topo da tela, como o entalho de um notebook, e junta:

| Módulo | O que faz | Versão |
| --- | --- | --- |
| Assistente | Chat de texto, timers, lembretes, tarefas, clima, cálculos, abrir apps e arquivos, histórico de copiados | v1 |
| Voz | "Ei, Vertil" offline, segurar atalho para falar, voz do assistente, interromper | v1 (Windows), Mac e Linux na v1.1 |
| IA | IA local grátis ou chave própria (Claude, ChatGPT, Gemini, Groq); a IA escolhe e executa comandos | v1, começando pela API |
| Modo foco | Bloqueio de URLs e apps por lista, Pomodoro, modo rígido, relatório de tempo, wallpaper e silêncio automáticos | v1 |
| Integrações e rotinas | Google Agenda, Gmail, Notion, Trello, Slack; rotinas ("modo trabalho") | Agenda e rotinas na v1 |
| Agente no PC | Delegar tarefas para Claude Code ou Codex; organizar pastas com confirmação | v2 |
| Wallpapers | Vídeo, imagem, GIF, página web; biblioteca própria; importação; Workshop da Steam | v1 sem Workshop |

A ideia central: o wallpaper muda com o modo. No modo foco, o desktop fica calmo, as distrações são bloqueadas e o assistente para de falar.

## Fases

| Fase | Entrega | Prazo aproximado |
| --- | --- | --- |
| 0 | Base: janela no topo, atalho global, SQLite, builds dos 3 sistemas | 1 semana |
| 1 | Assistente + IA por API | 2 semanas |
| 2 | Voz no Windows | 2 semanas |
| 3 | Wallpapers (vídeo, imagem, importação) | 3 semanas |
| 4 | Modo foco | 2 semanas |
| 5 | Produto vendável: licença, Cakto, site, instalador assinado, beta | 2 semanas |
| — | **Lançamento no site (Windows), por volta da semana 12** | |
| 6 | Mac, Linux, IA local, Google Agenda, rotinas | 4 semanas |
| 7 | Steam: Steamworks, Workshop, inglês, página da loja | 3 semanas |
| — | **Lançamento na Steam, por volta da semana 19** | |
| Futuro | Temas: trocar as cores do app (claro, escuro, outros tons), temas da comunidade junto com o Workshop | depois da Steam |

## Decisões já tomadas

- Stack: Tauri 2 + React + TypeScript; libmpv para vídeo; sherpa-onnx para voz; llama.cpp para IA local.
- Venda: site primeiro (Cakto/PIX), Steam depois. Na Steam, o app se apresenta como app de wallpapers e personalização do desktop (categoria aceita pela Valve).
- Visual: "hora violeta" (ver `docs/design.md`).
- Nome: Vertil App. Palavra de ativação: "Ei, Vertil".
