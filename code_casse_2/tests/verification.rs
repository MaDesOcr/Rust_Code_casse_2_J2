//! Tests de vérification. NE PAS MODIFIER CE FICHIER.
//! Lancer : cargo test
//! Pour un seul test : cargo test nom_du_test

use code_casse_2::*;

#[test]
fn t01_moyenne() {
    assert_eq!(moyenne(&[10, 12, 14]), 12.0);
    assert_eq!(moyenne(&[15, 16]), 15.5);
}

#[test]
fn t02_moyenne_liste_vide() {
    assert_eq!(
        moyenne(&[]),
        0.0,
        "la moyenne d'une liste vide doit valoir 0.0"
    );
}

#[test]
fn t03_note_max() {
    assert_eq!(note_max(&[12, 19, 7]), 19);
    assert_eq!(note_max(&[]), 0);
}

#[test]
fn t04_note_min() {
    assert_eq!(
        note_min(&[12, 19, 7]),
        7,
        "la plus basse note de [12, 19, 7] est 7"
    );
    assert_eq!(note_min(&[15]), 15);
    assert_eq!(note_min(&[]), NOTE_MAX);
}

#[test]
fn t05_ecart() {
    assert_eq!(ecart(&[12, 19, 7]), 12);
    assert_eq!(ecart(&[10, 10]), 0);
}

#[test]
fn t06_admission() {
    assert!(est_admis(10.0));
    assert!(est_admis(15.5));
    assert!(!est_admis(9.99));
}

#[test]
fn t07_mentions() {
    assert_eq!(mention(8.0), "insuffisant");
    assert_eq!(mention(10.0), "passable");
    assert_eq!(mention(12.5), "assez bien");
    assert_eq!(mention(14.0), "bien");
    assert_eq!(mention(17.0), "très bien");
}

#[test]
fn t08_nb_au_dessus() {
    assert_eq!(nb_au_dessus(&[8, 10, 15, 9, 12], 10), 3);
    assert_eq!(nb_au_dessus(&[8, 9], 10), 0);
}

#[test]
fn t09_repartition() {
    assert_eq!(repartition(&[5, 10, 13, 14, 20, 9]), [2, 2, 2]);
    assert_eq!(
        repartition(&[25, 18]),
        [0, 0, 1],
        "une note > 20 doit être ignorée"
    );
}

#[test]
fn t10_bonus() {
    let mut notes = vec![10, 15];
    appliquer_bonus(&mut notes, 2);
    assert_eq!(notes, vec![12, 17]);
}

#[test]
fn t11_bonus_plafonne() {
    let mut notes = vec![19, 20, 5];
    appliquer_bonus(&mut notes, 3);
    assert_eq!(
        notes,
        vec![20, 20, 8],
        "une note ne peut pas dépasser NOTE_MAX"
    );
}

#[test]
fn t12_ajouter_note() {
    let notes = ajouter_note(vec![12, 8], 15);
    assert_eq!(notes, vec![12, 8, 15]);
}

#[test]
fn t13_moyenne_ponderee() {
    assert_eq!(moyenne_ponderee(&[10, 16], &[1, 2]), 14.0);
    assert_eq!(moyenne_ponderee(&[12], &[0]), 0.0);
}

#[test]
fn t14_eliminatoire() {
    assert_eq!(premiere_eliminatoire(&[12, 4, 3]), 1);
    assert_eq!(premiere_eliminatoire(&[12, 15]), -1);
}

#[test]
fn t15_sur_100() {
    assert_eq!(sur_100(14), 70.0);
    assert_eq!(sur_100(0), 0.0);
    assert_eq!(
        sur_100(25),
        100.0,
        "une note sur 100 ne peut pas dépasser 100"
    );
}

#[test]
fn t16_mediane_impaire() {
    let mut notes = vec![15, 8, 12];
    assert_eq!(mediane(&mut notes), 12);
    assert_eq!(notes, vec![8, 12, 15], "la liste doit être triée");
}

#[test]
fn t17_mediane_paire() {
    assert_eq!(
        mediane(&mut [15, 8, 12, 10]),
        11,
        "médiane de 8 10 12 15 : (10 + 12) / 2"
    );
    assert_eq!(mediane(&mut [9, 14]), 11);
}

#[test]
fn t18_resume() {
    assert_eq!(
        resume(String::from("B3 Dev"), &[10, 15]),
        "B3 Dev : 2 notes, moyenne 12.50"
    );
}
