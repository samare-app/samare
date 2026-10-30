# Modèle de menace de Samare

> Mis à jour le 28/10/2026. Samare est en cours de développement : **rien de ce qui suit n'est encore codé**. Ce document décrit ce que l'appli doit garantir et sert de référence pour la construire. Il sera revu à chaque version, et avant l'audit de sécurité externe.

Ce document dit en langage simple ce que Samare protège, contre qui, et surtout **ce qu'elle ne protège pas**. Si votre sécurité est en jeu, lisez au moins la partie « Ce que Samare ne protège pas ».

## En bref

- Les personnes qui relaient vos messages ne peuvent ni les lire, ni savoir qui les envoie, ni à qui.
- Il n'y a ni serveur, ni compte, ni numéro de téléphone : aucune entreprise, pas même nous, ne détient de données sur vous.
- En mode avion avec le Bluetooth allumé, votre téléphone ne se connecte plus aux antennes : l'opérateur ne peut plus le localiser.
- Samare n'est **ni anonyme ni intraçable**. Un téléphone qui émet en Bluetooth se détecte à quelques dizaines de mètres, et un téléphone déverrouillé montre tout ce qu'il contient.

## Quelques mots utilisés ici

- **Relais** : un téléphone qui fait passer un message qui ne lui est pas destiné. Chaque utilisateur de Samare sert de relais aux autres.
- **Chiffré de bout en bout** : seuls l'expéditeur et les destinataires peuvent lire le message.
- **Clé** : ce qui sert à chiffrer et à signer. Votre identité Samare est une paire de clés créée sur votre téléphone.
- **Métadonnées** : tout ce qui n'est pas le contenu : qui parle à qui, quand, combien, d'où.

## Ce que Samare protège

1. **Le contenu de vos messages** : textes, vocaux, statut « je vais bien », SOS, points de rendez-vous. Tout est chiffré de bout en bout.
2. **Qui parle à qui.** Aucun message ne porte le nom ou la clé de son expéditeur de façon lisible pour les relais. Le destinataire est désigné par une étiquette qui change régulièrement et que seuls les intéressés reconnaissent. La liste des membres d'un groupe n'apparaît nulle part sur le réseau.
3. **Le lien avec votre vraie identité.** Pas de numéro, pas d'e-mail, pas de compte. Seul le pseudo que vous choisissez circule, et seulement vers vos contacts et vos groupes.
4. **L'authenticité.** Un message affiché comme venant d'un contact vient bien de lui. Une annonce officielle d'événement est signée par la clé de l'organisateur.
5. **Vos données sur le téléphone** : base chiffrée, déverrouillée par un code, effaçable en un geste.

## Qui peut essayer de vous nuire, et ce qu'il voit

### Les autres utilisateurs qui relaient vos messages

**Ce qu'ils voient** : des paquets tous de la même taille, avec un en-tête minimal : le nombre de sauts restants, un numéro tiré au hasard, et l'étiquette du destinataire.

**Ce qu'ils ne voient pas** : le contenu, l'expéditeur, le destinataire. Les signatures et les clés qui permettraient de reconnaître l'auteur d'un message sont elles aussi chiffrées. Le nombre de sauts de départ est tiré au hasard : un voisin ne peut pas en déduire qu'il reçoit le paquet directement de son auteur.

**Ce qui fuit quand même** :
- pendant un court moment, l'étiquette d'une conversation reste la même : on peut voir que plusieurs paquets vont au même endroit, sans savoir qui est derrière ;
- un long message ou un vocal est découpé en plusieurs paquets, dont le nombre trahit à peu près la longueur.

### Quelqu'un qui écoute la radio à proximité

Un simple scanner Bluetooth suffit, à quelques dizaines de mètres, plus avec une bonne antenne.

**Ce qu'il ne voit pas** : ce que s'échangent deux téléphones voisins, en-tête compris. Chaque saut est chiffré, en plus du chiffrement de bout en bout.

**Ce qu'il voit** :
- **qu'un téléphone utilise Samare.** L'annonce Bluetooth le dit forcément, sinon les autres téléphones ne le trouveraient pas ;
- la force du signal, donc une position approximative ;
- le rythme des émissions.

