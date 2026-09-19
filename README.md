# PrideVault SC

Smart contract MultiversX (Rust / SpaceCraft) pour **PrideVault** — vault de staking des NFTs *Heart of ROAR / ROARHighSpeX*.

Repo: https://github.com/luxaway/pridevault-sc

## Ce que fait le contrat

- Définir la collection NFT autorisée (`collection_id`)
- Staker un NFT (transfert ESDT/SFT/NFT vers le contrat)
- Unstaker un NFT (retour au owner)
- Lister les positions d’un wallet
- Pause / unpause (owner)
- Upgrade

Base à étendre (rewards ESDT, lock period, boost rarity, farm multi-collection).
**Pas audité. Ne pas déployer en mainnet sans audit.**

## Prérequis

```bash
rustup default stable
rustup target add wasm32-unknown-unknown
cargo install multiversx-sc-meta --locked
```

Rust ≥ 1.85. Framework : `multiversx-sc` **0.66.2**.

## Build

```bash
sc-meta all build
```

Sortie : `output/pridevault.wasm` et `output/pridevault.mxsc.json`.

## Tests

```bash
cargo test
```

## Déploiement (devnet)

1. Génère un interactor : `sc-meta all snippets`
2. Déploie depuis l’interactor ou mxpy / xPortal.
3. Appelle `init` avec le ticker de collection (ex. `ROAR-xxxxxx`).

## Structure

```
pridevault-sc/
  src/lib.rs          # contrat
  meta/               # générateur ABI / wasm
  wasm/               # crate wasm
  tests/              # tests SpaceCraft
```

## Licence

GPL-3.0-only (aligné avec le framework MultiversX).
