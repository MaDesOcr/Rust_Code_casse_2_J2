//! Exercice « Code cassé 2 » — Jour 2 — CORRIGÉ.
//! Chaque correction est repérée par son numéro [n] (voir corrige.md).

/// Note maximale possible.
pub const NOTE_MAX: u32 = 20;

/// Moyenne des notes. Renvoie 0.0 si la liste est vide.
pub fn moyenne(notes: &[u32]) -> f64 {
    if notes.is_empty() {
        return 0.0; // [14] sinon 0.0 / 0.0 donne NaN
    }
    let mut total = 0;
    for note in notes {
        total += note;
    }
    total as f64 / notes.len() as f64 // [1] conversions explicites
}

/// Plus haute note de la liste (0 si la liste est vide).
pub fn note_max(notes: &[u32]) -> u32 {
    let mut max = 0; // [8]
    for &note in notes {
        if note > max {
            max = note;
        }
    }
    max
}

/// Plus basse note de la liste (NOTE_MAX si la liste est vide).
pub fn note_min(notes: &[u32]) -> u32 {
    let mut min = NOTE_MAX; // [15] partir de 0 donnait toujours 0
    for &note in notes {
        if note < min {
            min = note;
        }
    }
    min
}

/// Écart entre la meilleure et la moins bonne note.
pub fn ecart(notes: &[u32]) -> u32 {
    note_max(notes) - note_min(notes) // [16] min - max : dépassement d'entier (panique)
}

/// Vrai si la moyenne permet d'être admis (10 ou plus).
pub fn est_admis(moyenne: f64) -> bool {
    moyenne >= 10.0 // [2] sans « ; » : c'est la valeur renvoyée
}

/// Mention correspondant à une moyenne :
/// moins de 10 « insuffisant », de 10 à moins de 12 « passable »,
/// de 12 à moins de 14 « assez bien », de 14 à moins de 16 « bien », 16 et plus « très bien ».
pub fn mention(moyenne: f64) -> &'static str {
    // [17] tester du seuil le plus haut au plus bas ; [3] 12.0 et non 12
    if moyenne >= 16.0 {
        "très bien"
    } else if moyenne >= 14.0 {
        "bien"
    } else if moyenne >= 12.0 {
        "assez bien"
    } else if moyenne >= 10.0 {
        "passable"
    } else {
        "insuffisant"
    }
}

/// Nombre de notes supérieures ou égales au seuil.
pub fn nb_au_dessus(notes: &[u32], seuil: u32) -> usize {
    let mut compte = 0;
    for note in notes {
        if *note >= seuil {
            // [4] note est un &u32 : on le déréférence
            compte += 1;
        }
    }
    compte
}

/// Répartition des notes en trois tranches :
/// [0] moins de 10, [1] de 10 à 13, [2] de 14 à 20. Les notes invalides (> 20) sont ignorées.
pub fn repartition(notes: &[u32]) -> [u32; 3] {
    let mut tranches = [0, 0, 0];
    for &note in notes {
        let indice: usize = match note {
            // [5] un indice de tableau est un usize
            0..=9 => 0,
            10..=13 => 1,
            14..=20 => 2,
            _ => continue, // [9] note invalide : ignorée
        };
        tranches[indice] += 1;
    }
    tranches
}

/// Ajoute `bonus` points à chaque note, sans jamais dépasser NOTE_MAX.
pub fn appliquer_bonus(notes: &mut [u32], bonus: u32) {
    for note in notes.iter_mut() {
        // [10] iter_mut pour modifier les éléments
        *note += bonus;
        if *note > NOTE_MAX {
            // [18] plafonnement
            *note = NOTE_MAX;
        }
    }
}

/// Ajoute une note à la liste et renvoie la liste complétée.
pub fn ajouter_note(mut notes: Vec<u32>, note: u32) -> Vec<u32> {
    // [11] le paramètre possédé doit être déclaré mut pour être modifié
    notes.push(note);
    notes
}

/// Moyenne pondérée : chaque note est multipliée par son coefficient
/// (les deux listes ont la même longueur). Renvoie 0.0 si la somme des coefficients est nulle.
pub fn moyenne_ponderee(notes: &[u32], coefs: &[u32]) -> f64 {
    let mut total = 0;
    let mut somme_coefs = 0;
    for i in 0..notes.len() {
        // [19] 0..len et non 0..=len (indice hors limites)
        total += notes[i] * coefs[i];
        somme_coefs += coefs[i];
    }
    if somme_coefs == 0 {
        return 0.0;
    }
    total as f64 / somme_coefs as f64
}

/// Position (à partir de 0) de la première note éliminatoire (strictement inférieure à 5),
/// ou -1 s'il n'y en a pas.
pub fn premiere_eliminatoire(notes: &[u32]) -> i32 {
    for (i, &note) in notes.iter().enumerate() {
        if note < 5 {
            return i as i32; // [6] enumerate fournit un usize
        }
    }
    -1
}

/// Convertit une note sur 20 en note sur 100, plafonnée à 100.
pub fn sur_100(note: u32) -> f64 {
    let resultat = note;
    let resultat = resultat as f64 * 5.0; // [7] masquage : nouvelle variable, nouveau type
    resultat.min(100.0)
}

/// Médiane : trie les notes, puis renvoie la valeur centrale ;
/// pour un nombre pair de notes, la moyenne entière (arrondie vers le bas) des deux valeurs centrales.
/// La liste n'est jamais vide.
pub fn mediane(notes: &mut [u32]) -> u32 {
    let premiere = notes[0]; // [12] copie du u32 au lieu d'une référence
    notes.sort();
    println!("(première note avant tri : {premiere})");
    let milieu = notes.len() / 2;
    if notes.len() % 2 == 0 {
        // [20] nombre pair : moyenne des deux valeurs centrales
        (notes[milieu - 1] + notes[milieu]) / 2
    } else {
        notes[milieu]
    }
}

/// Résumé d'une classe : « <nom> : <n> notes, moyenne <moyenne avec 2 décimales> ».
pub fn resume(nom_classe: String, notes: &[u32]) -> String {
    let titre = nom_classe;
    let moy = moyenne(notes);
    // [13] nom_classe a été déplacée dans titre
    format!("{titre} : {} notes, moyenne {:.2}", notes.len(), moy)
}
