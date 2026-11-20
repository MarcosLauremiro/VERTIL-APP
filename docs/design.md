# Sistema visual do Vertil App

## Conceito: hora violeta

A hora violeta é a luz do fim da tarde, quando o expediente acaba. O Vertil mora na borda de cima da tela, como uma linha do horizonte. A tela é escura para se fundir ao entalho do notebook, mas o tom é **ameixa**, não preto. O roxo é a luz do app. O âmbar aparece **só** quando algo está em andamento (timer, foco).

## Cores

| Nome | Hex | Papel |
| --- | --- | --- |
| Ameixa noturna | `#1B1029` | Fundo do entalho e dos painéis |
| Ameixa vidro | `#2A1A40` | Superfícies elevadas: campo de texto, itens |
| Violeta Vertil | `#8C52FF` | Cor da marca: aura, linha do horizonte, foco do teclado, botão principal |
| Lilás névoa | `#BBA6E8` | Texto secundário, ícones |
| Luar | `#F5F0FF` | Texto principal |
| Âmbar crepúsculo | `#FFB45E` | Somente estados em andamento (timer rodando, sessão de foco) |

Regras:
- Sem degradês decorativos. Brilho só onde ele informa algo (a aura e a linha do horizonte).
- O violeta não é usado em texto pequeno sobre o fundo escuro (contraste baixo). Para texto, use Luar ou Lilás névoa.
- O âmbar nunca aparece parado: se ele está na tela, algo está contando.

## Tipografia

| Fonte | Uso |
| --- | --- |
| Unbounded (500, 600) | Relógio, números grandes, marca. Larga e arredondada como o próprio entalho. Nunca em parágrafos. |
| Instrument Sans (400, 500, 600) | Toda a interface: textos, botões, campos. |

Escala (px): 12,5 · 14 · 16 · 20 · 30. Texto em frase normal, nada de CAIXA ALTA em rótulos. As fontes vão empacotadas no app (funciona offline).

## Forma e layout

- O entalho é o único elemento em forma de pílula: cantos de cima retos (ele "sai" da borda da tela), cantos de baixo com raio 22.
- Dentro dele, elementos com raio 12 (campos) e 8 (itens pequenos). A hierarquia de raios mostra o que é recipiente e o que é conteúdo.
- Conteúdo sempre alinhado à esquerda, com 20px de respiro.

```
Recolhido (236 x 38)
 ┌──────────────────────────┐
 │ 16:42                  ● │   ● = aura
 └──────────═══════─────────┘   ═ = linha do horizonte

Expandido (560 x auto)
 ┌────────────────────────────────────────────┐
 │ 16:42          Boa tarde                   │
 │                sexta-feira, 20 de novembro │
 │ ┌────────────────────────────────────────┐ │
 │ │ Anote algo ou peça ao Vertil   ⌘ ⇧ ␣   │ │
 │ └────────────────────────────────────────┘ │
 │ Anotações recentes                         │
 │ Ligar para o cliente             há 5 min  │
 └══════════════════════════════════════════──┘
```

## O elemento marcante: a linha do horizonte

Uma linha de luz violeta de 2px na borda de baixo do entalho. Ela **informa o estado**:

| Estado | Linha |
| --- | --- |
| Parado | Curta (28% da largura), fraca, centralizada |
| Mouse em cima / aberto | Ocupa toda a largura |
| Algo salvo | Varre da esquerda para a direita uma vez |
| Foco ou timer rodando (Fase 4) | Âmbar, cresce conforme o progresso da sessão |
| Ouvindo a voz (Fase 2) | Pulsa no ritmo do volume |

Todo o resto fica quieto. Movimento só como resposta: abrir, salvar, ouvir. Com "reduzir movimento" ligado no sistema, as transições viram troca direta.

## Voz dos textos

Verbos simples, frase curta, sem exclamação. O botão diz o que faz ("Salvar nota"), e a confirmação repete a mesma palavra ("Nota salva"). Tela vazia sempre diz o que fazer a seguir.

## Temas (futuro, depois da Steam)

O tema "hora violeta" é o padrão. No futuro, o usuário poderá trocar o tema (por exemplo, uma versão clara e outros tons), e temas da comunidade podem vir junto com o Workshop.

Para isso funcionar sem retrabalho, a regra já vale desde a Fase 0: **toda cor sai de `src/styles/tokens.css`**. Um tema novo será só outro conjunto de valores para as mesmas variáveis (aplicado com um atributo `data-tema` no `<html>`). Não crie cor solta em nenhum componente.
