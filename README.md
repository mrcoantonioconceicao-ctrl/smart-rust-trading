# Smart Rust Trading

O Smart Rust Trading é um ecossistema autônomo, distribuído e de alta performance voltado para o trading algorítmico, desenvolvido integralmente em linguagem Rust. Projetado para operar com eficiência e segurança em ambientes restritos, como o Termux no Android, o projeto une inteligência de mercado orientada por agentes autônomos a uma arquitetura de cibersegurança estrita em repouso, operando sob o princípio de zero exposição de texto em claro (Zero Plaintext).

---

## Arquitetura e Organização do Sistema

O projeto é estruturado em um workspace modularizado em crates independentes, garantindo baixo acoplamento, alta coesão e testabilidade rigorosa:

- **sec_crypto:** O motor criptográfico central, responsável pela derivação segura de chaves através do algoritmo Argon2id (com parâmetros rigorosos e otimizados para dispositivos móveis) e encriptação autenticada de dados baseada em AES-256-GCM.
- **sec_storage:** O subsistema de persistência transacional responsável por cifrar, gravar e decifrar com segurança os cofres de carteiras e históricos no disco de forma isolada.
- **sec_binance:** O conector de oráculos que consome feeds de cotações spot em tempo real a partir da API pública da Binance, monitorando ativos de alta liquidez como SOLUSDT e BTCUSDT.
- **sec_cli:** O painel de controlo interativo em linha de comandos (CLI) e motor central que orquestra o loop de execução do Autopiloto e a auditoria do sistema.

---

## O Ecossistema de Agentes

O ecossistema is liderado por uma arquitetura hierárquica inteligente que divide as responsabilidades operacionais:

- **Agente Pai (Nolan):** Assume o papel de maestro contábil e operacional do sistema. Inicia com um capital base estrito de cinquenta dólares, consome os ticks do mercado em tempo real a cada ciclo de cinco segundos e gere ativamente a tomada de decisão.
- **Matriz de Decisão Ativa:** A cada ciclo de execução, o Agente Nolan analisa o comportamento e a variação do preço para classificar o momento do mercado em três estados fundamentais:
  - COMPRAR (BUY): Tendência de alta detetada, momento de alocação de posições.
  - VENDER (SELL): Correção ou exaustão detetada, momento de realização de lucros e proteção do caixa.
  - NEUTRO (NEUTRAL): Mercado lateralizado ou com ruído, mantendo a posição e preservando o capital.
- **Sub-Agentes Táticos:** Unidades operacionais geradas de forma dinâmica e automática para expandir a capacidade de execução do ecossistema em paralelo, operando sob um teto controlado de instâncias concorrentes.

---

## Regras de Negócio e Gestão de Capital

O fluxo financeiro do ecossistema opera sob uma arquitetura de cascata rigorosa:

1. **Piso Operacional Fixo:** O capital principal gerido pelo Agente Nolan é mantido estritamente atrelado à base inicial de cinquenta dólares.
2. **Transbordo Automático de Lucros:** Sempre que as operações geram ganhos que elevam a carteira principal acima de sessenta dólares, o excedente é automaticamente cortado e transferido para uma Carteira de Reserva isolada.
3. **Spawn Dinâmico por Acumulação:** A cada múltiplo de quinze dólares acumulados na Carteira de Reserva através dos transbordos de lucro, o ecossistema dispara autonomamente um novo sub-agente tático para atuar em paralelo, convertendo os ganhos em poder de fogo operacional.

---

## Padrão de Segurança e Cibersegurança

Nenhum dado sensível, saldo ou histórico contábil é armazenado de forma legível no armazenamento do dispositivo:

- **Criptografia em Repouso:** Todos os cofres contábeis e registos de ordens são blindados através de criptografia autenticada de 256 bits, exigindo a autenticação por senha mestre do operador para qualquer operação de leitura ou escrita.
- **Isolamento de Estado:** O mecanismo de persistência garante que falhas de rede ou interrupções abruptas não corrompam o estado financeiro, salvando automaticamente as transações de forma cifrada a cada ciclo.
