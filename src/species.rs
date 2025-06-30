use crate::BuiltinType;
use enum_assoc::Assoc;
use std::collections::HashSet;

include!("./species.gen.rs");

impl Species {
    pub fn evolution_line(self) -> Vec<Self> {
        if self.from().is_none() && self.to().is_none() {
            return vec![self];
        }

        let origin = {
            let mut origin = self;
            while let Some(from_species) = origin.from() {
                origin = from_species
            }
            origin
        };

        let mut seen = HashSet::new();
        seen.insert(origin);

        fn follow(species: Species, seen: &mut HashSet<Species>) {
            if let Some(to) = species.to() {
                for &to_species in to {
                    seen.insert(to_species);
                    follow(to_species, seen);
                }
            }
        }

        follow(origin, &mut seen);

        let mut out = seen.into_iter().collect::<Vec<_>>();
        out.sort();
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_evo_line {
        ($($s:ident),*$(,)?) => {
            for species in [$(Species::$s),*] {
                assert_eq!(species.evolution_line(), [$(Species::$s),*])
            }
        };
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
