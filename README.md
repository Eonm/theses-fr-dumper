<div class="center" align="center">
 
# Theses.fr dumper
 
 [![Build Status](https://travis-ci.org/Eonm/theses-fr-dumper.svg?branch=master)](https://travis-ci.org/Eonm/theses-fr-dumper)
 [![Coverage Status](https://coveralls.io/repos/github/Eonm/theses-fr-dumper/badge.svg?branch=master)](https://coveralls.io/github/Eonm/theses-fr-dumper?branch=master)
 [![dependency status](https://deps.rs/repo/github/eonm/theses-fr-dumper/status.svg)](https://deps.rs/repo/github/eonm/theses-fr-dumper)
 [![made-with-Rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)
 [![License: MIT](https://img.shields.io/badge/License-GPLv3-yellow.svg)](https://www.gnu.org/licenses/gpl-3.0.html)
 
</div>

_Theses.fr dumper_ permet de récupérer les données de [theses.fr](https://www.theses.fr) par lots.

## Usage

__en utilisant un fichier de sortie__

```sh
theses-fr-dumper -s 0 15 30 -f jsonl -o dump.jsonl
```

__en utilisant un pipe__

```sh
theses-fr-dumper -s 0 15 30 -f jsonl | grep -i "lorem ipsum"
```

### Création d'une séquence de téléchargement `-s num num num`

La séquence de téléchargement s'exprime de la façon suivante : `-s début incrément fin`.

Si aucune séquence de téléchargement n'est spécifiée _theses.fr dumper_ téléchargera l'ensemble des notices par lot de 10&nbsp;000.

### Formats de récupération des données `-f`

* CSV
* Json
* [Jsonl](http://jsonlines.org/)
* XML (à venir)

### Fichier de sortie `-o`

Cet argument permet de préciser le fichier de sortie. Si un fichier existe déjà son contenu sera effacé.

Sans l'argument `-o` les informations récupérées du serveur sont affichées dans directement dans console.

### Mode de connexion `-m keep-alive/reset`

_Theses.fr dumper_ permet de grader la connexion ouverte avec le serveur grâce à l'option `-m keep-alive`. Tous les lots seront téléchargé par la même connexion.

> :warning: L'option keep-alive peut entraîner un time out côté serveur. Par défaut chaque téléchargement de lots entraîne la création d'une nouvelle connexion avec le serveur.

## Build

```sh
cargo build --release
```

## Test

```sh
cargo test
```
