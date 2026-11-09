mart Rust Trading — Nolan & Sub-Agentes (v0.5.2)
​O Smart Rust Trading é um ecossistema de trading autónomo de alta performance desenvolvido em Rust, projetado para execução nativa em ambientes restritos (como o Termux em dispositivos móveis) e infraestruturas em nuvem de alta disponibilidade. O projeto combina resiliência de engenharia, criptografia de padrão industrial e uma arquitetura estritamente modular baseada em um Cargo Workspace.
​🏛️ Arquitetura do Workspace
​O ecossistema está dividido em crates especializados que separam as responsabilidades de segurança, execução, conexão com exchanges e gestão de agentes:
​sec_crypto: Módulo de primitivas criptográficas. Implementa a derivação segura de chaves através de Argon2id (otimizada para hardware móvel) e cifragem simétrica com AES-256-GCM.
​sec_storage: Camada de persistência atômica e segura. Garante o princípio de Zero Plaintext em disco, utilizando escrita baseada em ficheiros temporários, sincronização de E/S, vinculação de metadados via AAD, limites rígidos de tamanho e permissões restritas de arquivo.
​sec_binance: Conector assíncrono para a API pública da Binance, integrado com mecanismos de retentativas e oráculos de simulação de ticks para testes de stress.
​sec_solana: Módulo de integração com a blockchain Solana, gerindo conexões RPC, saldos e carteiras digitais.
​sec_domain & sec_execution: Camadas de domínio de negócio, validações rigorosas de mercado e gestão segura de ordens.
​sec_mcp_server: Servidor de protocolo de contexto para integração avançada com assistentes de inteligência artificial.
​agent: Sub-sistema autónomo de background responsável por sub-rotinas de monitorização contínua.
​sec_cli: Painel de controlo interativo em linha de comando que opera o motor contábil, executa benchmarks, comanda o Autopiloto e realiza auditorias de segurança.
​💼 Modelo Contábil e Regras em Cascata
​O ecossistema rege-se por uma política estrita de gestão de capital e alocação dinâmica:
​1. Capital Base (Agente Pai / Nolan): O agente principal mantém um patamar operacional nominal fixo de $50.00 USD.
​2. Transbordo Automático de Lucros: Sempre que o saldo de Nolan ultrapassa o teto de $60.00 USD devido a operações bem-sucedidas, o excedente líquido é automaticamente transferido para a Carteira de Reserva, reajustando o principal para o patamar base.
​3. Spawn Dinâmico de Sub-Agentes: A cada incremento de $15.00 USD acumulados na Carteira de Reserva, o sistema ativa um novo sub-agente tático em paralelo (limitado a um teto máximo de 10 unidades), escalando a capacidade analítica de forma proporcional ao lucro gerado.
​🛡️ Blindagem Criptográfica e Segurança
​Para assegurar a confidencialidade e integridade dos dados em repouso (como o cofre contábil e o histórico de ordens cifradas):
​Derivação Argon2id: As senhas mestre são processadas com parâmetros seguros de memória e iteração, com limpeza imediata de buffers sensíveis na memória RAM (zeroize).
​AES-256-GCM com AAD: Os dados são autenticados e cifrados com nonces exclusivos, vinculando o contexto criptográfico diretamente ao caminho do ficheiro para mitigar ataques de substituição.
​Gravação Atômica: A persistência em disco previne corrupções causadas por interrupções abruptas de energia ou falhas de sistema (crash).
​🚀 Como Executar o Painel de Controlo
​Após compilar o projeto com o perfil de otimização de release, o painel interativo pode ser iniciado diretamente no terminal:
​1. Compilação otimizada para o workspace:
cargo build --release
​2. Execução do painel de controlo:
./target/release/sec_cli
​Através do painel, é possível inspecionar o cofre contábil em cascata, iniciar o Autopiloto ao vivo com o oráculo da Binance, auditar ordens cifradas, testar o SecurityGuard, executar benchmarks de derivação Argon2id e realizar simulações de stress e backtest em ambiente isolado.