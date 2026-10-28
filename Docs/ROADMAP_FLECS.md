# Roteiro até um Artificial Culture utilizável em Flecs

Atualizado em 27/10/2026. Este plano leva do vertical slice atual a um módulo que Gala of the Hidden possa chamar e cujos NPCs produzam histórias sociais compreensíveis e diferentes entre si. “Utilizável” significa integração pelo contrato do servidor, estado subjetivo por NPC, interações sociais que mudam decisões, execução reproduzível para depuração e uma avaliação qualitativa com jogadores. Não significa provar que NPCs são humanos em qualquer jogo.

## Definição de pronto

O módulo está pronto para um piloto quando GotH puder entregar observações limitadas, contatos e resultados confirmados; NPCs puderem manter interpretações e relações próprias; esses estados influenciarem o que fazem e contam; e um conjunto de cenários sociais produzir trajetórias coerentes em várias sementes. Jogadores/revisores cegos devem avaliar trajetórias e conseguir confundir algumas interações com as de outros jogadores. Isso é uma meta empírica, não uma condição que a implementação possa declarar sozinha.

O servidor de GotH continua sendo a autoridade sobre posição, navegação, inventário, disponibilidade de recursos, eventos físicos e sucesso das ações. Artificial Culture conserva estado cognitivo e emite intenções. Não deve fazer uma segunda simulação do mundo.

## Sequência de trabalho

### 0. Congelar a linha de base e preparar a compilação

**Estado:** concluída para o build local Windows: preset MSVC, Release build e demo executados. Ver [`IMPLEMENTATION_STATUS.md`](./IMPLEMENTATION_STATUS.md). O clone limpo ainda depende de acesso à rede para obter Flecs.

**Fazer:** preservar a branch/tag `archive/rust-bevy`; manter `master` como linha ativa C++/Flecs; configurar CMake/MSVC e compilar o `ac_demo`. Guardar o log e registrar o comportamento observado do demo como referência. Corrigir primeiro falhas de build ou de interface que impeçam repetir esse passo.

**Pronto quando:** uma máquina Windows limpa, com as dependências documentadas, configura e compila do README; é possível iniciar o demo e ver a troca observação → intenção → resultado.

**Não ampliar** cálculos de necessidades ou neuroquímica aqui. A finalidade é uma base executável.

### 1. Fechar o adaptador Flecs/servidor

**Estado:** base do contrato v1 e ciclo de vida local implementados; validar ainda contra o adapter real do GotH e resolver persistência/threading somente conforme necessário.

**Fazer:** fixar os tipos e IDs que atravessam a fronteira, regras de ciclo de vida de entidade, relógio, sequência idempotente de intents, rejeição/aceitação e efeitos reais devolvidos pelo host. Definir como remover NPCs e limpar eventos pendentes. Manter a simulação isolada de Flecs onde possível, mas usar o `flecs::world` do jogo na integração.

**Pronto quando:** um adaptador mínimo do GotH consegue criar NPC, entregar snapshot de percepção, reportar contato, chamar `step`, executar ou rejeitar uma intenção no servidor e devolver resultado confirmado sem duplicar ações.

**Entrega:** contrato versionado do adapter e pequeno exemplo executável dentro do host ou de um adapter falso com a mesma interface.

### 2. Fazer o estado subjetivo de eventos

**Estado:** estrutura inicial concluída para armazenar observações individuais por evento; sem transmissão, replay narrativo ou influência sobre decisões ainda.

**Fazer:** separar `WorldEvent` (fato autoritativo que o host comunica) de `EventInterpretation` (o que cada agente percebe/entende). Guardar observador, instante, detalhes vistos e limites da observação. Um evento ambíguo pode originar interpretações diferentes. NPC ausente não conhece evento sem rota de informação.

**Pronto quando:** cenários “sala fechada” e “duas testemunhas” de `CULTURAL_SCENARIOS.md` mantêm fatos e relatos distintos; trocar observador ou linha de visão muda a lembrança de modo explicável.

**Entrega:** componentes e API para receber eventos observados e consultar interpretações individuais; primeiros cenários narrados no runner, ainda sem afirmar aprovação humana.

