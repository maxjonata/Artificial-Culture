# Validação standalone de uma comunidade de NPCs

**Status:** harness experimental para validar fluxos sociais do Artificial Culture sem iniciar o jogo. Não demonstra realismo cultural nem substitui a integração com GotH.

## O que este teste exercita

`ac_community_simulator` executa vários NPCs AC em uma sociedade pequena. Um host de teste cria um mundo social simplificado, oferece contatos, executa ou rejeita intenções, entrega testemunhos realmente compartilhados e devolve resultados observados. O AC escolhe parceiros, decide quando compartilhar um relato lembrado e guarda o resultado na perspectiva do agente.

Cada execução contém duas comunidades de seis NPCs e um NPC isolado. Duas pessoas começam com relatos divergentes sobre o mesmo evento. A topologia pode incluir uma ligação entre as comunidades ou deixá-las separadas. Os contatos variam por seed. As transcrições preservam o caminho de proveniência. Em uma fração das retransmissões, o host de teste reduz a confiança do relato; essa perturbação é artificial e serve para exercitar a trilha, não vem de uma estimativa publicada.

O simulador testa se os mecanismos já implementados podem produzir circulação social rastreável. Ele não gera diálogo, não decide se uma crença é verdadeira, não constrói normas ou grupos dentro do AC, e não simula química cerebral. O host de teste executa 90% das ações, escolhe aleatoriamente respostas de aceitação/contestação para testemunhos em proporção 70/30 e reciprocidade/recusa em proporção 65/35 para conversas; em 20% das retransmissões reduz a confiança por um fator de 0,8. Estes valores servem para provocar caminhos diferentes e testar integridade, não são parâmetros estimados de estudos humanos. A regra de escolha de parceiro continua sendo uma hipótese simples, não uma escala de confiança validada.

## Evidência que orienta os cenários

Os estudos selecionados na síntese local sugerem dimensões úteis para variar e observar, sem fornecer constantes universais para o jogo:

- Informação social pode mudar a distribuição e a imprevisibilidade das escolhas coletivas; por isso comparamos trajetórias sob seeds e caminhos de contato diferentes. [Salganik et al. (2006)](https://doi.org/10.1126/science.1121066).
- A estrutura de conectividade pode afetar a diversidade e a acumulação cultural em uma tarefa específica; por isso comparamos comunidades separadas e conectadas por uma ponte. Não presumimos que uma topologia seja sempre superior. [Derex & Boyd (2016)](https://doi.org/10.1073/pnas.1518798113).
- Convenções frequentes e normas esperadas/cobradas precisam ser tratadas como fenômenos diferentes; estudos de mudança normativa e de sanção também mostram que sanção não implica benefício coletivo. Os recursos atuais ainda não implementam esses mecanismos. [Centola et al. (2018)](https://doi.org/10.1126/science.aas8827), [Przepiorka et al. (2022)](https://doi.org/10.1177/23780231221124556), [Abbink et al. (2017)](https://doi.org/10.1038/s41467-017-00731-0), [Herrmann et al. (2008)](https://doi.org/10.1126/science.1153808).

A triagem bibliográfica local ainda registra lacunas de cobertura e não deve ser descrita como exaustiva ou concluída. Estes estudos justificam perguntas e contrastes, não provam que o simulador atual replica comunidades humanas.

## Executar no Windows

Na raiz do repositório:

```powershell
cmake --build --preset windows-msvc-release
ctest --preset windows-msvc-release
```

Uma execução comparável, com 32 comunidades em cada condição:

```powershell
./out/build/windows-msvc-release/Release/ac_community_simulator.exe --runs 32 --ticks 32 --seed 100 --topology bridge --output out/community-bridge.txt
./out/build/windows-msvc-release/Release/ac_community_simulator.exe --runs 32 --ticks 32 --seed 100 --topology separate --output out/community-separate.txt
```

Para inspecionar as transmissões em uma única trajetória:

```powershell
./out/build/windows-msvc-release/Release/ac_community_simulator.exe --runs 1 --ticks 24 --seed 42 --topology bridge --verbose --output out/community-trace.txt
```

`--topology bridge` adiciona uma aresta entre os dois grupos; `--topology separate` não a adiciona. Para o contrafactual, mantenha seed e ticks iguais. O binário grava resumo por execução, entregas, recusas do host e alcance de cada relato inicial. Os totais agregados contam instâncias de receptores entre mundos recriados a cada run, não indivíduos únicos de uma população persistente. A execução conectada também falha se nenhuma transmissão atravessar a ponte no lote. `--verbose` acrescenta uma linha por testemunho transmitido.

## Verificações automáticas

O harness falha se ocorrer qualquer uma destas situações:

1. Um NPC recebe contato com alguém fora das arestas da rede.
2. Um compartilhamento é proposto sem o agente ter recebido o testemunho de origem.
3. O host aceita uma cadeia de proveniência quebrada ou um evento diferente do que a intenção referenciou.
4. Uma ação executada não vira memória social do ator, ou uma ação rejeitada é lembrada como realizada.
5. O NPC isolado ganha uma observação do evento ou testemunho sem caminho social.
6. Na condição separada, um dos relatos cruza de grupo.
7. Na condição com ponte, nenhuma transmissão atravessa a ligação em todo o lote.

Esses são testes de integridade do cenário. A condição conectada não exige que todos os NPCs recebam ambos os relatos; distribuição desigual e histórias dependentes da seed são resultados a inspecionar, não erros automáticos.

## Como revisar resultados

Ao comparar os arquivos:

- Confira se toda nova versão aponta para um pai anterior do mesmo evento e identifica quem a contou imediatamente.
- Compare quantos NPCs de cada grupo ouviram cada origem, quantas transmissões atravessaram a ponte e quais indivíduos concentraram as conversas.
- Leia trajetórias representativas e trajetórias raras. Pergunte se a diferença decorre de contatos e experiências que aparecem no log.
- Repita com várias seeds; não escolha uma execução favorável como demonstração.
- Rode um contrafactual com a mesma seed e remova a ponte. A informação externa não deve atravessar grupos desconectados.
- Registre falhas e trajetórias estranhas. Não ajuste o sistema para forçar consenso ou uma interpretação narrativa única.

Antes de interpretar dados comunitários como “cultura”, o AC ainda precisa representar e testar crenças revisadas por evidência, relações direcionais mais ricas, grupos, expectativas normativas, dissenso e sanção. O simulador atual cobre uma rede de contatos e circulação de testemunhos, não uma comunidade humana completa.
