# Architecture de Samare

Samare est une messagerie **sans internet et sans serveur** : les messages passent de téléphone en téléphone en Bluetooth, relayés par les autres utilisateurs. Elle est pensée pour les endroits où le réseau est saturé ou absent : festivals, montagne, métro, manifestations, coupures imposées.

Ce document décrit ce qui est choisi, et pourquoi.

## 1. Principes non négociables

1. **Aucune permission `INTERNET`** dans l'appli Android. N'importe qui peut le vérifier.
2. **Aucune localisation** : jamais demandée sur Android 12 et plus (`neverForLocation`). La seule exception est `ACCESS_FINE_LOCATION` limitée à `maxSdkVersion="30"`, car Android 8 à 11 l'imposent pour le scan Bluetooth.
3. **Aucune crypto maison** : uniquement des bibliothèques éprouvées et auditées.
4. **Les relais ne voient rien** : ni contenu, ni expéditeur, ni destinataire identifiable.
5. **Pas de numéro, pas d'e-mail, pas de compte** : l'identité est une paire de clés générée sur l'appareil.
6. **Transparence** : code ouvert, modèle de menace publié en langage simple, état du maillage visible dans l'interface.

## 2. Fonctionnalités

### Cœur
- Identité locale (pseudo et paire de clés), sans inscription.
- Ajout de contact par **QR code** :
  - scan caméra en personne : contact marqué **vérifié** ;
  - import d'un QR depuis une image (sélecteur de photos, ou partage depuis une autre appli) : contact non vérifié ;
  - vérification ultérieure par scan mutuel en personne.
- **Alerte de changement de clé** d'un contact.
- Discussions 1:1 chiffrées de bout en bout.
- **Groupes** rejoignables en un scan de QR, avec option **éphémère** (expiration automatique).

### Usages
- **Talkie** : messages vocaux courts en appui long, lecture automatique dans un groupe quand des écouteurs sont branchés.
- **Statut « je vais bien » / SOS** envoyé aux contacts en un geste.
- **Retrouver ses amis** : mode « chaud / froid » d'après la force du signal Bluetooth, qui marche sur tous les téléphones.
- **Maillage lisible** : nombre de relais à portée, état de chaque message (*envoyé → relayé → livré*), impact sur la batterie.

### Sécurité pour les usages sensibles
- Base de données locale chiffrée, déverrouillée par code.
- **Mode discret** : icône et nom neutres.
- **Code leurre** ouvrant un profil vide.
- **Effacement rapide**.

### Événements
- **Groupe officiel d'un événement** : annonces signées par la clé de l'organisateur, lecture seule pour les participants.
- **SOS vers l'équipe sécurité ou prévention** de l'événement, à l'initiative de l'utilisateur, avec le contenu qu'il choisit.
- **Point de rendez-vous** posé à la main dans un groupe (« scène B, bar de gauche »), sans position GPS.

## 3. Pile technique

```
┌───────────────────────────────────────────────┐
│  Interface : Flutter (Dart)                   │
│  une seule appli pour Android et iOS          │
├───────────────────────────────────────────────┤
│  Transport natif                              │
│  Android (Kotlin) : BLE (central +            │
│            périphérique), foreground service  │
│  iOS (Swift) : CoreBluetooth + arrière-plan   │
├───────────────────────────────────────────────┤
│  Cœur partagé en Rust                         │
│  UniFFI (Kotlin, Swift) · flutter_rust_bridge │
│  (Dart)                                       │
│  identité · contacts · crypto · paquets ·     │
│  routage · stockage · logique des groupes     │
└───────────────────────────────────────────────┘
```

- **Interface en Flutter** : une seule interface pour Android et iOS. Flutter dessine lui-même l'écran, le design est donc identique sur les deux plateformes. En contrepartie, l'appli pèse environ 5 à 10 Mo de plus.
- **Cœur en Rust** : les bibliothèques de crypto auditées retenues (`vodozemac`, `snow`) sont en Rust, et le cœur se teste sans téléphone, dans un simulateur de maillage.
- **Transport natif** (Kotlin, Swift) : aucun paquet Flutter ne gère les deux rôles Bluetooth (central et périphérique) ni le fonctionnement en arrière-plan.

**Le relais ne passe jamais par Dart.** La boucle de maillage vit dans le service natif et le cœur Rust, pour continuer de tourner quand l'interface est fermée. L'interface Flutter n'est qu'une vue sur le cœur. La même bibliothèque Rust est appelée depuis le service (UniFFI) et depuis Dart (`flutter_rust_bridge`), dans le même processus.

Le transport ne fait que pousser et recevoir des octets ; toute la logique vit dans le cœur. Les notifications, widgets d'écran d'accueil et Live Activities iOS, hors de portée de Flutter, sont faits en natif.

**Protocole maison.** Les protocoles de maillage Bluetooth existants exposent l'expéditeur, le destinataire ou les membres des groupes à chaque relais, ce qui contredit le principe 4. Samare utilise donc son propre protocole, décrit ci-dessous et spécifié dans `docs/PROTOCOL.md`.

## 4. Identité et contacts