### 3. Dar proveniência e transmissão às crenças

**Estado:** a camada de testemunho começou: cada ouvinte pode registrar fala, fonte imediata, origem atribuída e versão anterior citada. Recontagem produzida por NPC, correção e revisão de crença ainda não existem.

**Fazer:** substituir rumores por afirmações ligadas a evento/conteúdo, fonte imediata, cadeia de transmissão, confiança na fonte, força/certeza, instante e transformações durante a recontagem. Registrar omissão, acréscimo, contestação e evidência em primeira mão como operações observáveis. Não usar uma única string global como verdade compartilhada.

**Pronto quando:** “uma história muda ao viajar” preserva quem ouviu de quem e permite correção ou persistência do relato; contato não sobrescreve automaticamente crenças prévias.

**Entrega:** registros de crença/proveniência e eventos de transmissão consultáveis para ferramentas de diagnóstico.

### 4. Modelar relações pessoais e memória de interações

**Fazer:** adicionar memória de interações e avaliações direcionadas A→B: confiança, familiaridade e expectativas apoiadas em episódios. Atualizar essas relações após promessas, erros, ajuda e conflito confirmados pelo host. Traits podem modular a interpretação, mas não substituir histórico entre pares.

**Pronto quando:** a mesma afirmação de um NPC é recebida de forma diferente por dois ouvintes com experiências diferentes; contrafactual que remove uma interação altera resposta posterior por uma razão rastreável.

**Entrega:** cenário de reputação além de uma conversa, com trilha explicável do estado.

### 5. Conectar conhecimento e relações a ações sociais

**Fazer:** permitir que crenças e relações alterem intenções disponíveis: alertar, perguntar, compartilhar, evitar, cooperar ou confrontar, conforme ações que GotH realmente suporte. Toda ação passa pela validação do servidor. Atualizar cognição só a partir do resultado observado. Evitar árvore de diálogo roteirizada que force a mesma personalidade/decisão.

**Pronto quando:** um evento social muda comportamento posterior; duas trajetórias divergem com histórico diferente; uma ação rejeitada não é lembrada como realizada.

**Entrega:** loop observação→interpretação→intenção social→resultado do servidor→memória.

### 6. Representar grupos e normas sem “cultura global” uniforme

**Fazer:** introduzir grupos pela participação/contato reconhecidos pelo mundo, expectativas compartilhadas e evidências de aprovação/sanção. Manter diferenças e dissenso dentro de um grupo. Uma convenção frequente e uma norma cobrada são estados distintos. Só usar mecanismos que tenham cenários e evidência de referência registrados.

**Pronto quando:** o cenário de norma mostra adesão, dissenso, sanção e mudança ao longo do tempo; grupos diferentes podem atribuir sentidos diferentes ao mesmo evento sem flags uniformes.

**Entrega:** normas locais emergentes ou aprendidas com proveniência, conectadas a decisões dos agentes.

### 7. Construir o runner de cenários caóticos, determinístico e inspecionável

**Fazer:** criar harness C++ que injeta fatos do servidor, snapshots parciais, contatos e resultados; permita controlar seed; grave transcript/event log por agente e permita replay. Variar sementes, ordem/timing de contatos, agentes que não comparecem, relatos contraditórios, falha de ações e evidência tardia. Manter o evento externo fixo para comparar trajetórias.

**Pronto quando:** os seis cenários em `CULTURAL_SCENARIOS.md` rodam automaticamente, podem ser repetidos pela seed e mostram observações, fontes, crenças, relações, decisões e efeitos. Os relatórios descrevem diversidade coerente e casos incoerentes; não passam por coincidir com uma resposta narrativa única.

**Entrega:** comando de execução do harness e transcrições legíveis, com resumos de variação entre seeds. Os checks unitários continuam úteis para bugs, mas são sinalizados como integridade técnica, não evidência de realismo.

### 8. Fazer avaliação de percepção com pessoas

**Fazer:** preparar estudo piloto dentro do jogo, com NPC e parceiro humano em situações comparáveis; ocultar a identidade quando eticamente apropriado; medir identificação, confiança da resposta, coerência social percebida e decisões de interação. Pré-definir cenários, métricas e análise. Entrevistas/respostas livres ajudam a explicar cues percebidos. Separar resultado por cenário e não apenas uma média global.

