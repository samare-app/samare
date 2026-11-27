+++
title = "Samare : se retrouver quand le réseau ne passe plus"
date = 2026-11-27
description = "Pourquoi on construit une messagerie qui passe de téléphone en téléphone, ce qu'on s'impose et à quoi elle devrait ressembler : la maquette en images."
+++

Dans [notre premier billet](@/blog/se-parler-sans-reseau.md) on a fait le tour des outils qui permettent déjà de se parler quand le réseau sature. Celui-ci parle de l'appli qu'on construit : Samare. Elle n'existe pas encore. Les écrans montrés plus bas viennent d'une maquette.

## Pourquoi une appli de plus

Notre point de départ est simple. Un groupe d'amis part en festival. Le soir le réseau sature et le groupe se disperse. Il faut se retrouver, se prévenir et se rassurer.

Les applis qui existent répondent à une partie de ce besoin. À notre avis il manque encore une appli aussi simple qu'une messagerie ordinaire. Elle doit marcher dans un groupe où il y a des iPhone et des Android. Elle ne doit demander ni compte ni numéro de téléphone. On doit pouvoir s'en servir d'une main dans la foule, en plein soleil comme en pleine nuit.

## Le principe

Chaque téléphone qui a Samare fait passer les messages des autres en Bluetooth, de proche en proche. Plus il y a de monde équipé autour, plus ça porte loin. Aucune antenne n'est nécessaire : l'appli marche en mode avion avec le Bluetooth allumé.

Il faut donc du monde. Seul avec un ami au milieu d'une foule qui n'a pas l'appli, on reste limité à la portée du Bluetooth. C'est pour ça qu'on pense d'abord aux festivals : beaucoup de gens au même endroit et au même moment.

## Ce qu'on s'impose

