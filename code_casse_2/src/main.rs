//! Rapport de classe. Ce fichier est correct : ne le modifiez pas.

use code_casse_2::*;

fn main() {
    let mut notes = vec![12, 8, 15, 17, 9, 4, 19];
    println!("{}", resume(String::from("B3 Dev"), &notes));

    println!(
        "Meilleure : {} · Moins bonne : {} · Écart : {}",
        note_max(&notes),
        note_min(&notes),
        ecart(&notes)
    );

    let moy = moyenne(&notes);
    println!("Mention : {} · Admis : {}", mention(moy), est_admis(moy));
    println!("Notes ≥ 10 : {}", nb_au_dessus(&notes, 10));
    println!("Répartition [<10, 10-13, ≥14] : {:?}", repartition(&notes));
    println!(
        "Première éliminatoire : position {}",
        premiere_eliminatoire(&notes)
    );
    println!("Meilleure note sur 100 : {}", sur_100(note_max(&notes)));
    println!(
        "Moyenne pondérée : {:.2}",
        moyenne_ponderee(&[12, 15, 9], &[2, 3, 1])
    );

    notes = ajouter_note(notes, 14);
    appliquer_bonus(&mut notes, 2);
    println!("Après bonus : {notes:?}");
    println!("Médiane : {}", mediane(&mut notes));
}
