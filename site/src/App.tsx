import { useEffect, useMemo, useState, type CSSProperties } from 'react'

type Language = 'pt' | 'en'

const text = {
  pt: {
    nav: ['A ideia', 'Arquitetura', 'Na prática', 'Cenários', 'Pesquisa'],
    heroTag: 'CAMADA COGNITIVA · GALA OF THE HIDDEN',
    heroTitleA: 'O mundo não é o que acontece.',
    heroTitleB: 'É o que cada um consegue perceber.',
    heroBody: 'Artificial Culture é o sistema que dá aos NPCs necessidades, memória limitada e uma vida social em formação. Não uma inteligência que sabe tudo — mas pessoas artificiais tentando entender o mesmo mundo por lugares diferentes.',
    explore: 'Explore o projeto',
    watch: 'Ver a demonstração antiga',
    release: 'Em desenvolvimento',
    heroNote: 'Uma camada de cognição para os personagens de Gala of the Hidden.',
    manifestoLabel: 'A pergunta por trás do sistema',
    manifestoTitle: 'O que muda quando ninguém tem a história inteira?',
    manifestoBody: 'Em um mundo vivo, uma pessoa não sabe tudo o que o servidor sabe. Ela vê alguns recursos, encontra algumas pessoas, ouve uma versão do acontecimento. O projeto parte dessa assimetria: a cultura nasce entre percepções parciais, memória imperfeita e relações que ainda estão sendo construídas.',
    manifestoAside: 'A intenção é criar comportamentos sociais coerentes o bastante para que suas diferenças tenham sentido — sem confundir uma simulação com uma explicação completa da experiência humana.',
    boundaryLabel: '01 / Onde mora a autoridade',
    boundaryTitle: 'O jogo conhece o mundo. O NPC conhece um pedaço dele.',
    boundaryIntro: 'Artificial Culture não controla a realidade do jogo. Ela recebe uma visão filtrada, atualiza o estado interno do agente e sugere uma ação. O servidor do jogo continua sendo quem valida e executa.',
    worldTitle: 'GALA OF THE HIDDEN · AUTORIDADE',
    worldItems: ['Posição, navegação e colisão', 'Recursos disponíveis e interações', 'O que uma ação realmente produziu'],
    acTitle: 'ARTIFICIAL CULTURE · COGNIÇÃO',
    acItems: ['Percepções enviadas para cada agente', 'Necessidades, traços e memória de recursos', 'Rumores, propostas e resposta ao resultado'],
    flowCaption: 'Perceber não é saber. Propor não é executar.',
    pipelineLabel: '02 / Um ciclo, sem atalhos',
    pipelineTitle: 'Uma intenção precisa encontrar a realidade.',
    pipelineIntro: 'A fronteira entre cognição e autoridade evita duas simulações concorrentes do mesmo mundo. O retorno do servidor é parte do ciclo, não um detalhe de implementação.',
    steps: [
      ['01', 'Percepção', 'O host envia recursos e agentes visíveis. O NPC não descobre entidades ocultas por conta própria.'],
      ['02', 'Memória', 'Fatos observados sobre recursos podem continuar conhecidos mesmo quando eles não aparecem no próximo recorte de visão.'],
      ['03', 'Proposta', 'Necessidades e alvos conhecidos determinam uma intenção priorizada, enviada ao host em uma fila de ações.'],
      ['04', 'Confirmação', 'O host aceita ou rejeita a ação; só então Artificial Culture aplica o resultado cognitivo correspondente.'],
    ],
    demoLabel: '03 / Um recorte observável do código',
    demoTitle: 'De uma necessidade a uma ação proposta.',
    demoIntro: 'Este pequeno percurso corresponde à fronteira do adaptador de demonstração atual. Avance etapa por etapa para ver o que o NPC recebe e o que fica a cargo do jogo.',
    demoSteps: ['O adaptador observa alimento próximo', 'Fome: 0,25 · alvo: recurso 101', 'AC propõe find_food', 'GotH confirma +0,40 de satisfação'],
    demoButton: 'Avançar etapa',
    demoRestart: 'Recomeçar percurso',
    demoNote: 'Demonstração conceitual baseada no contrato atual; a integração jogável entre módulo, servidor e apresentação ainda está em construção.',
    hunger: 'Fome', thirst: 'Fome', observeTitle: 'Recorte de percepção recebido', observeLine1: 'Recurso visível', observeLine2: 'Alimento · ID 101 · distância 4', needTitle: 'Estado interno', needValue: '0,25 / 1,00', intentTitle: 'Fila de intenção', intentAction: 'find_food → alvo 101', intentUtility: 'proposta, ainda não executada', resultTitle: 'Retorno do jogo', resultAction: 'Ação aceita · fome +0,40', resultWait: 'O jogo ainda precisa validar e executar', scenarioMark: 'DADOS DO AC_DEMO',
    rumorLabel: '04 / Quando uma história circula', rumorTitle: 'Um contato abre uma possibilidade, não uma certeza.', rumorIntro: 'No código atual, um boato textual pode ser injetado e transmitido quando o host reporta um contato social. A transmissão é probabilística; a estrutura não preserva a origem do relato nem versões divergentes do mesmo evento.',
    rumorCalc: 'Explore a regra de transmissão', rumorCaption: 'Exemplo de cálculo com abertura ilustrativa de 0,6 e 0,4. Os valores descrevem o modelo, não pessoas reais.', rumorBase: 'Taxa-base', rumorDistance: 'Parâmetro de distância social', rumorChance: 'chance por contato nesta configuração', rumorFormula: 'p = base × [1 / (1 + distância × 0,01)] × fator de abertura de quem fala × fator de abertura de quem ouve', rumorDecay: 'Crenças decaem 0,01 por segundo. Uma transferência recebe 0,8 da força que o emissor tinha. O valor é uma regra de código, não uma medida validada de comportamento humano.',
    statusLabel: '05 / O que existe hoje', statusTitle: 'Uma fundação concreta, com limites declarados.', statusIntro: 'A migração para C++20 e Flecs está em andamento. O núcleo atual é um alicerce de integração e cognição; não é a antiga simulação Bevy inteira reescrita.',
    statusColumns: [
      { label: 'PORTADO · PARCIAL', title: 'Presente no núcleo C++', items: ['Necessidades normalizadas e decaimento', 'Traços Big Five', 'Memória de recursos observados', 'Propostas e resultados confirmados', 'Crenças rumorais e propagação por contato'] },
      { label: 'INTEGRAÇÃO · A SEGUIR', title: 'Trabalho por conectar', items: ['Adaptador do mundo Flecs de GotH', 'Validação e retorno das ações pelo servidor', 'Cenários narrativos reprodutíveis', 'Comparação de comportamento com o legado'] },
      { label: 'AINDA NÃO REPRESENTA', title: 'Limites do modelo atual', items: ['Confiança entre pares e reputação', 'Fonte e trajetória de cada relato', 'Grupos, normas e memória coletiva', 'Interpretações subjetivas de um evento', 'Aprendizado persistente e adaptação social'] },
    ],
    scenariosLabel: '06 / A régua é narrativa', scenariosTitle: 'Uma equação correta não basta para chamar algo de humano.', scenariosIntro: 'O projeto usa trajetórias qualitativas para perguntar se cada crença e decisão é compreensível a partir do que aquele agente viu, lembrou e ouviu. Os cenários abaixo são critérios de aceitação futuros — não funcionalidades que o C++ já passa.',
    scenarios: [
      ['Um relato de uma sala fechada', 'Quem não presenciou o fato não deve aprendê-lo magicamente. A informação precisa viajar por contatos plausíveis, mantendo quem contou o quê.'],
      ['Dois testemunhos, um acontecimento', 'Observações diferentes podem gerar relatos divergentes sobre o mesmo evento; transmitir não deve reduzir todos a uma única frase compartilhada.'],
      ['Uma história que muda', 'A cadeia de fontes, as correções e as interpretações precisam mudar com o percurso do relato.'],
      ['Reputação que sobrevive a uma conversa', 'Confiança precisa depender de uma história de interações entre pessoas, não de um único traço fixo de personalidade.'],
      ['Uma norma aparece no comportamento', 'Aprovação, punição e observação social devem explicar como normas podem emergir, enfraquecer ou ser contestadas.'],
      ['O mesmo evento, vários significados', 'Histórias de grupo podem influenciar o que importa para cada pessoa, sem forçar todos os membros a pensar igual.'],
    ],
    researchLabel: '07 / De onde vêm as hipóteses', researchTitle: 'A pesquisa orienta perguntas — não serve de selo de realismo.', researchBody: 'A biblioteca reúne literatura sobre cognição, memória, personalidade, interação e comportamento coletivo. O projeto separa mecanismos sugeridos por estudos empíricos de hipóteses interpretativas úteis para desenhar testes. Os arquivos de artigos ainda são uma coleção em avaliação, não uma base inteira aprovada.',
    researchRules: ['Mecanismos precisam de evidência empírica identificável e limites explícitos.', 'A percepção de comportamento humano precisa ser medida com participantes, em comparações controladas.', 'Uma simulação reproduzível ajuda a investigar um mecanismo; por si só, não prova realismo.'],
    legacyLabel: 'ARQUIVO DO PROJETO · VERSÃO BEVY', legacyTitle: 'Uma janela para a pesquisa que veio antes.', legacyBody: 'Este vídeo apresenta uma versão antiga construída com Rust e Bevy. Ele ajuda a entender a origem e a direção das ideias, mas não mostra o estado do módulo C++20 e Flecs descrito nesta página.', legacyNote: 'Demonstração histórica. A versão atual é uma migração parcial e ainda não está conectada à fatia jogável de Gala of the Hidden.',
    buildLabel: '08 / Para quem quer acompanhar de perto', buildTitle: 'Leia o código. Rode o adaptador.', buildIntro: 'Artificial Culture é C++20, usa Flecs v4.1.6 via CMake FetchContent e inclui um executável local de demonstração chamado ac_demo. O exemplo é um adaptador isolado — não uma integração pronta ao servidor do jogo.', buildCommand1: 'cmake -S . -B build -DCMAKE_BUILD_TYPE=Release', buildCommand2: 'cmake --build build --config Release', buildCommand3: '.\\build\\ac_demo.exe', copy: 'Copiar', copied: 'Copiado',
    linksLabel: 'Próximas leituras', links: ['Arquitetura e migração', 'Cenários culturais', 'Biblioteca de pesquisa', 'Código no GitHub', 'Projeto Gala of the Hidden'], sourcesLabel: '09 / Documentos de primeira mão', sourcesTitle: 'Explore as fontes do projeto.', sourcesIntro: 'A página resume o estado atual; estas são as referências técnicas e de pesquisa que a sustentam.', sourceCards: [['Arquitetura & migração', 'Fronteira de autoridade, componentes ativos e fases do port.'], ['Cenários culturais', 'Critérios narrativos e lacunas conhecidas da versão C++.'], ['Biblioteca de pesquisa', 'Método de seleção, limitações e estado das fontes.'], ['Núcleo cognitivo', 'Componentes, dados de entrada, intenções e resultados.'], ['Adaptador de demonstração', 'Exemplo que percorre observação, ação e rumores.'], ['Licença do projeto', 'CC BY-NC-SA 4.0 e condições de redistribuição.']],
    footerText: 'Um projeto em migração. Um mundo visto a partir de dentro.', footerLicense: 'Licenciado sob CC BY-NC-SA 4.0. Veja a licença e avalie as condições antes de distribuir adaptações.'
  },
  en: {
    nav: ['The idea', 'Architecture', 'In practice', 'Scenarios', 'Research'],
    heroTag: 'COGNITIVE LAYER · GALA OF THE HIDDEN',
    heroTitleA: 'The world is not what happens.', heroTitleB: 'It is what each person can perceive.',
    heroBody: 'Artificial Culture gives NPCs needs, limited memory, and a social life still taking shape. Not an intelligence that knows everything, but artificial people trying to make sense of the same world from different places.',
    explore: 'Explore the project', watch: 'See the older demo', release: 'In development', heroNote: 'A cognition layer for the characters of Gala of the Hidden.',
    manifestoLabel: 'The question behind the system', manifestoTitle: 'What changes when no one has the whole story?', manifestoBody: 'In a living world, a person does not know everything the server knows. They see some resources, meet some people, hear one account of an event. The project starts from that asymmetry: culture emerges between partial perceptions, imperfect memory, and relationships still being built.', manifestoAside: 'The intention is to create social behavior coherent enough for its differences to make sense—without mistaking a simulation for a complete explanation of human experience.',
    boundaryLabel: '01 / Where authority lives', boundaryTitle: 'The game knows the world. An NPC knows a piece of it.', boundaryIntro: 'Artificial Culture does not control game reality. It receives a filtered view, updates an agent’s internal state, and suggests an action. The game server remains responsible for validating and executing it.', worldTitle: 'GALA OF THE HIDDEN · AUTHORITY', worldItems: ['Position, navigation, and collision', 'Available resources and interactions', 'What an action actually produced'], acTitle: 'ARTIFICIAL CULTURE · COGNITION', acItems: ['Perceptions sent to each agent', 'Needs, traits, and resource memory', 'Rumors, proposals, and result feedback'], flowCaption: 'Perceiving is not knowing. Proposing is not executing.',
    pipelineLabel: '02 / One loop, no shortcuts', pipelineTitle: 'An intention has to meet reality.', pipelineIntro: 'The boundary between cognition and authority prevents two competing simulations of the same world. The server’s response is part of the loop, not an implementation detail.', steps: [
      ['01', 'Perception', 'The host sends visible resources and agents. An NPC does not discover hidden entities on its own.'], ['02', 'Memory', 'Observed facts about resources may remain known even when they disappear from the next perception snapshot.'], ['03', 'Proposal', 'Needs and known targets determine a prioritized intent, sent to the host through an action outbox.'], ['04', 'Confirmation', 'The host accepts or rejects the action; only then does Artificial Culture apply the corresponding cognitive result.']
    ],
    demoLabel: '03 / A visible slice of the code', demoTitle: 'From a need to a proposed action.', demoIntro: 'This short walkthrough follows the current ac_demo adapter. Step through the exact example: what the NPC receives, proposes, and learns from the host’s reply.', demoSteps: ['The adapter observes nearby food', 'Hunger: 0.25 · target: resource 101', 'AC proposes find_food', 'GotH confirms +0.40 satisfaction'], demoButton: 'Next step', demoRestart: 'Replay the walkthrough', demoNote: 'An illustrative trace of ac_demo, a local adapter example. The playable integration between module, server, and presentation is still being built.', thirst: 'Hunger', observeTitle: 'Perception snapshot received', observeLine1: 'Visible resource', observeLine2: 'Food · ID 101 · distance 4', needTitle: 'Internal state', needValue: '0.25 / 1.00', intentTitle: 'Intent outbox', intentAction: 'find_food → target 101', intentUtility: 'proposed, not yet executed', resultTitle: 'Game response', resultAction: 'Action accepted · hunger +0.40', resultWait: 'The game still needs to validate and execute', scenarioMark: 'AC_DEMO DATA',
    rumorLabel: '04 / When a story travels', rumorTitle: 'A contact creates a possibility, not a certainty.', rumorIntro: 'In the current code, a text rumor can be injected and transmitted when the host reports a social contact. Transmission is probabilistic; the structure does not preserve the account’s source or divergent versions of an event.', rumorCalc: 'Explore the transmission rule', rumorCaption: 'Example calculation using illustrative openness values of 0.6 and 0.4. These values describe the model, not real people.', rumorBase: 'Base rate', rumorDistance: 'Social-distance parameter', rumorChance: 'chance per contact with these settings', rumorFormula: 'p = base × [1 / (1 + distance × 0.01)] × speaker openness factor × listener openness factor', rumorDecay: 'Beliefs decay by 0.01 per second. A transfer receives 0.8 of the sender’s belief strength. This is a code rule, not a validated measure of human behavior.',
    statusLabel: '05 / What exists today', statusTitle: 'A concrete foundation, with declared limits.', statusIntro: 'The migration to C++20 and Flecs is underway. The current core is a foundation for cognition and integration; it is not the entire Bevy simulation rewritten.', statusColumns: [
      { label: 'PORTED · PARTIAL', title: 'Present in the C++ core', items: ['Normalized needs and decay', 'Big Five traits', 'Memory of observed resources', 'Proposals and confirmed results', 'Rumor beliefs and contact-based spread'] },
      { label: 'INTEGRATION · NEXT', title: 'Work still to connect', items: ['Adapter for GotH’s Flecs world', 'Server validation and action feedback', 'Replayable narrative scenarios', 'Behavior comparison with the legacy version'] },
      { label: 'NOT REPRESENTED YET', title: 'Limits of the current model', items: ['Trust between peers and reputation', 'Source and path of each account', 'Groups, norms, and collective memory', 'Subjective interpretations of an event', 'Persistent learning and social adaptation'] }
    ], scenariosLabel: '06 / The measure is narrative', scenariosTitle: 'A correct equation is not enough to call something human.', scenariosIntro: 'The project uses qualitative trajectories to ask whether each belief and choice makes sense given what an agent saw, remembered, and heard. These are future acceptance criteria—not features the C++ core already passes.', scenarios: [
      ['An account from a closed room', 'Someone who did not witness an event should not learn it by magic. Information should travel through plausible contacts, retaining who told whom.'], ['Two witnesses, one event', 'Different observations may produce divergent accounts; transmission should not compress them into one shared sentence.'], ['A story changes as it travels', 'Sources, corrections, and interpretations should change along the path of an account.'], ['Reputation outlives one conversation', 'Trust should depend on interaction history between people, not one fixed personality trait.'], ['A norm becomes visible through behavior', 'Approval, sanctions, and social observation should explain how norms emerge, weaken, or face dissent.'], ['The same event, several meanings', 'Group histories may influence what matters to each person without forcing everyone in the group to think alike.']
    ], researchLabel: '07 / Where the hypotheses come from', researchTitle: 'Research guides questions—it does not certify realism.', researchBody: 'The library gathers work on cognition, memory, personality, interaction, and collective behavior. The project distinguishes mechanisms suggested by empirical studies from interpretive hypotheses useful for test design. The papers on disk are still being reviewed; the collection has not all been vetted.', researchRules: ['Mechanisms need identifiable empirical evidence and explicit limits.', 'Perceived human behavior needs to be measured with participants in controlled comparisons.', 'A reproducible simulation can investigate a mechanism; by itself, it cannot prove realism.'],
    legacyLabel: 'PROJECT ARCHIVE · BEVY VERSION', legacyTitle: 'A window into the research that came before.', legacyBody: 'This video presents an older version built with Rust and Bevy. It helps explain where the ideas came from and where they are headed, but it does not show the C++20/Flecs module described on this page.', legacyNote: 'Historical demo. Today’s version is a partial migration and is not yet connected to the playable Gala of the Hidden slice.',
    buildLabel: '08 / For anyone who wants to follow along', buildTitle: 'Read the code. Run the adapter.', buildIntro: 'Artificial Culture is C++20, uses Flecs v4.1.6 via CMake FetchContent, and includes a local demo executable called ac_demo. It is an isolated adapter example—not a ready-to-ship connection to the game server.', buildCommand1: 'cmake -S . -B build -DCMAKE_BUILD_TYPE=Release', buildCommand2: 'cmake --build build --config Release', buildCommand3: './build/ac_demo', copy: 'Copy', copied: 'Copied', linksLabel: 'Further reading', links: ['Architecture and migration', 'Cultural scenarios', 'Research library', 'Code on GitHub', 'Gala of the Hidden'], sourcesLabel: '09 / First-hand project sources', sourcesTitle: 'Explore the project sources.', sourcesIntro: 'This page summarizes the current state; these are the technical and research references behind it.', sourceCards: [['Architecture & migration', 'Authority boundary, active components, and porting phases.'], ['Cultural scenarios', 'Narrative acceptance criteria and known C++ gaps.'], ['Research library', 'Selection method, limits, and source status.'], ['Cognition core', 'Components, inputs, action intents, and results.'], ['Demo adapter', 'Example trace of observation, action, and rumors.'], ['Project license', 'CC BY-NC-SA 4.0 and redistribution terms.']], footerText: 'A project in migration. A world seen from within.', footerLicense: 'Licensed under CC BY-NC-SA 4.0. Review the license and its terms before distributing adaptations.'
  }
} as const

