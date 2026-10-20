# Galère 3 : modifier une collection pendant qu'on la lit

## Le code

```rust
for (p, c) in &map {
    c = &process_cell(p, &map);   // veut mettre à jour chaque cellule
}
```

L'idée : parcourir toutes les positions, calculer la nouvelle cellule à partir de la map, puis la remplacer. Ça paraît évident, mais impossible de le faire compiler.

## Les difficultés de l'utilisateur

- **Deux problèmes empilés.** En corrigeant le premier (`c = …` au lieu de `*c = …`, déjà vu dans la galère 1), on tombe sur un second, plus profond. On a l'impression de tourner en rond.
- **Ne voit pas pourquoi « lire et modifier en même temps » est interdit.** Le raisonnement « je lis la map, je modifie une case » paraît naturel. Que Rust refuse un emprunt en lecture (`&map`) pendant un emprunt en écriture (`&mut map`) semble arbitraire.
- **Le compilateur montre un problème à la fois.** Il propose `mut c`, qui ne mène nulle part, et ne laisse pas deviner le vrai mur qui arrive juste après.
- **Ne relie pas la règle de Rust au jeu lui-même.** Pour être juste, chaque cellule doit compter ses voisins sur l'état *d'avant*. Modifier sur place fausserait le calcul. Rust bloque en fait un vrai bug, mais ce lien n'est pas visible.
- **La solution proposée reste opaque.** L'idée de passer par « une autre map » puis de remplacer l'ancienne n'a pas été comprise. Les étapes ne sont pas claires : pourquoi une deuxième collection, pourquoi `*p`, pourquoi l'affectation finale ne pose pas de problème.
- **Ne sait pas ce que donne `for … in &map`.** Le fait de recevoir des couples de références (`&Position`, `&Cell`) et non les valeurs, et ce qu'on a le droit d'en faire, n'est pas évident.
- **Frustration accumulée.** Le problème semble simple (« une position → une cellule → je la change »), et l'écart entre cette idée et ce que Rust accepte est décourageant.
