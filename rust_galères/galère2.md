# Galère 2 : les fonctions chaînées sur les collections

## Le code

```rust
DIRECTIONS.iter().for_each(|d| {
    let select = Position::new(my_pos.x + d.0, my_pos.y + d.1);
    let state_select = what_is_here(&my_pos, &map);
    // ... comment compter les voisines vivantes ?
});
```

## Les difficultés de l'utilisateur

- **Ne connaît que `for_each`** et essaie de tout faire avec. Bloque dès qu'il faut récupérer un résultat, ici compter.
- **Le chaînage n'est pas intuitif.** L'idée qu'une méthode renvoie « un autre itérateur » sur lequel on rappelle une méthode ne lui parle pas.
- **N'arrive pas à lire la doc des itérateurs.** Les signatures et les types de retour (`Map<Self, F>`, etc.) restent opaques.
- **Ne sait pas quelle méthode choisir.** Ne fait pas le lien entre « ce que je veux faire » (transformer, garder, compter) et le nom de la méthode.
- **Est surpris de recevoir des références** au lieu des valeurs, sans savoir d'où elles viennent ni quand ça arrive.
- **Est tiraillé entre `for i in …` et les chaînes.** A envie de revenir à la boucle classique, mais croit que ce n'est « pas très Rust ».
- **Se sent vite submergé** dès que la logique demande plusieurs étapes (calculer la voisine, lire son état, compter).