**Pronto quando:** resultados e limitações estão registrados; falsos sinais de bot orientam iterações; uma segunda rodada testa se mudanças reduziram esses sinais em conteúdo e cenários não usados no ajuste.

**Entrega:** protocolo, conjunto de estímulos/transcrições e relatório de resultados. Sem participantes humanos, chamar apenas de preparação de avaliação, não de validação.

### 9. Integrar o piloto ao GotH e congelar o primeiro corte

**Fazer:** integrar o adapter com versão e configuração explícitas; limitar população/taxa de simulação; instrumentar performance e eventos; salvar/restaurar estados necessários; observar casos de desconexão e reload. Atualizar GDD e arquitetura somente com comportamento que passou pelos cenários e avaliação definida.

**Pronto quando:** equipe pode habilitar AC em um mapa/piloto, inspecionar decisões e reproduzir uma falha usando seed/log; desligar módulo sem corromper autoridade ou progresso do mundo.

**Entrega:** release de piloto C++/Flecs, instruções de build/execução, dados de configuração e lista conhecida de limitações.

## Primeiro ciclo recomendado

Para chegar a algo utilizável sem uma migração enorme, concluir nesta ordem:

1. Base build + contrato de adapter (etapas 0–1).
2. Eventos, interpretações e proveniência (etapas 2–3).
3. Relações com memória de episódios (etapa 4).
4. Uma ação social que afeta comportamento e recebe resultado do servidor (etapa 5).
5. Runner com os cenários 1–4 e replay de várias seeds (etapa 7 em paralelo com implementação).
6. Só então grupos/normas (etapa 6), avaliação com jogadores (etapa 8) e piloto GotH (etapa 9).

Essa ordem entrega um primeiro módulo de sociedade pequena — testemunho, confiança e resposta — antes de tentar simular uma cultura inteira. O runner começa cedo para expor estados impossíveis enquanto as abstrações ainda são baratas de mudar.

## Regras de escopo e aceite

- Não usar “todos os cálculos estão corretos” como conclusão de realismo.
- Cada cenário diz quais fatos cada pessoa poderia observar e qual histórico sustenta a reação; mais de um desfecho pode ser aceitável.
- Rodar várias seeds e mudanças pequenas de ordem/tempo. Aleatoriedade sem causa observável não conta como comportamento humano.
- Antes de portar outro subsistema Rust, mapear sua função para um cenário e contrato do host; não fazer paridade por volume de código.
- Cada regra inspirada em paper aponta ao registro da biblioteca, população/tarefa e limite. Paper gera hipótese, não comprovação do NPC.
- Manter eventos do mundo autoritativos fora da memória subjetiva; nunca dar onisciência a um NPC para simplificar.
- Não implementar a integração real de GotH até o adapter e as interfaces C++/Flecs terem um vertical slice verificável localmente.

## Estado conhecido

- C++20/Flecs v4.1.6, `CognitionModule`, demo local e contrato conceitual já existem.
- Build Windows foi reportado compilando com CMake 4.4.4, MSVC 19.51, Windows SDK 10.0.26100; o projeto ainda depende de FetchContent no primeiro configure.
- Necessidades, Big Five, memória simples de recursos, propostas/resultados de ação e rumor de texto escalar já estão no vertical slice.
- Ainda faltam proveniência, interpretação de eventos, confiança relacional, memória de episódios, grupos/normas e decisão baseada em cultura.
- `CULTURAL_SCENARIOS.md` descreve seis critérios; atualmente são critérios de projeto, não cenários C++ aprovados.
- Não há integração live nem bateria de aceitação de comportamento no código atual.

## Relação com migração antiga

Este plano substitui a ideia de portar tudo por paridade literal com Rust. O código Rust continua arquivado; comportamento só migra se for necessário aos cenários e ao contrato do jogo. A pesquisa dos artigos continua em `Docs/Research/systematic/`; ela informa mecanismos e avaliações, enquanto este roadmap organiza a entrega de software.