- Clés : Ed25519 (signature) et Curve25519 (échange de clés).
- QR de contact : version, pseudo, clés publiques, empreinte, en encodage compact.
- Empreinte lisible (« numéro de sécurité ») affichable pour une comparaison à la main.

## 5. Chiffrement

- **Lien entre voisins** : Noise (`snow`), qui chiffre le saut Bluetooth lui-même.
- **Bout en bout 1:1** : double ratchet avec `vodozemac` (Olm), la bibliothèque Rust de Matrix, auditée et pensée pour l'asynchrone.
- **Groupes** : Megolm (`vodozemac`). MLS suppose un ordre de livraison qu'un réseau sans serveur ne garantit pas.

## 6. Format des paquets

- **Taille fixe**, ajustée au MTU Bluetooth, avec bourrage : tous les paquets se ressemblent. Les messages longs et les vocaux sont fragmentés.
- En-tête en clair minimal :
  - version ;
  - TTL (nombre de sauts restants) ;
  - identifiant aléatoire du paquet (déduplication) ;
  - **étiquette de destinataire éphémère**.
- **Aucun expéditeur en clair.**
- Étiquette éphémère = HMAC(secret partagé, fenêtre de temps), tronquée. Elle change à chaque fenêtre ; seuls les deux contacts, ou les membres du groupe, savent la reconnaître.
- Les accusés de livraison sont chiffrés de bout en bout et voyagent comme des messages ordinaires.
- **Vecteurs de test octet par octet** (`docs/vectors/`), pour que les implémentations Android et iOS restent compatibles.

## 7. Routage

- **Inondation contrôlée** : chaque nœud retransmet à ses voisins, avec TTL et déduplication (filtre de Bloom tournant).
- **Stockage et retransmission** : cache local des paquets en transit, avec expiration et taille maximale, pour que les gens qui se déplacent portent les messages.
- **Anti-spam** : limite de débit par voisin, taille maximale, priorité aux paquets récents.
- **Limiter les retransmissions inutiles**, des idées reprises de Knit et réglées dans le simulateur :
  - délai aléatoire avant de relayer ;
  - abandon du relais quand le paquet a déjà été entendu de plusieurs voisins ;
  - pas de renvoi aux voisins dont le paquet vient ;
  - quota par conversation dans le cache, pour qu'un seul groupe ne le remplisse pas ;
  - expiration comptée depuis l'envoi, pas depuis la réception.
- « Relayé » signifie qu'au moins un voisin a accusé réception du paquet au niveau du lien.

## 8. Stockage

- SQLite chiffré (SQLCipher), clé dérivée du code de l'utilisateur (Argon2id).
- Le profil leurre est une seconde base distincte.
- L'effacement rapide détruit la clé, puis les fichiers.

## 9. Android

**Version minimale : Android 8 (`minSdk 26`)**, pour ne pas exclure les téléphones anciens, fréquents dans les contextes visés. Sur Android 8 à 11, le scan Bluetooth exige la permission de localisation et le service de localisation activé.

- Permissions :
  - `BLUETOOTH_SCAN` avec `android:usesPermissionFlags="neverForLocation"` ;
  - `BLUETOOTH_ADVERTISE`, `BLUETOOTH_CONNECT` ;
  - `ACCESS_FINE_LOCATION` avec `android:maxSdkVersion="30"` ;
  - `FOREGROUND_SERVICE`, `FOREGROUND_SERVICE_CONNECTED_DEVICE`, `POST_NOTIFICATIONS` ;
  - `CAMERA` (scan des QR), `RECORD_AUDIO` (talkie).
- **`INTERNET` est retirée explicitement** du manifeste (`tools:node="remove"`), au cas où une bibliothèque l'ajouterait. La CI vérifie l'APK release. Flutter ajoute `INTERNET` aux builds debug et profile pour le rechargement à chaud, elle n'est jamais dans une build release. Chaque paquet Flutter ajouté est vérifié (pas de polices téléchargées, par exemple).
- Service au premier plan de type `connectedDevice` pour le maillage en arrière-plan.
- QR : ZXing, qui marche hors-ligne et sans les services Google.
- Import d'image : sélecteur de photos du système (aucune permission de stockage) et réception du partage depuis une autre appli.
- Voix : Opus à bas débit (environ 6 à 8 kbit/s), durée plafonnée. L'encodeur Opus d'Android n'existe qu'à partir d'Android 10 : `libopus` est embarquée, dans le cœur Rust, pour servir aussi à iOS.
- Pas de demande d'exemption d'optimisation de la batterie ; l'appli guide l'utilisateur sur les téléphones dont le constructeur coupe les applis en arrière-plan.

## 10. iOS

En arrière-plan, iOS ne scanne que l'UUID de service, au ralenti. L'annonce passe dans une zone qu'Android ne lit pas sans décodage spécifique. Pour compenser :

- garder les connexions établies, qu'iOS maintient en arrière-plan ;
- restauration d'état CoreBluetooth ;
- un **mode actif** pour les événements (appli ouverte, écran sombre) ;
- les téléphones Android servent de colonne vertébrale du réseau ;
- au premier plan, l'iPhone n'annonce que l'UUID de service, et les autres lisent le reste par une caractéristique GATT.

L'interface indique clairement « relais limité en arrière-plan ».
