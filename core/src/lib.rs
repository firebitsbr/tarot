use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Couleur {
    Carreau,
    Pique,
    Trefle,
    Coeur,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Valeur {
    As,
    Deux,
    Trois,
    Quatre,
    Cinq,
    Six,
    Sept,
    Huit,
    Neuf,
    Dix,
    Valet,
    Cavalier,
    Dame,
    Roi,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Atout {
    Excuse,
    Un,
    Deux,
    Trois,
    Quatre,
    Cinq,
    Six,
    Sept,
    Huit,
    Neuf,
    Dix,
    Onze,
    Douze,
    Treize,
    Quatorze,
    Quinze,
    Seize,
    DixSept,
    Dixhuit,
    Dixneuf,
    Vingt,
    VingEtUn,
}

macro_rules! c {
    (Excuse) => { Carte::Atout(Atout::Excuse) };
    (Atout $a:ident) => { Carte::Atout(Atout::$a) };
    ($v:ident de $c:ident) => { Carte::CarteNorm(Couleur::$c, Valeur::$v) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creer_jeu() {
        let jeu = creer_jeu();
        assert_eq!(jeu.len(), 78);
        assert_eq!(jeu[10], c!(Valet de Carreau));
    }

    #[test]
    fn test_plus_forte_que() {
        assert!(c!(Roi de Coeur).plus_forte_que(&c!(Dame de Coeur), Couleur::Coeur));
        assert!(c!(Valet de Coeur).plus_forte_que(&c!(Six de Coeur), Couleur::Coeur));
        assert!(c!(Deux de Pique).plus_forte_que(&c!(Dame de Coeur), Couleur::Pique));
        assert!(c!(Atout Vingt).plus_forte_que(&c!(Atout Dix), Couleur::Carreau));
    }

    #[test]
    fn test_couleur_demandee() {
        assert_eq!(couleur_demandee(&[c!(Roi de Coeur), c!(Atout Dix)]), Some(Couleur::Coeur));
    }

    #[test]
    fn test_cartes_jouables() {
        // Si on peut jouer la couleur demandée
        let jouables = cartes_jouables(
            &[c!(Trois de Carreau), c!(Six de Carreau), c!(Sept de Carreau)],
            &[
                c!(Deux de Carreau),
                c!(Trois de Coeur),
                c!(Atout Trois),
                c!(Atout Douze),
            ],
            false,
            Couleur::Carreau,
        );
        assert_eq!(jouables, vec![ c!(Deux de Carreau) ]);

        let jouables = cartes_jouables(
            &[c!(Trois de Carreau), c!(Six de Carreau), c!(Sept de Carreau)],
            &[
                c!(Deux de Carreau),
                c!(Trois de Coeur),
                c!(Atout Trois),
                c!(Atout Douze),
                c!(Excuse),
            ],
            false,
            Couleur::Carreau,
        );

        assert_eq!(jouables, vec![ c!(Deux de Carreau), c!(Excuse) ]);
    }

    #[test]
    fn test_gagnant_de_tour() {
        assert_eq!(gagnant_de_tour(&[
            (Joueur::creer("Johan", Equipe::Attaque), c!(Atout Quinze)),
            (Joueur::creer("Mathis", Equipe::Defense), c!(Six de Pique)),
            (Joueur::creer("Clara", Equipe::Defense), c!(Sept de Pique)),
            (Joueur::creer("Pénélope", Equipe::Defense), c!(Atout Treize)),
            (Joueur::creer("Théo", Equipe::Defense), c!(Deux de Carreau)),
        ]).pseudo, "Johan");
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Carte {
    CarteNorm(Couleur, Valeur),
    Atout(Atout),
}

#[derive(Clone, Eq, PartialEq)]
enum Equipe {
    Attaque,
    Defense,
}

#[derive(Clone, Eq, PartialEq)]
struct Joueur<'a>{
    pseudo : &'a str,
    equipe : Equipe,
    main : Vec<Carte>,
    plis: Vec<&'a Carte>,
}

impl<'a> Joueur<'a> {
    fn creer(nom: &'a str, eq: Equipe) -> Joueur<'a> {
        Joueur {
            pseudo: nom,
            equipe: eq,
            main: Vec::new(),
            plis: Vec::new(),
        }
    }
}

type Table<'a> = &'a [(Joueur<'a>, Carte)];

fn range_pli<'a>(gagnant_tour: &'a mut Joueur<'a>, table: Table<'a>) {
    for t in table {
        gagnant_tour.plis.push(&t.1);
    }
}

impl Carte {
    fn plus_forte_que(&self, autre: &Carte, appelee: Couleur) -> bool {
        match self {
            Carte::CarteNorm(coul, val) => match autre {
                Carte::Atout(_) => false,
                Carte::CarteNorm(coul_autre, val_autre) => {
                    if *coul == *coul_autre {
                        assert!(*val != *val_autre);
                        *val > *val_autre
                    } else {
                        appelee == *coul
                    }
                }
            },
            Carte::Atout(at) => match autre {
                Carte::CarteNorm(_, _) => true,
                Carte::Atout(autre_at) => *at > *autre_at,
            },
        }
    }
}

fn couleur_demandee(carte: &[Carte]) -> Option<Couleur> {
    if let Carte::CarteNorm(ref coul, _) = carte[0] {
        Some(coul.clone())
    } else {
        None
    }
}

fn a_couleur(cartes: &[Carte], couleur: Option<Couleur>) -> bool {
    for c in cartes {
        match c {
            Carte::CarteNorm(c, _) => {
                if let Some(coul_demandee) = couleur.clone() {
                    if *c == coul_demandee {
                        return true;
                    }
                }
            }
            Carte::Atout(_) => {
                if couleur.is_none() {
                    return true;
                }
            }
        }
    }
    return false;
}

/// Donne le plus grand atout d'une liste de carte si il existe
fn atout_max(cartes: &[Carte]) -> Option<Atout> {
    cartes.iter().fold(None, |max, carte| match carte {
        Carte::CarteNorm(_, _) => max,
        Carte::Atout(at) => {
            if let Some(at_max) = max.clone() {
                if *at > at_max {
                    Some(at.clone())
                } else {
                    max.clone()
                }
            } else {
                max.clone()
            }
        }
    })
}

fn gagnant_de_tour<'a>(table: Table<'a>) -> Joueur<'a> {
    let demandee = couleur_demandee(&table.iter().map(|x| x.1.clone()).collect::<Vec<_>>()[..]);
    table
        .iter()
            .fold(table[0].clone(), |gagnant, joueur| {
            if gagnant
                .1
                .plus_forte_que(&joueur.1, demandee.clone().unwrap_or(Couleur::Carreau))
            {
                gagnant
            } else {
                joueur.clone()
            }
        })
        .0
}

fn cartes_jouables(
    cartes_jouees: &[Carte],
    cartes_joueur: &[Carte],
    premier_tour: bool,
    roi_appele: Couleur,
) -> Vec<Carte> {
    cartes_joueur.iter().filter(|carte| {
    if let Carte::Atout(Atout::Excuse) = carte {
      return true;
    }

    if premier_tour && cartes_jouees.len() == 0 {
      match carte {
        Carte::CarteNorm(coul, val) => (*coul != roi_appele) || (*val == Valeur::Roi),
        Carte::Atout(_) => true,
      }
    } else {
      let couleur_dem = couleur_demandee(&cartes_jouees);
      if let Some(coul) = couleur_dem {
        if a_couleur(cartes_joueur, Some(coul.clone())) {
          match carte {
            Carte::CarteNorm(c,_) => *c == coul,
            _ => false,
          }
        } else {
          if a_couleur(cartes_joueur, None) {
            match carte {
              Carte::Atout(val) => if let Some(max) = atout_max(cartes_jouees) {
                if let Some(atout_max_joueur) = atout_max(cartes_joueur) {
                  if atout_max_joueur > max {
                    *val > max
                  } else {
                    true
                  }
                } else {
                  unreachable!("Pas d'atout max mais un atout quand même. Ça pulse pas.")
                }
              } else {
                true
              },
              _ => false,
            }
          } else {
            true
          }
        }
      } else {
        if a_couleur(cartes_joueur, None) {
          match carte {
            Carte::Atout(val) => if let Some(max) = atout_max(cartes_jouees) {
              if let Some(atout_max_joueur) = atout_max(cartes_joueur) {
                if atout_max_joueur > max {
                  *val > max
                } else {
                  true
                }
              } else {
                unreachable!("Pas d'atout max mais un atout quand même. Ça pulse pas.")
              }
            } else {
              true
            },
            _ => false,
          }
        } else {
          true
        }
      }
    }
  }).map(|x| x.clone()).collect()
}

/// Crée un jeu de carte, avec les cartes classées.
fn creer_jeu() -> Vec<Carte> {
    let mut jeu = Vec::with_capacity(78);
    for coul in [
        Couleur::Carreau,
        Couleur::Coeur,
        Couleur::Trefle,
        Couleur::Pique,
    ]
    .iter()
    {
        for val in [
            Valeur::As,
            Valeur::Deux,
            Valeur::Trois,
            Valeur::Quatre,
            Valeur::Cinq,
            Valeur::Six,
            Valeur::Sept,
            Valeur::Huit,
            Valeur::Neuf,
            Valeur::Dix,
            Valeur::Valet,
            Valeur::Cavalier,
            Valeur::Dame,
            Valeur::Roi,
        ]
        .iter()
        {
            jeu.push(Carte::CarteNorm(coul.clone(), val.clone()));
        }
    }

    for atout in [
        Atout::Un,
        Atout::Deux,
        Atout::Trois,
        Atout::Quatre,
        Atout::Cinq,
        Atout::Six,
        Atout::Sept,
        Atout::Huit,
        Atout::Neuf,
        Atout::Dix,
        Atout::Onze,
        Atout::Douze,
        Atout::Treize,
        Atout::Quatorze,
        Atout::Quinze,
        Atout::Seize,
        Atout::DixSept,
        Atout::Dixhuit,
        Atout::Dixneuf,
        Atout::Vingt,
        Atout::VingEtUn,
        Atout::Excuse,
    ]
    .iter()
    {
        jeu.push(Carte::Atout(atout.clone()));
    }
    jeu
}

enum Mise {
    Petite,
    Pousse,
    Garde,
    GardeSans,
    GardeContre,
    Chelem,
}
struct PrePartie<'a> {
    joueurs : Vec<Joueur<'a>>,
    chien : Vec<&'a Carte>,
    mise_mini : Mise,
}

fn distrib (mut prep : PrePartie) -> PrePartie {
    //on met le jeu trié en bordel pour le distribuer
    let mut jeu = creer_jeu();
    let mut rng = thread_rng();
    jeu.shuffle(&mut rng);

    //on fait le chien a la zgeg en prenant les premières cartes du jeu en bordel
    let nb_joueur = prep.joueurs.len();
    let nb_chien = if nb_joueur == 5 {3} else {6};
    let mut chien = vec![];
    for x in 1..=nb_chien {
        chien.push(jeu[x].clone());
        jeu.remove(x);
    }
    
    //boucle infinie pour distribuer les cartes 3 par 3 à chaque joueur
    let mut i = 0;
    let mut it = jeu.into_iter();
    loop {
        let carte1 = it.next();
        let carte2 = it.next();
        let carte3 = it.next();
        
        if carte1.is_none() {
            break;
        }
        
        prep.joueurs[i % nb_joueur].main.push(carte1.unwrap());
        prep.joueurs[i % nb_joueur].main.push(carte2.unwrap());
        prep.joueurs[i % nb_joueur].main.push(carte3.unwrap());
        
        i+=1;
    }
     //on renvoie prep #tehc
    prep

}

fn input(msg: &str) -> String {
    println!("{}", msg);
    let mut res = String::new();
    std::io::stdin().read_line(&mut res).unwrap();
    res.trim().to_string()
}

fn main() {
    println!("On va jouer au TAROT !!!!");
    let _nb_joueur: i32 = 5; //input("Combien de joueurs ?").parse().unwrap();
}
