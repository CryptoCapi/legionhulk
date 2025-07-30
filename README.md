<p align="center">
  <img src="https://raw.githubusercontent.com/tuusuario/contrato_meme/main/banner.png" alt="Contrato Meme Banner" width="100%">
</p>

# 🪙 Contrato Meme Token ($CINU)

[![Solana](https://img.shields.io/badge/Solana-Blockchain-purple?logo=solana)](https://solana.com/)
[![Anchor](https://img.shields.io/badge/Framework-Anchor-blue)](https://book.anchor-lang.com/)
[![Rust](https://img.shields.io/badge/Language-Rust-orange?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-green)](LICENSE)

---

## ✨ Descripción
Este proyecto implementa un **token meme** llamado **Codeinu ($CINU)** en la blockchain de **Solana** utilizando el framework **Anchor**.  
El contrato incluye funcionalidad extendida como:
- **Mint:** Acuñar nuevos tokens.
- **Transfer:** Transferir tokens entre cuentas.
- **Burn:** Quemar tokens para reducir el suministro.
- **Change Authority:** Cambiar la autoridad del token.
- **Stake/Unstake:** Permite hacer staking y obtener recompensas.

---

## 🚀 Funcionalidades principales
1. **Initialize:** Inicializa el contrato con:
   - Nombre del token
   - Símbolo ($CINU)
   - Decimales (9)
   - Suministro inicial (100,000,000,000 * 10^9)
   - Autoridad inicial del contrato

2. **Mint:** Permite a la autoridad acuñar nuevos tokens.

3. **Transfer:** Transferencias de tokens entre cuentas.

4. **Burn:** Permite quemar tokens y reducir el suministro.

5. **Change Authority:** Cambia la autoridad del contrato.

6. **Stake / Unstake:**
   - Stake: Bloqueo de tokens con inicio de cálculo de recompensa.
   - Unstake: Desbloqueo con devolución de tokens y recompensa calculada (1% por día).

---

## 🛠 Tecnologías utilizadas
- **Blockchain:** Solana
- **Framework:** Anchor (Rust)
- **Lenguaje:** Rust
- **Eventos:** Anchor Events
- **Almacenamiento:** Solana Accounts

---

## 📂 Estructura del contrato
```bash
📦 contrato_meme
 ┣ 📜 Cargo.toml
 ┣ 📜 Anchor.toml
 ┣ 📜 src/lib.rs   # Código del contrato principal
 ┣ 📜 programs/    # Lógica de programa
 ┗ 📜 README.md