Pour qu'on ne puisse pas suivre un téléphone d'un endroit à l'autre, Samare n'émet aucune donnée fixe : tout ce que contient l'annonce change en même temps que l'adresse Bluetooth, et chaque connexion entre voisins utilise des clés jetables.

Avec beaucoup de capteurs répartis sur un lieu (un organisateur sur son site, des autorités), on peut estimer **d'où part un message** : le téléphone qui l'émet en premier est celui de son auteur. On ne peut toujours pas le lire.

### Un téléphone malveillant dans le réseau

Quelqu'un peut faire tourner une version modifiée de Samare. Il voit ce que voit un relais et peut en plus jeter des paquets, en fabriquer, en renvoyer d'anciens ou inonder le réseau.

- Un message falsifié est détecté et ignoré : tout est signé et chiffré.
- Un message renvoyé une seconde fois est reconnu et ignoré.
- Chaque voisin a une limite de débit, et chaque paquet une taille maximale.

Il peut quand même ralentir ou bloquer les messages autour de lui.

### L'organisateur d'un événement

- **Groupe officiel** : en lecture seule. Aucun accusé de livraison ni de lecture ne remonte à l'organisateur : il ne sait pas qui a rejoint le groupe, ni qui a lu ses annonces.
- **SOS vers l'équipe sécurité ou prévention** : l'équipe reçoit ce que vous choisissez d'envoyer, et rien d'autre. Aucune position GPS n'est envoyée.
- **Mais** l'organisateur maîtrise le lieu et peut y installer des capteurs radio (voir plus haut).

### Vos contacts et les membres de vos groupes

- Ils lisent ce que vous leur envoyez, et peuvent le copier, le capturer, le montrer.
- Un groupe éphémère s'efface des téléphones, pas des copies que quelqu'un aurait faites.
- Les membres d'un groupe voient les pseudos des autres membres.
- Un groupe dont le QR circule publiquement est ouvert à tout le monde, y compris à qui veut le surveiller.
- Un contact ajouté **depuis une image** n'est pas vérifié : quelqu'un a pu remplacer le QR par le sien. Un contact ajouté **en scannant son téléphone en personne** est vérifié. Si la clé d'un contact change, Samare vous prévient.

### Quelqu'un qui prend votre téléphone

- **Éteint ou verrouillé** : la base est chiffrée. Sa clé est elle-même protégée par la puce de sécurité du téléphone, dont elle ne sort jamais : copier les fichiers ne suffit pas pour essayer tous les codes sur un ordinateur, chaque essai doit passer par le téléphone. Un code long reste la meilleure protection.
- **Allumé, appli fermée mais relais actif** : pour reconnaître les messages qui vous sont destinés, Samare garde une partie de ses secrets en mémoire. Un téléphone saisi allumé est donc moins protégé qu'un téléphone éteint. Ce point est encore à l'étude (voir la fin du document).
- **Déverrouillé, appli ouverte** : tout est lisible.
- **Si on vous force à donner votre code** : le code leurre (prévu) ouvre un profil vide. Toutes les installations ont deux profils, qu'on se serve du leurre ou non : l'existence du second ne prouve rien. Mais Samare est publique : quelqu'un qui la connaît sait que la fonction existe.
- **Effacement rapide** : il détruit les clés, puis les fichiers. Sans les clés, ce qui resterait des fichiers est illisible.

En France, refuser de donner le code de son téléphone peut, dans certains cas, être puni par la loi (Cour de cassation, 2022). Renseignez-vous auprès d'une legal team ou d'un avocat.

**Ce que le téléphone pourrait laisser fuir, et ce que Samare fait contre** :
- la sauvegarde du système (dans le cloud ou vers un nouveau téléphone) est désactivée pour Samare ;
- les captures d'écran sont bloquées, et l'appli est masquée dans l'écran des applis récentes ;
- par défaut, les notifications n'affichent ni le contenu ni l'expéditeur ;
- Samare demande au clavier de ne pas apprendre ce que vous tapez. Certains claviers ignorent cette demande.

### Les magasins d'applications, et nous

