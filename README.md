# 🛡️ Smart Rust Trading

> **Sistema de Alta Performance e Segurança para Trading CeFi Desenvolvido em Rust Puro.**  
> *Arquitetura Modular Baseada em Domain-Driven Design (DDD), Service-Oriented Architecture (SOA) e Blindagem Criptográfica AES-256-GCM.*

---

## 📑 Índice
1. [Visão Geral do Ecossistema](#-visão-geral-do-ecossistema)
2. [Arquitetura do Workspace (Cargo Workspace)](#-arquitetura-do-workspace-cargo-workspace)
3. [Segurança e Blindagem de Dados (*Secure by Design*)](#-segurança-e-blindagem-de-dados-secure-by-design)
4. [Agente Scout Multi-Ativo (Motor de Varredura)](#-agente-scout-multi-ativo-motor-de-varredura)
5. [Painel de Controlo CLI](#-painel-de-controlo-cli)
6. [Instalação e Execução](#-como-executar)
7. [Licenciamento e Governança](#-governança)

---

## 🚀 Visão Geral do Ecossistema

O **Smart Rust Trading** foi concebido sob rigorosos padrões de engenharia de software para operar de forma totalmente autónoma, resiliente e eficiente. O ecossistema foca-se na integração CeFi com a **Binance** (via Agente Scout Multi-Ativo) em ambiente mobile otimizado (**Termux**), garantindo consumo mínimo de recursos, concorrência segura em tempo real e persistência contábil inviolável.

---

## 🏗️ Arquitetura do Workspace (`Cargo Workspace`)

O projeto está estruturado em múltiplos crates independentes para isolar responsabilidades de domínio, armazenamento, conectividade e interface:

```text
smart-rust-trading/
├── Cargo.toml               # Configuração raiz do Workspace
├── agent/                   # Lógica central dos Agentes de Inteligência (Nolan)
├── crates/
│   ├── sec_domain/          # Entidades de domínio, modelos de ordens e regras de negócio
│   ├── sec_storage/         # Módulo de cofres e persistência cifrada (AEAD)
│   ├── sec_binance/         # Conector CeFi e Agente Scout Multi-Ativo (REST API / Testnet)
│   └── sec_cli/             # Painel de Controlo Interativo via Terminal
└── sec_solana/              # Módulo Web3 / On-chain (estruturado para expansão futura)

Segurança e Blindagem de Dados (Secure by Design)
​A segurança dos ativos contábeis e do histórico operacional constitui o núcleo do sistema:
​Cifragem em Repouso: Os cofres contábeis (sec_wallet.enc, sec_orders.enc) são protegidos por criptografia simétrica autenticada AES-256-GCM.
​Derivação de Chaves: Utilização de PBKDF2 para robustez contra ataques de força bruta.
​Integridade Operacional: Nenhuma credencial ou saldo sensível é exposto em texto plano no sistema de ficheiros local.
​🤖 Agente Scout Multi-Ativo (Motor de Varredura)
​O Agente Scout (Nolan) executa ciclos de varredura paralelos e contínuos sobre uma cesta diversificada de ativos na Binance, avaliando preços em tempo real, volatilidade e spreads sob a validação do SecurityGuard:

AtivoPar MonitoradoFunção no Portfólio
BitcoinBTCUSDTÂncora de Mercado e Tendência Macro
EthereumETHUSDTLiquidez e Derivativos
SolanaSOLUSDTAlta Frequência / Par de Rotação
Binance CoinBNBUSDTEficiência de Taxas e CeFi

📊 Painel de Controlo CLI
​O ecossistema dispõe de uma interface de linha de comandos robusta e interativa (sec_cli) que centraliza as operações de governança:
​Inspecionar Estado do Cofre & Saldo (Auditoria de portfólio).
​Executar Ciclo de Análise do Agente Nolan (Varredura multi-ativo em paralelo).
​Auditar Histórico de Ordens Cifradas (Validação AEAD).
​Testar Validação de Segurança (SecurityGuard).
​Consultar Preço Real na Binance (Conectividade REST API / Testnet).
​Efetuar Reset Contábil Seguro do Cofre.
​⚙️ Como Executar
​Para compilar e iniciar o painel de controlo no seu ambiente de desenvolvimento:

# 1. Clonar o repositório
git clone [https://github.com/mrcoantonioconceicao-ctrl/smart-rust-trading.git](https://github.com/mrcoantonioconceicao-ctrl/smart-rust-trading.git)
cd smart-rust-trading

# 2. Compilar o painel CLI e o ecossistema em Rust
cargo build --bin sec_cli

# 3. Executar o Painel de Controlo
cargo run --bin sec_cli


🏛️ Governança
​Desenvolvido sob rigor de engenharia de alta segurança, arquitetura limpa e rastreabilidade contábil em Rust.
©️ Marco Antonio Conceicao — Todos os direitos reservados.
