# Protocolo futuro: percepção humana de NPCs sociais

**Status:** plano para preparar e pré-registrar quando os sistemas sociais e a experiência de jogo estiverem funcionais. Nenhum participante foi recrutado e nenhum resultado foi coletado.

## Pergunta principal

Em interações comparáveis dentro de Gala of the Hidden, jogadores conseguem distinguir NPCs controlados pelo Artificial Culture de outros jogadores? Quais pistas levam a essa identificação, e como a origem percebida afeta confiança, coerência social e decisões de interação?

O estudo precisa avaliar interações ao longo de uma comunidade: relatos, respostas, relações e ações posteriores. Uma fala isolada, uma conta reproduzida corretamente ou um NPC que passa despercebido em uma única cena não demonstra que uma comunidade parece viva.

## Base e limites

A triagem bibliográfica local destaca experimentos que comparam interação humana com agentes sociais em jogos, incluindo Jesso et al. (2020), além de estudos sobre influência social, memória, reputação e normas. Esses trabalhos ajudam a definir controles e medidas, mas diferem do jogo, da tarefa e do tipo de agente. Registre as fontes e os métodos efetivamente consultados no protocolo final; a revisão local ainda é parcial.

## Pré-requisitos do jogo

Não iniciar o estudo confirmatório até que:

- O jogo suporte sessões completas e repetíveis, sem depender de comandos de desenvolvedor.
- NPCs mantenham memória social entre sessões ou entre as etapas previstas, com comportamento observável baseado nela.
- Uma comunidade de NPCs consiga interagir sem roteiro fixo por personagem; seja possível guardar logs de eventos, contatos, intenções e resultados.
- Existam trajetórias de NPCs e jogadores em condições sociais comparáveis e um método de separar suas identidades no log.
- Erros, desconexões e resultados não confirmados possam ser detectados e excluídos segundo regras pré-registradas.
- O fluxo de dados, o consentimento e a revisão ética aplicável estejam aprovados antes do recrutamento.

Se o jogo ainda não tiver diálogo/interação social suficientemente rico, limite-se a um estudo de navegação ou decisão específico. Não chame esse resultado de validação de comunidade ou de “confundido com outro jogador”.

## Desenho recomendado

Fazer primeiro um piloto de usabilidade para verificar cenários, duração, logs, compreensão das perguntas e funcionamento técnico. Usar o piloto para estimar variabilidade e calcular o tamanho da amostra confirmatória; não fixar N por conveniência nem reutilizar o piloto como confirmação sem plano prévio.

No estudo confirmatório, cada participante encontra agentes humanos e NPCs em episódios sociais equivalentes. Randomize e contrabalanceie ordem, cenário e contraparte; use versões de evento equivalentes para reduzir aprendizagem do roteiro. O rótulo da contraparte fica oculto durante o jogo. O consentimento informa que a sessão pode incluir pessoas e agentes automatizados; não afirmar que todos são humanos. Após cada episódio, peça a estimativa da probabilidade de ter interagido com uma pessoa, sem interromper a interação para não alterar o comportamento.

Inclua episódios em que o participante fala diretamente com uma contraparte e episódios em que observa uma rede de NPCs interagir. A parte de observação permite avaliar se as relações e consequências entre NPCs fazem sentido mesmo sem o participante ser o interlocutor.

## Medidas

**Desfecho primário:** capacidade de discriminar NPC de jogador, usando as estimativas de probabilidade e a identidade real da contraparte. Pré-registre a métrica principal (por exemplo, AUC ou acurácia balanceada) e seu intervalo de incerteza.

**Desfechos secundários:**

- Confiança na contraparte e coerência percebida da resposta com o que ela poderia saber.
- Naturalidade da interação e consistência da relação ao longo do tempo.
- Decisões observáveis: procurar novamente, compartilhar informação, ajudar, cooperar, evitar ou confrontar.
- Capacidade de explicar quais pistas fundamentaram a classificação, por pergunta aberta após os episódios.
- Em episódios observados, se o participante consegue reconstruir quem ouviu o quê, de quem e por que a comunidade se dividiu ou convergiu.

Use as mesmas escalas e perguntas para NPCs e pessoas. Registre também familiaridade prévia com jogos sociais, uso de IA e experiência no jogo; colete apenas dados demográficos necessários à pergunta.

## Hipóteses e análise pré-registradas

Antes da coleta confirmatória, defina:

1. Qual resultado de discriminação contaria como próximo o bastante do acaso para a afirmação pretendida. Escolha a margem de equivalência com pessoas relevantes ao projeto, antes de ver os resultados, e dimensione a amostra para esse teste.
2. Qual diferença em confiança, coerência e decisões de interação seria material para o jogador.
3. Como tratar participantes que não completam sessões, problemas de rede, episódios sem exposição suficiente e respostas faltantes.
4. Quais modelos compararão condição de contraparte, cenário e ordem, levando em conta medidas repetidas da mesma pessoa e a identidade da contraparte.
5. Quais análises são confirmatórias e quais são exploratórias.

Não concluir “indistinguível” apenas porque um teste não encontrou diferença estatística. Reporte estimativas e intervalos; se a margem de equivalência não foi pré-definida ou a amostra não consegue detectá-la, descreva o resultado como inconclusivo. Acurácia próxima ao acaso também não basta sozinha: confira calibração, confiança, decisões reais e pistas nos relatos livres.

## Ética, privacidade e transparência

Obtenha a revisão/aprovação ética exigida pela instituição e jurisdição antes de recrutar. Explique que pessoas e agentes automatizados podem aparecer, quais dados serão registrados e como desistir. Se as identidades individuais ficarem ocultas para evitar viés, registre essa informação no consentimento, siga as condições aprovadas e faça debriefing após a sessão. Não use conversa privada real nem peça dados sensíveis dentro do jogo.

Armazene identificadores pseudonimizados, separe contatos dos dados de pesquisa, limite acesso e prazo de retenção, e publique apenas transcrições revisadas para remover informações identificáveis. Registre conflitos de interesse, versão do jogo, configuração do AC, seeds e alterações entre rodadas.

## Materiais a preparar antes do estudo

- Protocolo pré-registrado, justificativa de amostra e plano analítico.
- Conjunto de eventos e redes sociais equivalentes entre condições, com seeds reservadas para a avaliação.
- Instruções de sessão, consentimento, questionários e debriefing revisados.
- Logs anonimizados de observações, testemunhos, contatos, intenções, resultados e decisões do participante.
- Plano para falhas técnicas, interrupções, moderação e retirada dos dados.
- Relatório que apresente limitações, resultados nulos e exemplos de pistas percebidas, sem declarar realismo universal.

## Referência próxima

Jesso et al. (2020), comparação experimental de interação com parceiros humanos e agentes sociais em um jogo multiplayer: [DOI 10.3389/frobt.2020.531805](https://doi.org/10.3389/frobt.2020.531805). Use como referência para desenho e medidas, não como garantia de que seus achados se transferem para Gala of the Hidden.
