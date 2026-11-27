+++
title = "Quand le réseau sature : les outils qui existent déjà pour se parler"
date = 2026-11-23
description = "Festival, montagne, métro : ce qu'on peut utiliser aujourd'hui quand le réseau mobile ne passe plus, ce que chaque outil fait bien et ce qu'il faut savoir."
+++

Dans un festival, des dizaines de milliers de téléphones se partagent les mêmes antennes. Les barres de réseau restent pleines mais plus rien ne passe : l'antenne est débordée. En montagne ou dans le métro, c'est plus simple : il n'y a pas d'antenne du tout.

Avant d'écrire notre propre appli, on a regardé ce qui existe déjà. Voici ce qu'on peut utiliser aujourd'hui, ce que chaque outil fait bien et ce qu'il faut savoir avant de compter dessus. Les faits ont été vérifiés le 23 novembre 2026 et ces applis évoluent vite.

## Avant toute appli : s'organiser

- **Fixer un point et une heure de rendez-vous** avant d'entrer. Quand plus rien ne passe, c'est ce qui marche le mieux.
- **Écrire plutôt qu'appeler.** Un opérateur le dit lui-même dans [son guide sur les réseaux en festival](https://www.bouyguestelecom.fr/guide-pratique/forfaits-mobile/reseau-festival-saturation-astuces) : « En cas de saturation, les SMS passent plus facilement que les appels vocaux ». Un SMS [finit « presque toujours » par passer](https://selectra.info/telecom/actualites/marche/reseau-mobile-sature-plage-festival-solutions) avec parfois quelques minutes de retard. Presque toujours ne veut pas dire toujours.

## Les talkies-walkies

