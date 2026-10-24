# Samare

Messagerie maillée **hors-ligne et sans serveur** : les messages passent de téléphone en téléphone en Bluetooth, relayés par les autres utilisateurs. Pensée pour les endroits où le réseau est saturé ou absent (festivals, montagne, manifestations).

Projet en cours de développement, pas encore utilisable.

## Organisation du dépôt

| Dossier | Contenu |
| --- | --- |
| `core/` | Cœur en Rust (workspace Cargo) : `samare-core` (logique, testée) et `samare-ffi` (liaisons UniFFI et `flutter_rust_bridge`). |
| `app/` | Interface Flutter, commune à Android et iOS. Transport BLE natif dans `app/android/` (Kotlin) et `app/ios/` (Swift). |
| `docs/` | Architecture, modèle de menace, spécification du protocole. |
| `scripts/` | Outils de vérification utilisés par la CI. |

Choix techniques et leurs raisons : `docs/ARCHITECTURE.md`.

## Construire

Prérequis : Rust (`rustup`, la version est figée dans `core/rust-toolchain.toml`), Flutter stable, JDK 17 et le SDK Android.

```bash
cd core && cargo test --workspace
cd app && flutter test
cd app && flutter build apk --release
```

## Licence

GPLv3 ou ultérieure, avec une exception pour la distribution sur les magasins d'applications. Voir `LICENSE` et `LICENSE-EXCEPTION`.