const scenariosIds = ['closed-room', 'witnesses', 'story', 'reputation', 'norms', 'groups']
const stepsIds = ['perception', 'memory', 'proposal', 'confirmation']

function Icon({ name, size = 18 }: { name: string; size?: number }) {
  const common = { width: size, height: size, viewBox: '0 0 24 24', fill: 'none', stroke: 'currentColor', strokeWidth: 1.6, strokeLinecap: 'round' as const, strokeLinejoin: 'round' as const, 'aria-hidden': true as const }
  if (name === 'arrow') return <svg {...common}><path d="M5 12h14M13 6l6 6-6 6" /></svg>
  if (name === 'play') return <svg {...common}><path d="m9 6 10 6-10 6z" /></svg>
  if (name === 'pause') return <svg {...common}><path d="M8 6v12M16 6v12" /></svg>
  if (name === 'check') return <svg {...common}><path d="m5 12 4 4L19 6" /></svg>
  if (name === 'copy') return <svg {...common}><rect x="8" y="8" width="12" height="12" rx="2"/><path d="M16 8V5a1 1 0 0 0-1-1H5a1 1 0 0 0-1 1v10a1 1 0 0 0 1 1h3"/></svg>
  if (name === 'external') return <svg {...common}><path d="M14 4h6v6M20 4l-9 9"/><path d="M18 13v6a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1V7a1 1 0 0 1 1-1h6"/></svg>
  if (name === 'eye') return <svg {...common}><path d="M2 12s3.6-7 10-7 10 7 10 7-3.6 7-10 7S2 12 2 12Z"/><circle cx="12" cy="12" r="3"/></svg>
  if (name === 'brain') return <svg {...common}><path d="M12 4a3 3 0 0 0-5.8-1A3.5 3.5 0 0 0 4 9.6 3.5 3.5 0 0 0 5.3 16a3 3 0 0 0 5.7 2.3L12 17l1 1.3a3 3 0 0 0 5.7-2.3 3.5 3.5 0 0 0 1.3-6.4 3.5 3.5 0 0 0-2.2-6.6A3 3 0 0 0 12 4Z"/><path d="M12 4v14M7 7c2 0 3 1 3 3M17 7c-2 0-3 1-3 3M6 13c2 0 3 1 3 3M18 13c-2 0-3 1-3 3"/></svg>
  if (name === 'network') return <svg {...common}><circle cx="12" cy="12" r="2.5"/><circle cx="5" cy="6" r="2"/><circle cx="19" cy="6" r="2"/><circle cx="5" cy="18" r="2"/><circle cx="19" cy="18" r="2"/><path d="m7 7.5 3.2 3M17 7.5l-3.2 3M7 16.5l3.2-3M17 16.5l-3.2-3"/></svg>
  if (name === 'book') return <svg {...common}><path d="M4 5.5A2.5 2.5 0 0 1 6.5 3H20v17H6.5A2.5 2.5 0 0 1 4 17.5z"/><path d="M4 6h12M4 17h12M8 6v11"/></svg>
  if (name === 'code') return <svg {...common}><path d="m8 8-4 4 4 4M16 8l4 4-4 4M14 5l-4 14"/></svg>
  return <svg {...common}><circle cx="12" cy="12" r="8"/><path d="M12 8v4l3 2"/></svg>
}

