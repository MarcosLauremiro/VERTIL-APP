# Fase 1: assistente

**Objetivo:** o campo do entalho vira um assistente. O usuário escreve em português do dia a dia e o Vertil executa.

A fase foi dividida em duas partes:

- **1a: comandos locais** (sem IA, sem internet). Código pronto; falta testar no Mac.
- **1b: IA por API** para pedidos livres. Próxima etapa.

## 1a: comandos locais

### Entregas

- [x] Interpretador em Rust (`src-tauri/src/assistant/`) com testes automáticos (`cargo test`)
- [x] Timer com nome: "timer de 10 minutos para a pizza", "timer de meia hora"
- [x] Lembrete por tempo ou horário: "me lembra de ligar pro cliente em 30 minutos", "lembrete às 16h30 reunião", "me lembra amanhã às 9h de pagar o boleto"
- [x] Tarefas: "tarefa revisar proposta", "minhas tarefas"; concluir clicando no círculo
- [x] Notas: "anota que preciso comprar café"
- [x] Cálculo: "quanto é 15% de 230", "12*7+3", "10 vezes 3 mais 2"
- [x] Abrir site ou app: "abre o youtube", "abre g1.com.br", "abre o spotify"
- [x] Pedido não reconhecido vira nota e o Vertil avisa
- [x] Timer rodando aparece no entalho recolhido; a linha do horizonte fica âmbar e cresce com o progresso
- [x] Timer ou lembrete que vence abre o entalho com um alerta e um som curto
- [x] Banco: migrações 2 (lembretes) e 3 (tarefas)
- [x] Registro de comandos com descrição para a IA (`assistant::definicoes`), usado na 1b

### Limitações conhecidas

- Timers ficam na memória: se o app fechar, eles somem. Lembretes ficam salvos.
- Lembretes vencidos enquanto o app estava fechado tocam assim que ele abre.
- "Abrir app" no Windows usa `start` e só encontra apps registrados no sistema.

### Testes à mão (Mac)

| Frase ou ação | Esperado | Mac |
| --- | --- | --- |
| `timer de 1 minuto para o café` | Entalho recolhido mostra "Café" e a contagem; linha âmbar cresce | |
| Esperar o timer acabar | Entalho abre com alerta âmbar e som; "Fechar" some com o alerta | |
| `me lembra de beber água em 2 minutos` | Aparece em "Agora" com o horário; toca em 2 min | |
| `lembrete amanhã às 9h pagar boleto` | Aparece em "Agora" como "amanhã, 09:00" | |
| Fechar e abrir o app | Lembrete continua na lista | |
| `tarefa revisar proposta` e clicar no círculo | Tarefa aparece e some ao concluir | |
| `quanto é 15% de 230` | Resposta "15% de 230 = 34,5" | |
| `abre o youtube` | Abre no navegador padrão | |
| `abre o spotify` (ou outro app instalado) | Abre o app | |
| `qual a capital da França` | "Ainda não entendo esse pedido. Salvei como nota." | |

## 1b: IA por API (próxima)

- IA com chamada de ferramentas usando `assistant::definicoes`: Gemini e Groq (cota grátis) primeiro, depois Claude e OpenAI.
- Frases que o interpretador local não entende vão para a IA; as simples continuam sem IA.
- Chave de API guardada criptografada no computador; tela de configurações em janela separada.
- Comando de clima (Open-Meteo, sem chave).
- Critério de pronto: 20 frases livres de teste funcionando, nenhuma ação destrutiva sem confirmação.
