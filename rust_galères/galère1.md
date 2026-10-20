# Galère : modifier une valeur à travers une référence

## Le code

```rust
let mut state = map.get(&Position { x: 0, y: 0 }).unwrap();
state = &State::Alive;          // pensait modifier la map

let state = map.get_mut(&Position { x: 0, y: 0 }).unwrap();
state = &mut State::Alive;      // 2e essai, même erreur de fond
```

## Les difficultés de l'utilisateur

- **Confond « changer la référence » et « changer la valeur pointée ».** Écrit `state = …` en pensant écrire dans la map. Ne pense pas spontanément à `*state = …`.
- **Mélange les deux `mut`.** Croit que `let mut state` donne le droit de modifier la valeur pointée, alors que ce droit vient de `&mut`.
- **Ne sait pas quand une valeur est « move ».** Après `map.insert(...)`, pense que `map` a été consommée. Ne relie pas ça au `&self` / `&mut self` / `self` de la signature.
- **Ne sait pas si on peut modifier quelque chose via une référence.** A dû demander explicitement si c'était possible.
- **`*` marche dans un cas et pas dans l'autre.** `*p` donne `cannot move out`, et l'utilisateur ne comprend pas pourquoi, puisque `*` « devrait » donner la valeur.
- **Ne trouve pas une méthode qui n'apparaît pas.** `map.insert` était invisible (traits `Eq` + `Hash` manquants), sans indice sur la cause.
- **Fait confiance aux suggestions du compilateur**, même quand elles mènent à un code qui compile sans rien faire (`let mut state`).
- **N'a jamais vu ces cas dans les tutos.** Les tutos montrent `*r += 1` sur une variable simple, jamais une référence sortie d'une collection.
