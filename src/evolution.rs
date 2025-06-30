use crate::Species;
use std::collections::{BTreeSet, btree_set};

#[derive(Debug)]
pub struct EvolutionLine(Inner);

#[derive(Debug)]
enum Inner {
    Unit(Option<Species>),
    Iter(btree_set::IntoIter<Species>),
}

impl EvolutionLine {
    pub fn new(species: Species) -> Self {
        if species.from().is_none() && species.to().is_none() {
            return Self(Inner::Unit(Some(species)));
        }

        let origin = {
            let mut origin = species;
            while let Some(from_species) = origin.from() {
                origin = from_species
            }
            origin
        };

        let mut seen = BTreeSet::new();
        seen.insert(origin);

        fn follow(species: Species, seen: &mut BTreeSet<Species>) {
            if let Some(to) = species.to() {
                for &to_species in to {
                    seen.insert(to_species);
                    follow(to_species, seen);
                }
            }
        }

        follow(origin, &mut seen);

        Self(Inner::Iter(seen.into_iter()))
    }
}

impl Iterator for EvolutionLine {
    type Item = Species;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            Inner::Unit(opt) => opt.take(),
            Inner::Iter(iter) => iter.next(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_evo_line {
        ($($s:ident),*$(,)?) => {
            for species in [$(Species::$s),*] {
                assert_eq!(EvolutionLine::new(species).collect::<Vec<_>>(), [$(Species::$s),*])
            }
        };
    }

    #[test]
    fn evo_line_tauros() {
        assert_evo_line![Tauros];
    }

    #[test]
    fn evo_line_bulbasaur() {
        assert_evo_line![Bulbasaur, Ivysaur, Venusaur];
    }

    #[test]
    fn evo_line_wurmple() {
        assert_evo_line![Wurmple, Silcoon, Beautifly, Cascoon, Dustox];
    }

    #[test]
    fn evo_line_eevee() {
        assert_evo_line![
            Eevee, Vaporeon, Jolteon, Flareon, Espeon, Umbreon, Leafeon, Glaceon, Sylveon
        ];
    }
}