On trouve les talkies « PMR446 » en grande surface. Ils s'utilisent sans licence en France d'après le [tableau national des fréquences](https://www.anfr.fr/fileadmin/TNRBF/2026/TNRBF_2026-03-16.pdf) et dans toute l'Europe d'après une [décision européenne](https://docdb.cept.org/document/448) : 16 canaux autour de 446 MHz et 500 mW au maximum. Ils ne dépendent d'aucune antenne. Ils marchent donc de la même façon quand le réseau sature.

À savoir :

- ce n'est pas chiffré : toute personne réglée sur le même canal entend tout ;
- il faut un appareil par personne et des piles ou une batterie chargée ;
- la portée dépend beaucoup du terrain.

## Les applis qui passent par le Bluetooth

Les téléphones se parlent directement en Bluetooth à courte distance. Certaines applis vont plus loin : chaque téléphone qui a l'appli relaie les messages des autres de proche en proche. Plus il y a de monde équipé autour, plus ça porte loin. Seul avec un ami au milieu d'une foule qui n'a pas l'appli, on reste limité à la portée du Bluetooth.

Point commun à toutes : les personnes à qui on veut écrire doivent avoir installé la même appli **avant**. Le jour J, le réseau ne permettra pas de la télécharger.

### bitchat

[bitchat](https://github.com/permissionlesstech/bitchat) marche sur iPhone à partir d'iOS 16 et sur Android à partir d'Android 8. L'appli est [gratuite sur l'App Store](https://apps.apple.com/us/app/bitchat-mesh/id6748219622) et ne demande ni compte ni numéro de téléphone.

Ce qui marche bien :

- un groupe qui mélange iPhone et Android peut se parler ;
- un message peut passer par jusqu'à 7 téléphones intermédiaires ;
- le code est dans le domaine public.

À savoir :

- [certaines fonctions passent par internet](https://github.com/permissionlesstech/bitchat-android) : les salons par zone géographique et les messages privés entre favoris quand le Bluetooth ne suffit pas ;
- sur Android, l'appli demande l'accès à la localisation même en arrière-plan ;
- la documentation précise que l'appli utilise un identifiant d'appareil stable ;
- à notre avis, l'interface s'adresse plutôt à des gens à l'aise avec l'informatique : on y tape des commandes.

### Knit

[Knit](https://getknit.app/) marche sur Android à partir d'Android 10 et sur iPhone à partir d'iOS 18. On le trouve dans le Play Store, sur F-Droid et [gratuitement sur l'App Store](https://apps.apple.com/app/id6820348510). C'est un logiciel libre qui ne demande ni compte ni numéro.

Ce qui marche bien :

- une interface claire et moderne ;
- un groupe qui mélange iPhone et Android peut se parler ;
- les messages privés et de groupe sont chiffrés de bout en bout ;
- on peut envoyer des photos, des vocaux et des fichiers jusqu'à 8 Go à un ami proche.

À savoir :

- le contenu des messages est chiffré. La [documentation technique du projet](https://github.com/getknit/knit/blob/main/docs/ARCHITECTURE.md) indique honnêtement que les téléphones qui relaient voient qui écrit à qui et combien (section 18). Pour retrouver ses amis en festival, c'est rarement un problème. Dans un contexte sensible, c'est à garder en tête ;
- la version iPhone est toute récente. [Sur iPhone, l'appli ne trouve de nouvelles personnes que quand elle est ouverte](https://github.com/getknit/knit-ios) : iOS met en pause les applis en arrière-plan.

### Briar

[Briar](https://briarproject.org/) marche sur Android et sur ordinateur. C'est un logiciel libre sans compte.

Ce qui marche bien :

- l'appli a passé un [audit de sécurité indépendant](https://briarproject.org/news/2017-beta-released-security-audit/) en 2017 ;
- [sans internet, les messages passent par Bluetooth ou Wi-Fi](https://briarproject.org/how-it-works/) et par le réseau Tor quand internet marche ;
- l'appli peut [se transmettre d'un téléphone Android à l'autre](https://briarproject.org/news/) sans réseau.

À savoir :

- pas de version iPhone ;
- les messages vont directement d'un contact à l'autre sans passer par les téléphones d'inconnus. Hors de portée du Bluetooth, il faut internet ;
- le projet est [en mode maintenance](https://briarproject.org/news/2026-maintenance-mode/) depuis juillet 2026 : seulement des correctifs. L'équipe y travaille sur son temps libre. Elle cite elle-même une forte consommation de batterie et un fonctionnement en arrière-plan peu fiable sur Android.

### Les autres en bref

- **[Proxima](https://apps.apple.com/us/app/-/id6757075654)** : iPhone et iPad seulement, par Bluetooth et Wi-Fi. Après 3 jours d'essai, l'abonnement coûte 4,99 $ par mois.
- **[Berty](https://github.com/berty/berty)** : iPhone et Android. Elle marche aussi en Bluetooth sans internet. Ses développeurs indiquent qu'elle est encore en développement et qu'elle ne doit pas encore servir à échanger des données sensibles.
- **Bridgefy** : longtemps la plus connue. Des chercheurs y ont trouvé des failles sérieuses [en 2020](https://royalholloway.ac.uk/about-us/news/royal-holloway-cryptographers-urge-caution-over-use-of-secure-messaging-app) puis [en 2022](https://www.usenix.org/conference/usenixsecurity22/presentation/albrecht). Un [audit de 2023](https://7asecurity.com/reports/pentest-report-bridgefy.pdf) a relevé 16 failles surtout côté serveurs. La plupart ont été corrigées pendant l'audit.

## Meshtastic : une petite radio en plus du téléphone

Avec [Meshtastic](https://meshtastic.org/), on associe un boîtier radio à son téléphone en Bluetooth. Les boîtiers se relaient entre eux sur « plusieurs kilomètres » selon le projet. Les messages sont chiffrés. Le projet indique qu'aucune licence n'est nécessaire dans la plupart des pays. Un [boîtier prêt à l'emploi](https://openelab.io/fr/collections/meshtastic-handheld) coûte autour de 40 à 50 €.

À savoir : chacun doit avoir son boîtier. C'est plutôt pour la montagne, la randonnée ou un groupe organisé à l'avance que pour une foule.

## Quoi choisir

- **Un groupe avec des iPhone et des Android** : bitchat ou Knit. À notre avis, Knit a l'interface la plus simple. Berty marche aussi mais elle se dit encore en développement.
- **Que des Android déjà en contact** : Briar si vous voulez une appli auditée. Elle n'évolue plus.
- **Montagne, longues distances** : des talkies PMR446 ou Meshtastic si vous êtes prêts à acheter un boîtier.
- **Ne rien installer** : un point de rendez-vous et des SMS.

Dans tous les cas : **installez avant de partir** avec tout le groupe et faites un essai à la maison en mode avion avec le Bluetooth allumé.

## Ce qu'on n'a pas mesuré

On n'a pas mesuré nous-mêmes la portée, le délai ni la consommation de batterie de ces applis. Les chiffres ci-dessus viennent de leurs auteurs et des études citées. Les mesures viendront avec nos propres essais et on les publiera ici.

## Pourquoi on en parle

On développe nous-mêmes une messagerie de ce type appelée Samare. On n'est donc pas neutres. On a commencé par regarder ce qui existe parce que ces projets font déjà beaucoup. Ils sont souvent portés par de toutes petites équipes. Ce que notre appli protégera et ce qu'elle ne protégera pas est décrit dans notre [modèle de menace](/modele-de-menace/). Pour suivre l'avancement, c'est sur [Mastodon](https://piaille.fr/@samare).