- Google et Apple savent que votre compte a installé Samare. Pour l'éviter : F-Droid, le fichier APK signé, ou le partage de l'appli de téléphone à téléphone (Android).
- Nous ne recevons rien de vous : l'appli n'a pas la permission d'accéder à Internet, et n'importe qui peut le vérifier.
- Une mise à jour piégée reste possible en théorie, de notre part ou de celle d'un magasin. Contre cela : le code est ouvert, des builds reproductibles (prévues) permettront de vérifier que l'appli distribuée correspond au code publié, et nous cherchons à financer un audit externe.

### Votre opérateur et Internet

Samare n'utilise jamais Internet. En mode avion avec le Bluetooth allumé, le téléphone ne parle plus aux antennes. Si vous laissez le réseau mobile allumé, l'opérateur sait où est votre téléphone, comme d'habitude, mais il ne voit rien de Samare.

## Ce que Samare ne protège pas

- **Un téléphone déverrouillé** entre d'autres mains.
- **Un logiciel espion** déjà installé, ou un téléphone compromis.
- **La détection radio** : utiliser Samare se voit à proximité. Couper le maillage dans l'appli, ou couper le Bluetooth, rend le téléphone silencieux, mais il ne reçoit plus rien.
- **La localisation approximative** d'un message à son départ par un réseau de capteurs.
- **Un contact ou un membre de groupe** qui trahit.
- **Le brouillage radio**, ou un réseau volontairement saturé.
- **La livraison** : un message peut ne jamais arriver si personne ne fait le lien.
- **L'avenir** : le chiffrement actuel n'est pas conçu pour résister à un futur ordinateur quantique. Un paquet capté aujourd'hui pourrait un jour être déchiffré.
- **Nos erreurs** : les briques de chiffrement viennent de bibliothèques auditées (`vodozemac` de Matrix, `snow` pour Noise, RustCrypto), mais la façon dont Samare les assemble est nouvelle et n'a pas encore été auditée.

## Selon votre situation

### Festival, concert, ski, métro

Les protections ci-dessus couvrent ce qu'on attend d'une messagerie : ni les autres participants ni l'organisateur ne lisent vos messages ni ne savent avec qui vous parlez.

### Manifestation

Samare laisse moins de traces : pas d'opérateur, pas de serveur, pas de compte. Mais **votre présence est détectable** : un scanner Bluetooth peut repérer un téléphone qui utilise Samare. Conseils :
- passez en mode avion et rallumez seulement le Bluetooth ;
- verrouillez votre téléphone par un code long plutôt que par empreinte ou visage ;
- vérifiez vos contacts en personne avant l'action ;
- préférez les groupes éphémères, et ne publiez pas le QR d'un groupe ;
- sachez où est l'effacement rapide.

### Pays où avoir l'appli est déjà un risque, coupure de réseau imposée

- **Le mode discret** (icône et nom neutres) protège d'un coup d'œil, pas d'une fouille sérieuse (liste des applis dans les réglages, analyse du téléphone), ni d'un scanner Bluetooth tant que le maillage est actif.
- Installez Samare hors des magasins (F-Droid, APK signé, partage hors-ligne), qui peuvent retirer l'appli sous la pression d'un gouvernement.
- Coupez le maillage quand vous n'en avez pas besoin.
- Le code leurre est prévu, avec ses limites (voir plus haut).

Une partie dédiée à ces contextes sera ajoutée avant la sortie publique.

## Points encore à l'étude

- **Les secrets gardés en mémoire** pendant que Samare relaie en arrière-plan : lesquels, et combien de temps.
- **Le premier message d'un nouveau contact.** Quand quelqu'un vous écrit pour la première fois après avoir importé votre QR, vous ne le connaissez pas encore. Il faut donc une étiquette que vous reconnaissez à partir de votre seule clé publique. Quiconque a votre QR pourrait alors reconnaître ces premiers messages, sans leur contenu ni leur auteur : si votre QR circule publiquement, on peut voir qu'on vous écrit.
- **La durée pendant laquelle une étiquette reste la même**, entre discrétion et fiabilité de la livraison.