function Logo() {
  return <a className="brand" href="#top" aria-label="Artificial Culture — início"><span className="brand-mark"><Icon name="brain" size={20}/></span><span>artificial<span className="brand-light">culture</span><small>GALA OF THE HIDDEN</small></span></a>
}

function App() {
  const [language, setLanguage] = useState<Language>(() => localStorage.getItem('ac-language') === 'en' ? 'en' : 'pt')
  const [step, setStep] = useState(0)
  const [distance, setDistance] = useState(100)
  const [openScenario, setOpenScenario] = useState<number | null>(0)
  const [copied, setCopied] = useState<number | null>(null)
  const t = text[language]

  useEffect(() => {
    document.documentElement.lang = language === 'pt' ? 'pt-BR' : 'en'
    localStorage.setItem('ac-language', language)
  }, [language])

  const rumorChance = useMemo(() => {
    const base = 0.5
    const senderOpenness = 0.5 + 0.6 * 0.5
    const receiverOpenness = 0.5 + 0.4 * 0.5
    return base * (1 / (1 + distance * 0.01)) * senderOpenness * receiverOpenness
  }, [distance])

  const copyCommand = async (index: number, command: string) => {
    try {
      await navigator.clipboard.writeText(command)
      setCopied(index)
      window.setTimeout(() => setCopied(null), 1800)
    } catch {
      setCopied(null)
    }
  }

  const goTo = (anchor: string) => document.getElementById(anchor)?.scrollIntoView({ behavior: 'smooth' })

  return (
    <>
      <a className="skip-link" href="#main">{language === 'pt' ? 'Pular para o conteúdo' : 'Skip to content'}</a>
      <header className="topbar" id="top">
        <div className="topbar-inner wrap">
          <Logo />
          <nav className="desktop-nav" aria-label={language === 'pt' ? 'Navegação principal' : 'Main navigation'}>
            {t.nav.map((item, index) => <a href={`#${['ideia', 'arquitetura', 'pratica', 'cenarios', 'pesquisa'][index]}`} key={item}>{item}</a>)}
          </nav>
          <div className="topbar-actions">
            <a className="github-link" href="https://github.com/maxjonata/Artificial-Culture" target="_blank" rel="noreferrer">GitHub <Icon name="external" size={14}/></a>
            <button className="language-button" type="button" onClick={() => setLanguage(language === 'pt' ? 'en' : 'pt')} aria-label={language === 'pt' ? 'Switch to English' : 'Mudar para português'}>{language === 'pt' ? 'EN' : 'PT'} <span>↗</span></button>
          </div>
        </div>
      </header>

      <main id="main">
        <section className="hero wrap" aria-labelledby="hero-title">
          <div className="hero-art" aria-hidden="true"><div className="hero-shade"/><div className="hero-coordinate">GOTH / AC · COGNITION STUDY № 01</div><div className="hero-beacon"><span/><span/><span/></div><div className="hero-bottom-rule"/></div>
          <div className="hero-copy">
            <p className="eyebrow"><i className="status-dot"/> {t.heroTag}</p>
            <h1 id="hero-title">{t.heroTitleA}<br/><em>{t.heroTitleB}</em></h1>
            <p className="hero-body">{t.heroBody}</p>
            <div className="hero-actions"><a className="button button-accent" href="#ideia">{t.explore} <Icon name="arrow"/></a><a className="button button-quiet" href="#legado"><Icon name="play"/>{t.watch}</a></div>
            <p className="hero-footnote"><span className="version-pill">v0.2.0</span><span>{t.release}</span><span className="footnote-line"/><span>{t.heroNote}</span></p>
          </div>
          <div className="hero-index" aria-hidden="true"><span>AC—01</span><span>01 / 08</span></div>
        </section>

        <section className="manifesto section-pad" id="ideia">
          <div className="wrap manifesto-grid">
            <div><p className="eyebrow muted"><span className="section-no">A</span> {t.manifestoLabel}</p><h2>{t.manifestoTitle}</h2></div>
            <div className="manifesto-text"><p>{t.manifestoBody}</p><p className="manifesto-aside"><span className="aside-mark">“</span>{t.manifestoAside}</p></div>
          </div>
        </section>

        <section className="boundary section-pad" id="arquitetura">
          <div className="wrap">
            <div className="section-heading"><div><p className="eyebrow"><span className="section-no">{t.boundaryLabel.split(' / ')[0]}</span> {t.boundaryLabel.split(' / ')[1]}</p><h2>{t.boundaryTitle}</h2></div><p className="section-deck">{t.boundaryIntro}</p></div>
            <div className="boundary-layout">
              <article className="system-card system-world"><div className="system-card-top"><span className="system-icon"><Icon name="eye" size={22}/></span><span className="mono micro">01—HOST</span></div><h3>{t.worldTitle}</h3><ul>{t.worldItems.map((item) => <li key={item}><span className="bullet bullet-sun"/>{item}</li>)}</ul><div className="system-ribbon">AUTORIDADE SOBRE O MUNDO</div></article>
              <div className="boundary-bridge"><div className="bridge-line"><span className="bridge-tick"/><span className="bridge-tick"/><span className="bridge-tick"/></div><div className="bridge-labels"><span>OBSERVAÇÃO</span><span>RESULTADO</span></div><div className="bridge-copy">{t.flowCaption}</div></div>
              <article className="system-card system-cognition"><div className="system-card-top"><span className="system-icon"><Icon name="brain" size={22}/></span><span className="mono micro">02—MODULE</span></div><h3>{t.acTitle}</h3><ul>{t.acItems.map((item) => <li key={item}><span className="bullet bullet-pink"/>{item}</li>)}</ul><div className="system-ribbon">ESTADO COGNITIVO INTERNO</div></article>
            </div>
            <div className="boundary-foot"><span className="mono micro">CONTRATO DE INTEGRAÇÃO / GOTH ↔ AC</span><span>Flecs world → PerceptionSnapshot → ActionIntent → ActionResult</span></div>
            <div className="pipeline"><div className="section-heading pipeline-heading"><div><p className="eyebrow"><span className="section-no">02</span> {t.pipelineLabel.split(' / ')[1]}</p><h2>{t.pipelineTitle}</h2></div><p className="section-deck">{t.pipelineIntro}</p></div>
              <div className="pipeline-steps">{t.steps.map(([number, title, body], index) => <article className="pipeline-step" key={number}><div className={`pipeline-node node-${index+1}`}>{index === 0 ? <Icon name="eye" size={20}/> : index === 1 ? <Icon name="book" size={20}/> : index === 2 ? <Icon name="code" size={20}/> : <Icon name="check" size={20}/>}</div><span className="mono micro step-index">{number} / {stepsIds[index]}</span><h3>{title}</h3><p>{body}</p>{index < 3 && <span className="step-arrow"><Icon name="arrow" size={14}/></span>}</article>)}</div>
            </div>
          </div>
        </section>

        <section className="demo-section section-pad" id="pratica">
          <div className="wrap demo-wrap">
            <div className="section-heading"><div><p className="eyebrow"><span className="section-no">03</span> {t.demoLabel.split(' / ')[1]}</p><h2>{t.demoTitle}</h2></div><p className="section-deck">{t.demoIntro}</p></div>
            <div className="demo-console">
              <div className="console-head"><div className="console-brand"><span className="console-pulse"/> AC / LOCAL ADAPTER <span className="console-dim">·</span> <span className="console-dim">DEMONSTRAÇÃO CONCEITUAL</span></div><span className="mono micro console-id">SEED 001 · FLECS</span></div>
              <div className="demo-progress" aria-label={`${t.demoSteps[step]}`}><span style={{ width: `${((step + 1) / 4) * 100}%` }}/></div>
              <div className="console-scene">
                <div className="scene-graph" aria-hidden="true"><svg viewBox="0 0 400 250" role="img"><path className={`graph-path ${step >= 0 ? 'is-lit' : ''}`} d="M76 128 C150 128 190 72 264 72"/><path className={`graph-path ${step >= 2 ? 'is-lit' : ''}`} d="M76 128 C150 128 194 186 264 186"/><path className={`graph-path graph-path-dash ${step >= 3 ? 'is-lit' : ''}`} d="M264 72 C324 72 336 126 264 186"/><circle className={`graph-orb agent ${step >= 0 ? 'active' : ''}`} cx="76" cy="128" r="24"/><circle className={`graph-orb resource ${step >= 0 ? 'active' : ''}`} cx="264" cy="72" r="19"/><circle className={`graph-orb host ${step >= 2 ? 'active' : ''}`} cx="264" cy="186" r="19"/><text x="76" y="134" textAnchor="middle">A</text><text x="264" y="78" textAnchor="middle">FOOD</text><text x="264" y="192" textAnchor="middle">HOST</text><text className="graph-label" x="76" y="180" textAnchor="middle">NPC 01</text><text className="graph-label" x="264" y="116" textAnchor="middle">RESOURCE 101</text><text className="graph-label" x="264" y="225" textAnchor="middle">GALA OF THE HIDDEN</text></svg></div>
                <div className="console-readout"><div className="readout-top"><span className="mono micro">{String(step + 1).padStart(2, '0')} / 04</span><span className="mono micro readout-mark">{t.scenarioMark}</span></div><h3>{t.demoSteps[step]}</h3><p>{step === 0 ? t.observeLine2 : step === 1 ? `${t.needTitle}: ${t.needValue}` : step === 2 ? t.intentAction : t.resultAction}</p>
                  <div className="readout-window"><div className="readout-window-title"><span/><span/><span/>{step < 3 ? t.observeTitle : t.resultTitle}</div><div className="readout-window-body">{step === 0 && <><span className="code-muted">host.</span><span> observe</span><span className="code-muted">(npc_01, snapshot)</span></>}{step === 1 && <><span className="code-muted">hunger</span><span> = </span><span className="code-pink">0.25</span><span className="code-muted">;</span><br/><span className="code-muted">target.food</span><span> = </span><span className="code-pink">101</span><span className="code-muted">;</span></>}{step === 2 && <><span className="code-muted">ActionIntent</span><span>{'{'}</span><br/><span className="code-pink">&nbsp; find_food, target 101</span><br/><span>{'}'}</span><span className="code-muted"> // proposal</span></>}{step === 3 && <><span className="code-muted">host.</span><span> report_result</span><span className="code-muted">(accepted)</span><br/><span className="code-pink">hunger → 0.65</span></>}</div></div>
                  <button className="button button-accent demo-next" type="button" onClick={() => setStep(step === 3 ? 0 : step + 1)}><Icon name={step === 3 ? 'play' : 'arrow'}/>{step === 3 ? t.demoRestart : t.demoButton}</button>
                </div>
              </div>
              <div className="console-foot"><span className="mono micro">{t.demoNote}</span><span className="console-foot-right"><span className="console-pulse"/> {t.demoSteps[step]}</span></div>
            </div>
          </div>
        </section>

        <section className="rumor-section section-pad">
          <div className="wrap rumor-grid"><div><p className="eyebrow"><span className="section-no">04</span> {t.rumorLabel.split(' / ')[1]}</p><h2>{t.rumorTitle}</h2><p className="section-deck rumor-deck">{t.rumorIntro}</p><div className="rumor-caveat"><span className="caveat-mark">!</span><span>{language === 'pt' ? 'O código mantém o texto do boato e a força da crença; não registra quem foi a fonte nem a relação entre duas pessoas.' : 'The code keeps rumor text and belief strength; it does not record the source or the relationship between two people.'}</span></div></div>
            <div className="rumor-console"><div className="rumor-console-top"><div><span className="mono micro">MODEL / TRANSMISSION</span><h3>{t.rumorCalc}</h3></div><Icon name="network" size={24}/></div><div className="rumor-network" aria-label="Illustrative rumor transmission between agents"><svg viewBox="0 0 420 135" preserveAspectRatio="xMidYMid meet"><path d="M75 70H194m32 0h119" className="network-link"/><path d="M75 70 176 19m50 0 119 51" className="network-link network-link-dashed"/><circle cx="58" cy="70" r="22" className="network-person person-a"/><circle cx="210" cy="70" r="22" className="network-person person-b"/><circle cx="362" cy="70" r="22" className="network-person person-c"/><circle cx="194" cy="19" r="5" className="network-contact"/><circle cx="226" cy="19" r="5" className="network-contact"/><text x="58" y="75" textAnchor="middle">A</text><text x="210" y="75" textAnchor="middle">B</text><text x="362" y="75" textAnchor="middle">C</text><text className="network-caption" x="58" y="117" textAnchor="middle">WITNESS</text><text className="network-caption" x="210" y="117" textAnchor="middle">CONTACT</text><text className="network-caption" x="362" y="117" textAnchor="middle">LISTENER</text></svg></div><p className="rumor-caption">{t.rumorCaption}</p><div className="range-row"><label htmlFor="distance-range">{t.rumorDistance}</label><output htmlFor="distance-range">{distance}</output></div><input id="distance-range" className="range-slider" type="range" min="0" max="200" step="5" value={distance} onChange={(event) => setDistance(Number(event.target.value))}/><div className="range-ends"><span>0</span><span>200</span></div><div className="chance-result"><div><strong>{Math.round(rumorChance * 100)}<small>%</small></strong><span>{t.rumorChance}</span></div><span className="chance-spark" style={{ '--chance': `${rumorChance * 100}%` } as CSSProperties}/></div><div className="formula">{t.rumorFormula}</div><p className="rumor-foot">{t.rumorDecay}</p></div>
          </div>
        </section>

        <section className="status-section section-pad">
          <div className="wrap"><div className="section-heading"><div><p className="eyebrow"><span className="section-no">05</span> {t.statusLabel.split(' / ')[1]}</p><h2>{t.statusTitle}</h2></div><p className="section-deck">{t.statusIntro}</p></div>
            <div className="status-columns">{t.statusColumns.map((column, index) => <article key={column.label} className={`status-card status-card-${index+1}`}><div className="status-card-top"><span className="mono micro">{column.label}</span><span className={`status-marker status-marker-${index+1}`}/></div><h3>{column.title}</h3><ul>{column.items.map((item) => <li key={item}><span className="status-check">{index === 0 ? <Icon name="check" size={14}/> : index === 1 ? '↗' : '·'}</span>{item}</li>)}</ul></article>)}</div>
            <div className="status-disclaimer"><span className="mono micro">LEITURA HONESTA</span><span>{language === 'pt' ? '“Em desenvolvimento” descreve o estado técnico atual. Não significa mundo simulado validado, comportamento realista demonstrado ou integração pronta.' : '“In development” describes the current technical state. It does not mean a validated simulated world, demonstrated realism, or a ready integration.'}</span></div>
          </div>
        </section>

        <section className="scenarios-section section-pad" id="cenarios">
          <div className="wrap"><div className="section-heading"><div><p className="eyebrow"><span className="section-no">06</span> {t.scenariosLabel.split(' / ')[1]}</p><h2>{t.scenariosTitle}</h2></div><p className="section-deck">{t.scenariosIntro}</p></div>
            <div className="scenario-list">{t.scenarios.map(([title, body], index) => <article className={`scenario-item ${openScenario === index ? 'scenario-open' : ''}`} key={scenariosIds[index]}><button type="button" className="scenario-trigger" onClick={() => setOpenScenario(openScenario === index ? null : index)} aria-expanded={openScenario === index}><span className="scenario-no mono micro">{String(index + 1).padStart(2, '0')}</span><span className="scenario-title">{title}</span><span className="scenario-control">{openScenario === index ? '−' : '+'}</span></button>{openScenario === index && <div className="scenario-panel"><div className="scenario-node"><Icon name="network"/></div><p>{body}</p><span className="mono micro scenario-state">{language === 'pt' ? 'CRITÉRIO DE ACEITAÇÃO · FUTURO' : 'ACCEPTANCE CRITERION · FUTURE'}</span></div>}</article>)}</div>
          </div>
        </section>

        <section className="research-section section-pad" id="pesquisa">
          <div className="wrap research-grid"><div><p className="eyebrow"><span className="section-no">07</span> {t.researchLabel.split(' / ')[1]}</p><h2>{t.researchTitle}</h2><p className="section-deck">{t.researchBody}</p></div><div className="research-rules">{t.researchRules.map((rule, index) => <article key={rule}><span className="rule-no mono micro">0{index+1}</span><p>{rule}</p></article>)}</div></div>
        </section>

        <section className="legacy-section section-pad" id="legado">
          <div className="wrap"><div className="legacy-heading"><div><p className="eyebrow"><span className="section-no">ARCHIVE</span> {t.legacyLabel}</p><h2>{t.legacyTitle}</h2></div><p>{t.legacyBody}</p></div><div className="video-frame"><iframe src="https://www.youtube-nocookie.com/embed/76Tgq8e8nEc?rel=0" title={language === 'pt' ? 'Demonstração antiga do Artificial Culture em Bevy' : 'Older Artificial Culture demo built with Bevy'} allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerPolicy="strict-origin-when-cross-origin" allowFullScreen loading="lazy"/><a className="video-open" href="https://www.youtube.com/watch?v=76Tgq8e8nEc" target="_blank" rel="noreferrer" aria-label={language === 'pt' ? 'Abrir vídeo no YouTube' : 'Open video on YouTube'}><Icon name="external"/></a></div><div className="legacy-note"><Icon name="clock" size={16}/><p>{t.legacyNote}</p><a href="https://www.youtube.com/watch?v=76Tgq8e8nEc" target="_blank" rel="noreferrer">{language === 'pt' ? 'Abrir no YouTube' : 'Open on YouTube'} <Icon name="arrow" size={14}/></a></div></div>
        </section>

        <section className="build-section section-pad">
          <div className="wrap build-grid"><div><p className="eyebrow"><span className="section-no">08</span> {t.buildLabel.split(' / ')[1]}</p><h2>{t.buildTitle}</h2><p className="section-deck">{t.buildIntro}</p><a href="https://github.com/maxjonata/Artificial-Culture" target="_blank" rel="noreferrer" className="button button-outline">{language === 'pt' ? 'Abrir repositório' : 'Open repository'} <Icon name="external" size={15}/></a></div><div className="terminal"><div className="terminal-top"><div className="terminal-dots"><i/><i/><i/></div><span className="mono micro">POWERSHELL / ARTIFICIAL-CULTURE</span><Icon name="copy" size={16}/></div>{[t.buildCommand1,t.buildCommand2,t.buildCommand3].map((command,index)=><button type="button" className="terminal-line" onClick={() => void copyCommand(index, command)} key={command}><span className="terminal-prompt">›</span><code>{command}</code><span className="terminal-copy"><Icon name={copied === index ? 'check' : 'copy'} size={15}/>{copied === index ? t.copied : t.copy}</span></button>)}</div></div>
        </section>

        <section className="sources-section section-pad">
          <div className="wrap"><div className="sources-heading"><div><p className="eyebrow"><span className="section-no">09</span> {t.sourcesLabel.split(' / ')[1]}</p><h2>{t.sourcesTitle}</h2></div><p className="section-deck">{t.sourcesIntro}</p></div><div className="source-grid">{t.sourceCards.map(([title, description], index) => <a className="source-card" key={title} href={[
            'https://github.com/maxjonata/Artificial-Culture/blob/master/Docs/ARCHITECTURE.md',
            'https://github.com/maxjonata/Artificial-Culture/blob/master/Docs/CULTURAL_SCENARIOS.md',
            'https://github.com/maxjonata/Artificial-Culture/blob/master/Docs/Research/README.md',
            'https://github.com/maxjonata/Artificial-Culture/blob/master/include/artificial_culture/cognition.hpp',
            'https://github.com/maxjonata/Artificial-Culture/blob/master/src/main.cpp',
            'https://github.com/maxjonata/Artificial-Culture/blob/master/LICENSE',
          ][index]} target="_blank" rel="noreferrer"><span className="source-icon"><Icon name={index === 3 || index === 4 ? 'code' : index === 2 ? 'book' : index === 1 ? 'network' : 'arrow'}/></span><span className="source-number mono micro">DOC / 0{index + 1}</span><strong>{title}</strong><span className="source-description">{description}</span><span className="source-open">{language === 'pt' ? 'Ver no GitHub' : 'View on GitHub'} <Icon name="external" size={13}/></span></a>)}</div></div>
        </section>
      </main>

      <footer className="footer">
        <div className="wrap footer-top"><Logo/><span>{t.footerText}</span><a href="https://github.com/maxjonata/Artificial-Culture" target="_blank" rel="noreferrer">{language === 'pt' ? 'Projeto no GitHub' : 'Project on GitHub'} <Icon name="external" size={14}/></a></div>
        <div className="wrap footer-bottom"><span>© ARTIFICIAL CULTURE · GALA OF THE HIDDEN</span><span>{t.footerLicense} <a href="https://creativecommons.org/licenses/by-nc-sa/4.0/" target="_blank" rel="noreferrer">CC BY-NC-SA 4.0 <Icon name="external" size={12}/></a></span></div>
      </footer>

      <nav className="mobile-dock" aria-label={language === 'pt' ? 'Navegação rápida' : 'Quick navigation'}>{['ideia', 'arquitetura', 'pratica', 'cenarios'].map((anchor,index) => <button type="button" key={anchor} onClick={() => goTo(anchor)}><span>{['◌','⌘','↗','⊞'][index]}</span>{t.nav[index]}</button>)}</nav>
    </>
  )
}

export default App
