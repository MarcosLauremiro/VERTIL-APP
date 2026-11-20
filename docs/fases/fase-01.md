# Fase 1: assistente + IA por API

**Objetivo:** o campo do entalho vira um assistente. O usuário escreve em português do dia a dia e o Vertil executa.

## Entregas

- Registro de comandos em `src-tauri/src/assistant/` com formato único (nome, descrição para a IA, parâmetros em JSON Schema, se pede confirmação).
- Comandos iniciais: timer com nome, lembrete com hora, tarefa (criar, listar, concluir), nota, cálculo, abrir app, abrir site, clima.
- Entendimento local rápido para frases simples ("timer de 10 minutos") sem gastar IA.
- IA por API com chamada de ferramentas: Gemini e Groq (cota grátis) primeiro, depois Claude e OpenAI. Chave guardada criptografada no PC.
- Respostas aparecem no entalho; timers rodando aparecem no entalho recolhido com a linha do horizonte em âmbar.
- Tela de configurações (janela separada) para chave de API e preferências.

## Critérios de pronto

- 20 frases de teste (lista em `docs/fases/frases-teste.md`, criar) funcionam com IA e as simples funcionam sem IA.
- Nenhuma ação destrutiva roda sem confirmação.
- Funciona sem internet para os comandos locais.