Ces règles sont écrites dans [l'architecture du projet](https://github.com/samare-app/samare/blob/main/docs/ARCHITECTURE.md). Chacun pourra les vérifier dans le code.

- **Pas d'accès à internet.** L'appli Android ne demandera pas la permission internet.
- **Ni compte ni numéro ni serveur.** Votre identité tient dans des clés créées sur votre téléphone. Aucune entreprise ne garde vos messages ni la liste de vos contacts.
- **Les téléphones qui relaient ne liront rien.** Ils ne verront ni le contenu ni l'expéditeur ni le destinataire.
- **Pas de chiffrement fait maison.** On utilisera des bibliothèques éprouvées et auditées. La principale est celle de la messagerie Matrix.
- **Du code libre** sous licence GPLv3, [publié sur GitHub](https://github.com/samare-app/samare).

Ce que Samare ne protégera pas compte autant. Elle n'est ni anonyme ni intraçable. Un téléphone qui émet en Bluetooth se repère à quelques dizaines de mètres. Un téléphone déverrouillé montre tout ce qu'il contient. Le détail est dans notre [modèle de menace](/modele-de-menace/).

## La maquette

Avant d'écrire une ligne de l'appli on a dessiné le parcours d'un festivalier en six écrans. C'est une maquette : des dessins cliquables et pas une appli. Les noms, les messages et les chiffres sont inventés.

La pastille verte en haut de chaque écran compte les personnes autour de vous dont le téléphone relaie vos messages. Le petit dessin qu'elle contient est notre logo.

**Rejoindre un groupe** se fait en scannant un QR code. Il peut être sur le téléphone d'un ami, sur une affiche ou sur un billet. Un groupe peut être éphémère : il disparaît tout seul à l'heure choisie. La liste des discussions ressemble à celle de n'importe quelle messagerie. Elle dit en plus qui est à portée.

<figure>
<img src="/images/maquette/rejoindre.webp" width="260" height="563" alt="Écran sombre de scan d'un QR code. En bas une carte d'invitation : groupe Team Camping, invitation de Léa, 4 amis déjà dedans, groupe éphémère qui disparaît lundi à 8 h, bouton Rejoindre le groupe.">
<img src="/images/maquette/discussions.webp" width="260" height="563" alt="Liste des discussions : le groupe Team Camping avec 3 messages non lus, le canal officiel du Festival Écho, Léa marquée à portée et Sami. En haut la pastille des relais indique 14.">
<figcaption>Maquette. Rejoindre un groupe d'un scan. La liste des discussions.</figcaption>
</figure>

Dans un groupe le gros bouton vert sert de **talkie** : on le maintient pour parler et on le lâche pour envoyer. Le bouton « Je vais bien » rassure tout le groupe d'un geste. Un **point de rendez-vous** se pose à la main, par exemple « bar à gauche de la scène B ». Aucune position GPS n'est envoyée. Un statut sous chaque message dit combien d'amis l'ont reçu et pour combien il est encore en route.

**Retrouver un ami** dans la foule marche comme le jeu du chaud et du froid. L'appli mesure la force du signal Bluetooth de son téléphone. Le cercle grossit à mesure qu'on se rapproche.

<figure>
<img src="/images/maquette/groupe.webp" width="260" height="563" alt="Discussion du groupe Team Camping : un message vocal de Sami, la question de Léa, un point de rendez-vous au bar à gauche de la scène B, une réponse reçue par 3 et en route pour 2. Enfin Sami va bien. En bas les boutons Je vais bien et Maintenir pour parler.">
<img src="/images/maquette/retrouver.webp" width="260" height="563" alt="Écran Retrouver Léa : Léa est à portée, Tu chauffes, environ 15 m. Trois cercles ocre concentriques autour de ses initiales.">
<figcaption>Maquette. Un groupe avec son talkie. La recherche d'une amie en chaud et froid.</figcaption>
</figure>

Les deux écrans suivants sont des idées à discuter avec des festivals. Un festival pourrait avoir son **canal officiel**. Ses annonces seraient signées : on saurait qu'elles viennent bien de l'organisation. L'organisation ne saurait pas qui lit le canal. Depuis ce canal on pourrait aussi **alerter l'équipe prévention**. L'alerte serait relayée de téléphone en téléphone. Elle ne contiendrait que ce que vous choisissez d'écrire.

<figure>
<img src="/images/maquette/canal-officiel.webp" width="260" height="563" alt="Canal officiel du Festival Écho, annonces signées par l'organisation : un changement d'horaire, un point d'eau gratuit, un portefeuille trouvé. En bas un bouton rouge Alerter l'équipe prévention.">
<img src="/images/maquette/alerte.webp" width="260" height="563" alt="Écran d'alerte : Qu'est-ce qui se passe ? Choix entre Malaise, Je suis perdu·e, Agression et Autre. Un champ facultatif pour dire où l'on est. Une note précise que la position GPS n'est jamais envoyée. Bouton rouge Envoyer l'alerte.">
<figcaption>Maquette. Le canal officiel d'un festival et l'alerte à l'équipe prévention.</figcaption>
</figure>

La maquette a aussi un mode sombre pensé pour la nuit et les concerts.

<figure>
<img src="/images/maquette/groupe-sombre.webp" width="260" height="563" alt="La discussion du groupe Team Camping en mode sombre : fond presque noir, texte clair, accents vert tendre et ocre.">
<figcaption>Maquette. Le même groupe en mode sombre.</figcaption>
</figure>

## Pourquoi « Samare »

Une samare est la graine ailée de l'érable. Le vent l'emporte en la faisant tourner et elle retombe un peu plus loin. Un message relayé fait à peu près le même voyage. Le mot se dit « sa-mar ». L'anglais a presque le même : *samara*. Notre logo montre deux samares attachées par leurs graines. C'est ainsi qu'elles poussent sur l'arbre.

## Où on en est

Il n'y a rien à installer aujourd'hui. On écrit d'abord le cœur de l'appli. La première brique est faite : chaque téléphone crée sa propre identité et peut la partager en QR code. Viennent ensuite le format des messages, le chiffrement et le relais d'un téléphone à l'autre. On le teste dans un simulateur avant de toucher à un vrai téléphone. Le simulateur doit répondre à la question que tout le monde pose : à partir de combien de personnes est-ce que ça marche ?

On ne donne pas de date. On racontera ici chaque étape avec ses chiffres, y compris ce qui ne marche pas.

## Suivre le projet

L'avancement se suit sur [Mastodon](https://piaille.fr/@samare) ou par le [flux du journal](/atom.xml). Le code est [sur GitHub](https://github.com/samare-app/samare). Vous organisez un festival et le sujet vous intéresse ? Écrivez-nous à [contact@samare.app](mailto:contact@samare.app).
