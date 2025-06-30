#[repr(u16)]
#[derive(Assoc, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[func(pub const fn name(self) -> &'static str)]
#[func(pub(crate) const fn ty(self) -> (BuiltinType, Option<BuiltinType>))]
#[func(pub(crate) const fn from(self) -> Option<Species>)]
#[func(pub(crate) const fn to(self) -> Option<&'static [Species]>)]
#[rustfmt::skip]
pub enum Species {
  #[assoc(name = "Bulbasaur", ty = (BuiltinType::Grass, Some(BuiltinType::Poison)), to = &[Species::Ivysaur])]
  Bulbasaur = 1,
  #[assoc(name = "Ivysaur", ty = (BuiltinType::Grass, Some(BuiltinType::Poison)), from = Species::Bulbasaur, to = &[Species::Venusaur])]
  Ivysaur = 2,
  #[assoc(name = "Venusaur", ty = (BuiltinType::Grass, Some(BuiltinType::Poison)), from = Species::Ivysaur)]
  Venusaur = 3,
  #[assoc(name = "Charmander", ty = (BuiltinType::Fire, None), to = &[Species::Charmeleon])]
  Charmander = 4,
  #[assoc(name = "Charmeleon", ty = (BuiltinType::Fire, None), from = Species::Charmander, to = &[Species::Charizard])]
  Charmeleon = 5,
  #[assoc(name = "Charizard", ty = (BuiltinType::Fire, Some(BuiltinType::Flying)), from = Species::Charmeleon)]
  Charizard = 6,
  #[assoc(name = "Squirtle", ty = (BuiltinType::Water, None), to = &[Species::Wartortle])]
  Squirtle = 7,
  #[assoc(name = "Wartortle", ty = (BuiltinType::Water, None), from = Species::Squirtle, to = &[Species::Blastoise])]
  Wartortle = 8,
  #[assoc(name = "Blastoise", ty = (BuiltinType::Water, None), from = Species::Wartortle)]
  Blastoise = 9,
  #[assoc(name = "Caterpie", ty = (BuiltinType::Bug, None), to = &[Species::Metapod])]
  Caterpie = 10,
  #[assoc(name = "Metapod", ty = (BuiltinType::Bug, None), from = Species::Caterpie, to = &[Species::Butterfree])]
  Metapod = 11,
  #[assoc(name = "Butterfree", ty = (BuiltinType::Bug, Some(BuiltinType::Flying)), from = Species::Metapod)]
  Butterfree = 12,
  #[assoc(name = "Weedle", ty = (BuiltinType::Bug, Some(BuiltinType::Poison)), to = &[Species::Kakuna])]
  Weedle = 13,
  #[assoc(name = "Kakuna", ty = (BuiltinType::Bug, Some(BuiltinType::Poison)), from = Species::Weedle, to = &[Species::Beedrill])]
  Kakuna = 14,
  #[assoc(name = "Beedrill", ty = (BuiltinType::Bug, Some(BuiltinType::Poison)), from = Species::Kakuna)]
  Beedrill = 15,
  #[assoc(name = "Pidgey", ty = (BuiltinType::Normal, Some(BuiltinType::Flying)), to = &[Species::Pidgeotto])]
  Pidgey = 16,
  #[assoc(name = "Pidgeotto", ty = (BuiltinType::Normal, Some(BuiltinType::Flying)), from = Species::Pidgey, to = &[Species::Pidgeot])]
  Pidgeotto = 17,
  #[assoc(name = "Pidgeot", ty = (BuiltinType::Normal, Some(BuiltinType::Flying)), from = Species::Pidgeotto)]
  Pidgeot = 18,
  #[assoc(name = "Rattata", ty = (BuiltinType::Normal, None), to = &[Species::Raticate])]
  Rattata = 19,
  #[assoc(name = "Raticate", ty = (BuiltinType::Normal, None), from = Species::Rattata)]
  Raticate = 20,
  #[assoc(name = "Spearow", ty = (BuiltinType::Normal, Some(BuiltinType::Flying)), to = &[Species::Fearow])]
  Spearow = 21,
  #[assoc(name = "Fearow", ty = (BuiltinType::Normal, Some(BuiltinType::Flying)), from = Species::Spearow)]
  Fearow = 22,
  #[assoc(name = "Ekans", ty = (BuiltinType::Poison, None), to = &[Species::Arbok])]
  Ekans = 23,
  #[assoc(name = "Arbok", ty = (BuiltinType::Poison, None), from = Species::Ekans)]
  Arbok = 24,
  #[assoc(name = "Pikachu", ty = (BuiltinType::Electric, None), from = Species::Pichu, to = &[Species::Raichu])]
  Pikachu = 25,
  #[assoc(name = "Raichu", ty = (BuiltinType::Electric, None), from = Species::Pikachu)]
  Raichu = 26,
  #[assoc(name = "Sandshrew", ty = (BuiltinType::Ground, None), to = &[Species::Sandslash])]
  Sandshrew = 27,
  #[assoc(name = "Sandslash", ty = (BuiltinType::Ground, None), from = Species::Sandshrew)]
  Sandslash = 28,
  #[assoc(name = "Nidoran♀", ty = (BuiltinType::Poison, None), to = &[Species::Nidorina])]
  NidoranF = 29,
  #[assoc(name = "Nidorina", ty = (BuiltinType::Poison, None), from = Species::NidoranF, to = &[Species::Nidoqueen])]
  Nidorina = 30,
  #[assoc(name = "Nidoqueen", ty = (BuiltinType::Poison, Some(BuiltinType::Ground)), from = Species::Nidorina)]
  Nidoqueen = 31,
  #[assoc(name = "Nidoran♂", ty = (BuiltinType::Poison, None), to = &[Species::Nidorino])]
  NidoranM = 32,
  #[assoc(name = "Nidorino", ty = (BuiltinType::Poison, None), from = Species::NidoranM, to = &[Species::Nidoking])]
  Nidorino = 33,
  #[assoc(name = "Nidoking", ty = (BuiltinType::Poison, Some(BuiltinType::Ground)), from = Species::Nidorino)]
  Nidoking = 34,
  #[assoc(name = "Clefairy", ty = (BuiltinType::Fairy, None), from = Species::Cleffa, to = &[Species::Clefable])]
  Clefairy = 35,
  #[assoc(name = "Clefable", ty = (BuiltinType::Fairy, None), from = Species::Clefairy)]
  Clefable = 36,
  #[assoc(name = "Vulpix", ty = (BuiltinType::Fire, None), to = &[Species::Ninetales])]
  Vulpix = 37,
  #[assoc(name = "Ninetales", ty = (BuiltinType::Fire, None), from = Species::Vulpix)]
  Ninetales = 38,
  #[assoc(name = "Jigglypuff", ty = (BuiltinType::Normal, Some(BuiltinType::Fairy)), from = Species::Igglybuff, to = &[Species::Wigglytuff])]
  Jigglypuff = 39,
  #[assoc(name = "Wigglytuff", ty = (BuiltinType::Normal, Some(BuiltinType::Fairy)), from = Species::Jigglypuff)]
  Wigglytuff = 40,
  #[assoc(name = "Zubat", ty = (BuiltinType::Poison, Some(BuiltinType::Flying)), to = &[Species::Golbat])]
  Zubat = 41,
  #[assoc(name = "Golbat", ty = (BuiltinType::Poison, Some(BuiltinType::Flying)), from = Species::Zubat, to = &[Species::Crobat])]
  Golbat = 42,
  #[assoc(name = "Oddish", ty = (BuiltinType::Grass, Some(BuiltinType::Poison)), to = &[Species::Gloom])]
  Oddish = 43,
  #[assoc(name = "Gloom", ty = (BuiltinType::Grass, Some(BuiltinType::Poison)), from = Species::Oddish, to = &[Species::Vileplume, Species::Bellossom])]
  Gloom = 44,
  #[assoc(name = "Vileplume", ty = (BuiltinType::Grass, Some(BuiltinType::Poison)), from = Species::Gloom)]
  Vileplume = 45,
  #[assoc(name = "Paras", ty = (BuiltinType::Bug, Some(BuiltinType::Grass)), to = &[Species::Parasect])]
  Paras = 46,
  #[assoc(name = "Parasect", ty = (BuiltinType::Bug, Some(BuiltinType::Grass)), from = Species::Paras)]
  Parasect = 47,
  #[assoc(name = "Venonat", ty = (BuiltinType::Bug, Some(BuiltinType::Poison)), to = &[Species::Venomoth])]
  Venonat = 48,
  #[assoc(name = "Venomoth", ty = (BuiltinType::Bug, Some(BuiltinType::Poison)), from = Species::Venonat)]
  Venomoth = 49,
  #[assoc(name = "Diglett", ty = (BuiltinType::Ground, None), to = &[Species::Dugtrio])]
  Diglett = 50,
  #[assoc(name = "Dugtrio", ty = (BuiltinType::Ground, None), from = Species::Diglett)]
  Dugtrio = 51,
  #[assoc(name = "Meowth", ty = (BuiltinType::Normal, None), to = &[Species::Persian, Species::Perrserker])]
  Meowth = 52,
  #[assoc(name = "Persian", ty = (BuiltinType::Normal, None), from = Species::Meowth)]
  Persian = 53,
  #[assoc(name = "Psyduck", ty = (BuiltinType::Water, None), to = &[Species::Golduck])]
  Psyduck = 54,
  #[assoc(name = "Golduck", ty = (BuiltinType::Water, None), from = Species::Psyduck)]
  Golduck = 55,
  #[assoc(name = "Mankey", ty = (BuiltinType::Fighting, None), to = &[Species::Primeape])]
  Mankey = 56,
  #[assoc(name = "Primeape", ty = (BuiltinType::Fighting, None), from = Species::Mankey, to = &[Species::Annihilape])]
  Primeape = 57,
  #[assoc(name = "Growlithe", ty = (BuiltinType::Fire, None), to = &[Species::Arcanine])]
  Growlithe = 58,
  #[assoc(name = "Arcanine", ty = (BuiltinType::Fire, None), from = Species::Growlithe)]
  Arcanine = 59,
  #[assoc(name = "Poliwag", ty = (BuiltinType::Water, None), to = &[Species::Poliwhirl])]
  Poliwag = 60,
  #[assoc(name = "Poliwhirl", ty = (BuiltinType::Water, None), from = Species::Poliwag, to = &[Species::Poliwrath, Species::Politoed])]
  Poliwhirl = 61,
  #[assoc(name = "Poliwrath", ty = (BuiltinType::Water, Some(BuiltinType::Fighting)), from = Species::Poliwhirl)]
  Poliwrath = 62,
  #[assoc(name = "Abra", ty = (BuiltinType::Psychic, None), to = &[Species::Kadabra])]
  Abra = 63,
  #[assoc(name = "Kadabra", ty = (BuiltinType::Psychic, None), from = Species::Abra, to = &[Species::Alakazam])]
  Kadabra = 64,
  #[assoc(name = "Alakazam", ty = (BuiltinType::Psychic, None), from = Species::Kadabra)]
  Alakazam = 65,
  #[assoc(name = "Machop", ty = (BuiltinType::Fighting, None), to = &[Species::Machoke])]
  Machop = 66,
  #[assoc(name = "Machoke", ty = (BuiltinType::Fighting, None), from = Species::Machop, to = &[Species::Machamp])]
  Machoke = 67,
  #[assoc(name = "Machamp", ty = (BuiltinType::Fighting, None), from = Species::Machoke)]
  Machamp = 68,
  #[assoc(name = "Bellsprout", ty = (BuiltinType::Grass, Some(BuiltinType::Poison)), to = &[Species::Weepinbell])]
  Bellsprout = 69,
  #[assoc(name = "Weepinbell", ty = (BuiltinType::Grass, Some(BuiltinType::Poison)), from = Species::Bellsprout, to = &[Species::Victreebel])]
  Weepinbell = 70,
  #[assoc(name = "Victreebel", ty = (BuiltinType::Grass, Some(BuiltinType::Poison)), from = Species::Weepinbell)]
  Victreebel = 71,
  #[assoc(name = "Tentacool", ty = (BuiltinType::Water, Some(BuiltinType::Poison)), to = &[Species::Tentacruel])]
  Tentacool = 72,
  #[assoc(name = "Tentacruel", ty = (BuiltinType::Water, Some(BuiltinType::Poison)), from = Species::Tentacool)]
  Tentacruel = 73,
  #[assoc(name = "Geodude", ty = (BuiltinType::Rock, Some(BuiltinType::Ground)), to = &[Species::Graveler])]
  Geodude = 74,
  #[assoc(name = "Graveler", ty = (BuiltinType::Rock, Some(BuiltinType::Ground)), from = Species::Geodude, to = &[Species::Golem])]
  Graveler = 75,
  #[assoc(name = "Golem", ty = (BuiltinType::Rock, Some(BuiltinType::Ground)), from = Species::Graveler)]
  Golem = 76,
  #[assoc(name = "Ponyta", ty = (BuiltinType::Fire, None), to = &[Species::Rapidash])]
  Ponyta = 77,
  #[assoc(name = "Rapidash", ty = (BuiltinType::Fire, None), from = Species::Ponyta)]
  Rapidash = 78,
  #[assoc(name = "Slowpoke", ty = (BuiltinType::Water, Some(BuiltinType::Psychic)), to = &[Species::Slowbro, Species::Slowking])]
  Slowpoke = 79,
  #[assoc(name = "Slowbro", ty = (BuiltinType::Water, Some(BuiltinType::Psychic)), from = Species::Slowpoke)]
  Slowbro = 80,
  #[assoc(name = "Magnemite", ty = (BuiltinType::Electric, Some(BuiltinType::Steel)), to = &[Species::Magneton])]
  Magnemite = 81,
  #[assoc(name = "Magneton", ty = (BuiltinType::Electric, Some(BuiltinType::Steel)), from = Species::Magnemite, to = &[Species::Magnezone])]
  Magneton = 82,
  #[assoc(name = "Farfetch’d", ty = (BuiltinType::Normal, Some(BuiltinType::Flying)), to = &[Species::Sirfetchd])]
  Farfetchd = 83,
  #[assoc(name = "Doduo", ty = (BuiltinType::Normal, Some(BuiltinType::Flying)), to = &[Species::Dodrio])]
  Doduo = 84,
  #[assoc(name = "Dodrio", ty = (BuiltinType::Normal, Some(BuiltinType::Flying)), from = Species::Doduo)]
  Dodrio = 85,
  #[assoc(name = "Seel", ty = (BuiltinType::Water, None), to = &[Species::Dewgong])]
  Seel = 86,
  #[assoc(name = "Dewgong", ty = (BuiltinType::Water, Some(BuiltinType::Ice)), from = Species::Seel)]
  Dewgong = 87,
  #[assoc(name = "Grimer", ty = (BuiltinType::Poison, None), to = &[Species::Muk])]
  Grimer = 88,
  #[assoc(name = "Muk", ty = (BuiltinType::Poison, None), from = Species::Grimer)]
  Muk = 89,
  #[assoc(name = "Shellder", ty = (BuiltinType::Water, None), to = &[Species::Cloyster])]
  Shellder = 90,
  #[assoc(name = "Cloyster", ty = (BuiltinType::Water, Some(BuiltinType::Ice)), from = Species::Shellder)]
  Cloyster = 91,
  #[assoc(name = "Gastly", ty = (BuiltinType::Ghost, Some(BuiltinType::Poison)), to = &[Species::Haunter])]
  Gastly = 92,
  #[assoc(name = "Haunter", ty = (BuiltinType::Ghost, Some(BuiltinType::Poison)), from = Species::Gastly, to = &[Species::Gengar])]
  Haunter = 93,
  #[assoc(name = "Gengar", ty = (BuiltinType::Ghost, Some(BuiltinType::Poison)), from = Species::Haunter)]
  Gengar = 94,
  #[assoc(name = "Onix", ty = (BuiltinType::Rock, Some(BuiltinType::Ground)), to = &[Species::Steelix])]
  Onix = 95,
  #[assoc(name = "Drowzee", ty = (BuiltinType::Psychic, None), to = &[Species::Hypno])]
  Drowzee = 96,
  #[assoc(name = "Hypno", ty = (BuiltinType::Psychic, None), from = Species::Drowzee)]
  Hypno = 97,
  #[assoc(name = "Krabby", ty = (BuiltinType::Water, None), to = &[Species::Kingler])]
  Krabby = 98,
  #[assoc(name = "Kingler", ty = (BuiltinType::Water, None), from = Species::Krabby)]
  Kingler = 99,
  #[assoc(name = "Voltorb", ty = (BuiltinType::Electric, None), to = &[Species::Electrode])]
  Voltorb = 100,
  #[assoc(name = "Electrode", ty = (BuiltinType::Electric, None), from = Species::Voltorb)]
  Electrode = 101,
  #[assoc(name = "Exeggcute", ty = (BuiltinType::Grass, Some(BuiltinType::Psychic)), to = &[Species::Exeggutor])]
  Exeggcute = 102,
  #[assoc(name = "Exeggutor", ty = (BuiltinType::Grass, Some(BuiltinType::Psychic)), from = Species::Exeggcute)]
  Exeggutor = 103,
  #[assoc(name = "Cubone", ty = (BuiltinType::Ground, None), to = &[Species::Marowak])]
  Cubone = 104,
  #[assoc(name = "Marowak", ty = (BuiltinType::Ground, None), from = Species::Cubone)]
  Marowak = 105,
  #[assoc(name = "Hitmonlee", ty = (BuiltinType::Fighting, None), from = Species::Tyrogue)]
  Hitmonlee = 106,
  #[assoc(name = "Hitmonchan", ty = (BuiltinType::Fighting, None), from = Species::Tyrogue)]
  Hitmonchan = 107,
  #[assoc(name = "Lickitung", ty = (BuiltinType::Normal, None), to = &[Species::Lickilicky])]
  Lickitung = 108,
  #[assoc(name = "Koffing", ty = (BuiltinType::Poison, None), to = &[Species::Weezing])]
  Koffing = 109,
  #[assoc(name = "Weezing", ty = (BuiltinType::Poison, None), from = Species::Koffing)]
  Weezing = 110,
  #[assoc(name = "Rhyhorn", ty = (BuiltinType::Ground, Some(BuiltinType::Rock)), to = &[Species::Rhydon])]
  Rhyhorn = 111,
  #[assoc(name = "Rhydon", ty = (BuiltinType::Ground, Some(BuiltinType::Rock)), from = Species::Rhyhorn, to = &[Species::Rhyperior])]
  Rhydon = 112,
  #[assoc(name = "Chansey", ty = (BuiltinType::Normal, None), from = Species::Happiny, to = &[Species::Blissey])]
  Chansey = 113,
  #[assoc(name = "Tangela", ty = (BuiltinType::Grass, None), to = &[Species::Tangrowth])]
  Tangela = 114,
  #[assoc(name = "Kangaskhan", ty = (BuiltinType::Normal, None))]
  Kangaskhan = 115,
  #[assoc(name = "Horsea", ty = (BuiltinType::Water, None), to = &[Species::Seadra])]
  Horsea = 116,
  #[assoc(name = "Seadra", ty = (BuiltinType::Water, None), from = Species::Horsea, to = &[Species::Kingdra])]
  Seadra = 117,
  #[assoc(name = "Goldeen", ty = (BuiltinType::Water, None), to = &[Species::Seaking])]
  Goldeen = 118,
  #[assoc(name = "Seaking", ty = (BuiltinType::Water, None), from = Species::Goldeen)]
  Seaking = 119,
  #[assoc(name = "Staryu", ty = (BuiltinType::Water, None), to = &[Species::Starmie])]
  Staryu = 120,
  #[assoc(name = "Starmie", ty = (BuiltinType::Water, Some(BuiltinType::Psychic)), from = Species::Staryu)]
  Starmie = 121,
  #[assoc(name = "Mr. Mime", ty = (BuiltinType::Psychic, Some(BuiltinType::Fairy)), from = Species::MimeJr, to = &[Species::MrRime])]
  MrMime = 122,
  #[assoc(name = "Scyther", ty = (BuiltinType::Bug, Some(BuiltinType::Flying)), to = &[Species::Scizor, Species::Kleavor])]
  Scyther = 123,
  #[assoc(name = "Jynx", ty = (BuiltinType::Ice, Some(BuiltinType::Psychic)), from = Species::Smoochum)]
  Jynx = 124,
  #[assoc(name = "Electabuzz", ty = (BuiltinType::Electric, None), from = Species::Elekid, to = &[Species::Electivire])]
  Electabuzz = 125,
  #[assoc(name = "Magmar", ty = (BuiltinType::Fire, None), from = Species::Magby, to = &[Species::Magmortar])]
  Magmar = 126,
  #[assoc(name = "Pinsir", ty = (BuiltinType::Bug, None))]
  Pinsir = 127,
  #[assoc(name = "Tauros", ty = (BuiltinType::Normal, None))]
  Tauros = 128,
  #[assoc(name = "Magikarp", ty = (BuiltinType::Water, None), to = &[Species::Gyarados])]
  Magikarp = 129,
  #[assoc(name = "Gyarados", ty = (BuiltinType::Water, Some(BuiltinType::Flying)), from = Species::Magikarp)]
  Gyarados = 130,
  #[assoc(name = "Lapras", ty = (BuiltinType::Water, Some(BuiltinType::Ice)))]
  Lapras = 131,
  #[assoc(name = "Ditto", ty = (BuiltinType::Normal, None))]
  Ditto = 132,
  #[assoc(name = "Eevee", ty = (BuiltinType::Normal, None), to = &[Species::Vaporeon, Species::Jolteon, Species::Flareon, Species::Espeon, Species::Umbreon, Species::Leafeon, Species::Glaceon, Species::Sylveon])]
  Eevee = 133,
  #[assoc(name = "Vaporeon", ty = (BuiltinType::Water, None), from = Species::Eevee)]
  Vaporeon = 134,
  #[assoc(name = "Jolteon", ty = (BuiltinType::Electric, None), from = Species::Eevee)]
  Jolteon = 135,
  #[assoc(name = "Flareon", ty = (BuiltinType::Fire, None), from = Species::Eevee)]
  Flareon = 136,
  #[assoc(name = "Porygon", ty = (BuiltinType::Normal, None), to = &[Species::Porygon2])]
  Porygon = 137,
  #[assoc(name = "Omanyte", ty = (BuiltinType::Rock, Some(BuiltinType::Water)), to = &[Species::Omastar])]
  Omanyte = 138,
  #[assoc(name = "Omastar", ty = (BuiltinType::Rock, Some(BuiltinType::Water)), from = Species::Omanyte)]
  Omastar = 139,
  #[assoc(name = "Kabuto", ty = (BuiltinType::Rock, Some(BuiltinType::Water)), to = &[Species::Kabutops])]
  Kabuto = 140,
  #[assoc(name = "Kabutops", ty = (BuiltinType::Rock, Some(BuiltinType::Water)), from = Species::Kabuto)]
  Kabutops = 141,
  #[assoc(name = "Aerodactyl", ty = (BuiltinType::Rock, Some(BuiltinType::Flying)))]
  Aerodactyl = 142,
  #[assoc(name = "Snorlax", ty = (BuiltinType::Normal, None), from = Species::Munchlax)]
  Snorlax = 143,
  #[assoc(name = "Articuno", ty = (BuiltinType::Ice, Some(BuiltinType::Flying)))]
  Articuno = 144,
  #[assoc(name = "Zapdos", ty = (BuiltinType::Electric, Some(BuiltinType::Flying)))]
  Zapdos = 145,
  #[assoc(name = "Moltres", ty = (BuiltinType::Fire, Some(BuiltinType::Flying)))]
  Moltres = 146,
  #[assoc(name = "Dratini", ty = (BuiltinType::Dragon, None), to = &[Species::Dragonair])]
  Dratini = 147,
  #[assoc(name = "Dragonair", ty = (BuiltinType::Dragon, None), from = Species::Dratini, to = &[Species::Dragonite])]
  Dragonair = 148,
  #[assoc(name = "Dragonite", ty = (BuiltinType::Dragon, Some(BuiltinType::Flying)), from = Species::Dragonair)]
  Dragonite = 149,
  #[assoc(name = "Mewtwo", ty = (BuiltinType::Psychic, None))]
  Mewtwo = 150,
  #[assoc(name = "Mew", ty = (BuiltinType::Psychic, None))]
  Mew = 151,
  #[assoc(name = "Chikorita", ty = (BuiltinType::Grass, None), to = &[Species::Bayleef])]
  Chikorita = 152,
  #[assoc(name = "Bayleef", ty = (BuiltinType::Grass, None), from = Species::Chikorita, to = &[Species::Meganium])]
  Bayleef = 153,
  #[assoc(name = "Meganium", ty = (BuiltinType::Grass, None), from = Species::Bayleef)]
  Meganium = 154,
  #[assoc(name = "Cyndaquil", ty = (BuiltinType::Fire, None), to = &[Species::Quilava])]
  Cyndaquil = 155,
  #[assoc(name = "Quilava", ty = (BuiltinType::Fire, None), from = Species::Cyndaquil, to = &[Species::Typhlosion])]
  Quilava = 156,
  #[assoc(name = "Typhlosion", ty = (BuiltinType::Fire, None), from = Species::Quilava)]
  Typhlosion = 157,
  #[assoc(name = "Totodile", ty = (BuiltinType::Water, None), to = &[Species::Croconaw])]
  Totodile = 158,
  #[assoc(name = "Croconaw", ty = (BuiltinType::Water, None), from = Species::Totodile, to = &[Species::Feraligatr])]
  Croconaw = 159,
  #[assoc(name = "Feraligatr", ty = (BuiltinType::Water, None), from = Species::Croconaw)]
  Feraligatr = 160,
  #[assoc(name = "Sentret", ty = (BuiltinType::Normal, None), to = &[Species::Furret])]
  Sentret = 161,
  #[assoc(name = "Furret", ty = (BuiltinType::Normal, None), from = Species::Sentret)]
  Furret = 162,
  #[assoc(name = "Hoothoot", ty = (BuiltinType::Normal, Some(BuiltinType::Flying)), to = &[Species::Noctowl])]
  Hoothoot = 163,
  #[assoc(name = "Noctowl", ty = (BuiltinType::Normal, Some(BuiltinType::Flying)), from = Species::Hoothoot)]
  Noctowl = 164,
  #[assoc(name = "Ledyba", ty = (BuiltinType::Bug, Some(BuiltinType::Flying)), to = &[Species::Ledian])]
  Ledyba = 165,
  #[assoc(name = "Ledian", ty = (BuiltinType::Bug, Some(BuiltinType::Flying)), from = Species::Ledyba)]
  Ledian = 166,
  #[assoc(name = "Spinarak", ty = (BuiltinType::Bug, Some(BuiltinType::Poison)), to = &[Species::Ariados])]
  Spinarak = 167,
  #[assoc(name = "Ariados", ty = (BuiltinType::Bug, Some(BuiltinType::Poison)), from = Species::Spinarak)]
  Ariados = 168,
  #[assoc(name = "Crobat", ty = (BuiltinType::Poison, Some(BuiltinType::Flying)), from = Species::Golbat)]
  Crobat = 169,
  #[assoc(name = "Chinchou", ty = (BuiltinType::Water, Some(BuiltinType::Electric)), to = &[Species::Lanturn])]
  Chinchou = 170,
  #[assoc(name = "Lanturn", ty = (BuiltinType::Water, Some(BuiltinType::Electric)), from = Species::Chinchou)]
  Lanturn = 171,
  #[assoc(name = "Pichu", ty = (BuiltinType::Electric, None), to = &[Species::Pikachu])]
  Pichu = 172,
  #[assoc(name = "Cleffa", ty = (BuiltinType::Fairy, None), to = &[Species::Clefairy])]
  Cleffa = 173,
  #[assoc(name = "Igglybuff", ty = (BuiltinType::Normal, Some(BuiltinType::Fairy)), to = &[Species::Jigglypuff])]
  Igglybuff = 174,
  #[assoc(name = "Togepi", ty = (BuiltinType::Fairy, None), to = &[Species::Togetic])]
  Togepi = 175,
  #[assoc(name = "Togetic", ty = (BuiltinType::Fairy, Some(BuiltinType::Flying)), from = Species::Togepi, to = &[Species::Togekiss])]
  Togetic = 176,
  #[assoc(name = "Natu", ty = (BuiltinType::Psychic, Some(BuiltinType::Flying)), to = &[Species::Xatu])]
  Natu = 177,
  #[assoc(name = "Xatu", ty = (BuiltinType::Psychic, Some(BuiltinType::Flying)), from = Species::Natu)]
  Xatu = 178,
  #[assoc(name = "Mareep", ty = (BuiltinType::Electric, None), to = &[Species::Flaaffy])]
  Mareep = 179,
  #[assoc(name = "Flaaffy", ty = (BuiltinType::Electric, None), from = Species::Mareep, to = &[Species::Ampharos])]
  Flaaffy = 180,
  #[assoc(name = "Ampharos", ty = (BuiltinType::Electric, None), from = Species::Flaaffy)]
  Ampharos = 181,
  #[assoc(name = "Bellossom", ty = (BuiltinType::Grass, None), from = Species::Gloom)]
  Bellossom = 182,
  #[assoc(name = "Marill", ty = (BuiltinType::Water, Some(BuiltinType::Fairy)), from = Species::Azurill, to = &[Species::Azumarill])]
  Marill = 183,
  #[assoc(name = "Azumarill", ty = (BuiltinType::Water, Some(BuiltinType::Fairy)), from = Species::Marill)]
  Azumarill = 184,
  #[assoc(name = "Sudowoodo", ty = (BuiltinType::Rock, None), from = Species::Bonsly)]
  Sudowoodo = 185,
  #[assoc(name = "Politoed", ty = (BuiltinType::Water, None), from = Species::Poliwhirl)]
  Politoed = 186,
  #[assoc(name = "Hoppip", ty = (BuiltinType::Grass, Some(BuiltinType::Flying)), to = &[Species::Skiploom])]
  Hoppip = 187,
  #[assoc(name = "Skiploom", ty = (BuiltinType::Grass, Some(BuiltinType::Flying)), from = Species::Hoppip, to = &[Species::Jumpluff])]
  Skiploom = 188,
  #[assoc(name = "Jumpluff", ty = (BuiltinType::Grass, Some(BuiltinType::Flying)), from = Species::Skiploom)]
  Jumpluff = 189,
  #[assoc(name = "Aipom", ty = (BuiltinType::Normal, None), to = &[Species::Ambipom])]
  Aipom = 190,
  #[assoc(name = "Sunkern", ty = (BuiltinType::Grass, None), to = &[Species::Sunflora])]
  Sunkern = 191,
  #[assoc(name = "Sunflora", ty = (BuiltinType::Grass, None), from = Species::Sunkern)]
  Sunflora = 192,
  #[assoc(name = "Yanma", ty = (BuiltinType::Bug, Some(BuiltinType::Flying)), to = &[Species::Yanmega])]
  Yanma = 193,
  #[assoc(name = "Wooper", ty = (BuiltinType::Water, Some(BuiltinType::Ground)), to = &[Species::Quagsire, Species::Clodsire])]
  Wooper = 194,
  #[assoc(name = "Quagsire", ty = (BuiltinType::Water, Some(BuiltinType::Ground)), from = Species::Wooper)]
  Quagsire = 195,
  #[assoc(name = "Espeon", ty = (BuiltinType::Psychic, None), from = Species::Eevee)]
  Espeon = 196,
  #[assoc(name = "Umbreon", ty = (BuiltinType::Dark, None), from = Species::Eevee)]
  Umbreon = 197,
  #[assoc(name = "Murkrow", ty = (BuiltinType::Dark, Some(BuiltinType::Flying)), to = &[Species::Honchkrow])]
  Murkrow = 198,
  #[assoc(name = "Slowking", ty = (BuiltinType::Water, Some(BuiltinType::Psychic)), from = Species::Slowpoke)]
  Slowking = 199,
  #[assoc(name = "Misdreavus", ty = (BuiltinType::Ghost, None), to = &[Species::Mismagius])]
  Misdreavus = 200,
  #[assoc(name = "Unown", ty = (BuiltinType::Psychic, None))]
  Unown = 201,
  #[assoc(name = "Wobbuffet", ty = (BuiltinType::Psychic, None), from = Species::Wynaut)]
  Wobbuffet = 202,
  #[assoc(name = "Girafarig", ty = (BuiltinType::Normal, Some(BuiltinType::Psychic)), to = &[Species::Farigiraf])]
  Girafarig = 203,
  #[assoc(name = "Pineco", ty = (BuiltinType::Bug, None), to = &[Species::Forretress])]
  Pineco = 204,
  #[assoc(name = "Forretress", ty = (BuiltinType::Bug, Some(BuiltinType::Steel)), from = Species::Pineco)]
  Forretress = 205,
  #[assoc(name = "Dunsparce", ty = (BuiltinType::Normal, None), to = &[Species::Dudunsparce])]
  Dunsparce = 206,
  #[assoc(name = "Gligar", ty = (BuiltinType::Ground, Some(BuiltinType::Flying)), to = &[Species::Gliscor])]
  Gligar = 207,
  #[assoc(name = "Steelix", ty = (BuiltinType::Steel, Some(BuiltinType::Ground)), from = Species::Onix)]
  Steelix = 208,
  #[assoc(name = "Snubbull", ty = (BuiltinType::Fairy, None), to = &[Species::Granbull])]
  Snubbull = 209,
  #[assoc(name = "Granbull", ty = (BuiltinType::Fairy, None), from = Species::Snubbull)]
  Granbull = 210,
  #[assoc(name = "Qwilfish", ty = (BuiltinType::Water, Some(BuiltinType::Poison)), to = &[Species::Overqwil])]
  Qwilfish = 211,
  #[assoc(name = "Scizor", ty = (BuiltinType::Bug, Some(BuiltinType::Steel)), from = Species::Scyther)]
  Scizor = 212,
  #[assoc(name = "Shuckle", ty = (BuiltinType::Bug, Some(BuiltinType::Rock)))]
  Shuckle = 213,
  #[assoc(name = "Heracross", ty = (BuiltinType::Bug, Some(BuiltinType::Fighting)))]
  Heracross = 214,
  #[assoc(name = "Sneasel", ty = (BuiltinType::Dark, Some(BuiltinType::Ice)), to = &[Species::Weavile, Species::Sneasler])]
  Sneasel = 215,
  #[assoc(name = "Teddiursa", ty = (BuiltinType::Normal, None), to = &[Species::Ursaring])]
  Teddiursa = 216,
  #[assoc(name = "Ursaring", ty = (BuiltinType::Normal, None), from = Species::Teddiursa, to = &[Species::Ursaluna])]
  Ursaring = 217,
  #[assoc(name = "Slugma", ty = (BuiltinType::Fire, None), to = &[Species::Magcargo])]
  Slugma = 218,
  #[assoc(name = "Magcargo", ty = (BuiltinType::Fire, Some(BuiltinType::Rock)), from = Species::Slugma)]
  Magcargo = 219,
  #[assoc(name = "Swinub", ty = (BuiltinType::Ice, Some(BuiltinType::Ground)), to = &[Species::Piloswine])]
  Swinub = 220,
  #[assoc(name = "Piloswine", ty = (BuiltinType::Ice, Some(BuiltinType::Ground)), from = Species::Swinub, to = &[Species::Mamoswine])]
  Piloswine = 221,
  #[assoc(name = "Corsola", ty = (BuiltinType::Water, Some(BuiltinType::Rock)), to = &[Species::Cursola])]
  Corsola = 222,
  #[assoc(name = "Remoraid", ty = (BuiltinType::Water, None), to = &[Species::Octillery])]
  Remoraid = 223,
  #[assoc(name = "Octillery", ty = (BuiltinType::Water, None), from = Species::Remoraid)]
  Octillery = 224,
  #[assoc(name = "Delibird", ty = (BuiltinType::Ice, Some(BuiltinType::Flying)))]
  Delibird = 225,
  #[assoc(name = "Mantine", ty = (BuiltinType::Water, Some(BuiltinType::Flying)), from = Species::Mantyke)]
  Mantine = 226,
  #[assoc(name = "Skarmory", ty = (BuiltinType::Steel, Some(BuiltinType::Flying)))]
  Skarmory = 227,
  #[assoc(name = "Houndour", ty = (BuiltinType::Dark, Some(BuiltinType::Fire)), to = &[Species::Houndoom])]
  Houndour = 228,
  #[assoc(name = "Houndoom", ty = (BuiltinType::Dark, Some(BuiltinType::Fire)), from = Species::Houndour)]
  Houndoom = 229,
  #[assoc(name = "Kingdra", ty = (BuiltinType::Water, Some(BuiltinType::Dragon)), from = Species::Seadra)]
  Kingdra = 230,
  #[assoc(name = "Phanpy", ty = (BuiltinType::Ground, None), to = &[Species::Donphan])]
  Phanpy = 231,
  #[assoc(name = "Donphan", ty = (BuiltinType::Ground, None), from = Species::Phanpy)]
  Donphan = 232,
  #[assoc(name = "Porygon2", ty = (BuiltinType::Normal, None), from = Species::Porygon, to = &[Species::PorygonZ])]
  Porygon2 = 233,
  #[assoc(name = "Stantler", ty = (BuiltinType::Normal, None), to = &[Species::Wyrdeer])]
  Stantler = 234,
  #[assoc(name = "Smeargle", ty = (BuiltinType::Normal, None))]
  Smeargle = 235,
  #[assoc(name = "Tyrogue", ty = (BuiltinType::Fighting, None), to = &[Species::Hitmonlee, Species::Hitmonchan, Species::Hitmontop])]
  Tyrogue = 236,
  #[assoc(name = "Hitmontop", ty = (BuiltinType::Fighting, None), from = Species::Tyrogue)]
  Hitmontop = 237,
  #[assoc(name = "Smoochum", ty = (BuiltinType::Ice, Some(BuiltinType::Psychic)), to = &[Species::Jynx])]
  Smoochum = 238,
  #[assoc(name = "Elekid", ty = (BuiltinType::Electric, None), to = &[Species::Electabuzz])]
  Elekid = 239,
  #[assoc(name = "Magby", ty = (BuiltinType::Fire, None), to = &[Species::Magmar])]
  Magby = 240,
  #[assoc(name = "Miltank", ty = (BuiltinType::Normal, None))]
  Miltank = 241,
  #[assoc(name = "Blissey", ty = (BuiltinType::Normal, None), from = Species::Chansey)]
  Blissey = 242,
  #[assoc(name = "Raikou", ty = (BuiltinType::Electric, None))]
  Raikou = 243,
  #[assoc(name = "Entei", ty = (BuiltinType::Fire, None))]
  Entei = 244,
  #[assoc(name = "Suicune", ty = (BuiltinType::Water, None))]
  Suicune = 245,
  #[assoc(name = "Larvitar", ty = (BuiltinType::Rock, Some(BuiltinType::Ground)), to = &[Species::Pupitar])]
  Larvitar = 246,
  #[assoc(name = "Pupitar", ty = (BuiltinType::Rock, Some(BuiltinType::Ground)), from = Species::Larvitar, to = &[Species::Tyranitar])]
  Pupitar = 247,
  #[assoc(name = "Tyranitar", ty = (BuiltinType::Rock, Some(BuiltinType::Dark)), from = Species::Pupitar)]
  Tyranitar = 248,
  #[assoc(name = "Lugia", ty = (BuiltinType::Psychic, Some(BuiltinType::Flying)))]
  Lugia = 249,
  #[assoc(name = "Ho-Oh", ty = (BuiltinType::Fire, Some(BuiltinType::Flying)))]
  HoOh = 250,
  #[assoc(name = "Celebi", ty = (BuiltinType::Psychic, Some(BuiltinType::Grass)))]
  Celebi = 251,
  #[assoc(name = "Treecko", ty = (BuiltinType::Grass, None), to = &[Species::Grovyle])]
  Treecko = 252,
  #[assoc(name = "Grovyle", ty = (BuiltinType::Grass, None), from = Species::Treecko, to = &[Species::Sceptile])]
  Grovyle = 253,
  #[assoc(name = "Sceptile", ty = (BuiltinType::Grass, None), from = Species::Grovyle)]
  Sceptile = 254,
  #[assoc(name = "Torchic", ty = (BuiltinType::Fire, None), to = &[Species::Combusken])]
  Torchic = 255,
  #[assoc(name = "Combusken", ty = (BuiltinType::Fire, Some(BuiltinType::Fighting)), from = Species::Torchic, to = &[Species::Blaziken])]
  Combusken = 256,
  #[assoc(name = "Blaziken", ty = (BuiltinType::Fire, Some(BuiltinType::Fighting)), from = Species::Combusken)]
  Blaziken = 257,
  #[assoc(name = "Mudkip", ty = (BuiltinType::Water, None), to = &[Species::Marshtomp])]
  Mudkip = 258,
  #[assoc(name = "Marshtomp", ty = (BuiltinType::Water, Some(BuiltinType::Ground)), from = Species::Mudkip, to = &[Species::Swampert])]
  Marshtomp = 259,
  #[assoc(name = "Swampert", ty = (BuiltinType::Water, Some(BuiltinType::Ground)), from = Species::Marshtomp)]
  Swampert = 260,
  #[assoc(name = "Poochyena", ty = (BuiltinType::Dark, None), to = &[Species::Mightyena])]
  Poochyena = 261,
  #[assoc(name = "Mightyena", ty = (BuiltinType::Dark, None), from = Species::Poochyena)]
  Mightyena = 262,
  #[assoc(name = "Zigzagoon", ty = (BuiltinType::Normal, None), to = &[Species::Linoone])]
  Zigzagoon = 263,
  #[assoc(name = "Linoone", ty = (BuiltinType::Normal, None), from = Species::Zigzagoon, to = &[Species::Obstagoon])]
  Linoone = 264,
  #[assoc(name = "Wurmple", ty = (BuiltinType::Bug, None), to = &[Species::Silcoon, Species::Cascoon])]
  Wurmple = 265,
  #[assoc(name = "Silcoon", ty = (BuiltinType::Bug, None), from = Species::Wurmple, to = &[Species::Beautifly])]
  Silcoon = 266,
  #[assoc(name = "Beautifly", ty = (BuiltinType::Bug, Some(BuiltinType::Flying)), from = Species::Silcoon)]
  Beautifly = 267,
  #[assoc(name = "Cascoon", ty = (BuiltinType::Bug, None), from = Species::Wurmple, to = &[Species::Dustox])]
  Cascoon = 268,
  #[assoc(name = "Dustox", ty = (BuiltinType::Bug, Some(BuiltinType::Poison)), from = Species::Cascoon)]
  Dustox = 269,
  #[assoc(name = "Lotad", ty = (BuiltinType::Water, Some(BuiltinType::Grass)), to = &[Species::Lombre])]
  Lotad = 270,
  #[assoc(name = "Lombre", ty = (BuiltinType::Water, Some(BuiltinType::Grass)), from = Species::Lotad, to = &[Species::Ludicolo])]
  Lombre = 271,
  #[assoc(name = "Ludicolo", ty = (BuiltinType::Water, Some(BuiltinType::Grass)), from = Species::Lombre)]
  Ludicolo = 272,
  #[assoc(name = "Seedot", ty = (BuiltinType::Grass, None), to = &[Species::Nuzleaf])]
  Seedot = 273,
  #[assoc(name = "Nuzleaf", ty = (BuiltinType::Grass, Some(BuiltinType::Dark)), from = Species::Seedot, to = &[Species::Shiftry])]
  Nuzleaf = 274,
  #[assoc(name = "Shiftry", ty = (BuiltinType::Grass, Some(BuiltinType::Dark)), from = Species::Nuzleaf)]
  Shiftry = 275,
  #[assoc(name = "Taillow", ty = (BuiltinType::Normal, Some(BuiltinType::Flying)), to = &[Species::Swellow])]
  Taillow = 276,
  #[assoc(name = "Swellow", ty = (BuiltinType::Normal, Some(BuiltinType::Flying)), from = Species::Taillow)]
  Swellow = 277,
  #[assoc(name = "Wingull", ty = (BuiltinType::Water, Some(BuiltinType::Flying)), to = &[Species::Pelipper])]
  Wingull = 278,
  #[assoc(name = "Pelipper", ty = (BuiltinType::Water, Some(BuiltinType::Flying)), from = Species::Wingull)]
  Pelipper = 279,
  #[assoc(name = "Ralts", ty = (BuiltinType::Psychic, Some(BuiltinType::Fairy)), to = &[Species::Kirlia])]
  Ralts = 280,
  #[assoc(name = "Kirlia", ty = (BuiltinType::Psychic, Some(BuiltinType::Fairy)), from = Species::Ralts, to = &[Species::Gardevoir, Species::Gallade])]
  Kirlia = 281,
  #[assoc(name = "Gardevoir", ty = (BuiltinType::Psychic, Some(BuiltinType::Fairy)), from = Species::Kirlia)]
  Gardevoir = 282,
  #[assoc(name = "Surskit", ty = (BuiltinType::Bug, Some(BuiltinType::Water)), to = &[Species::Masquerain])]
  Surskit = 283,
  #[assoc(name = "Masquerain", ty = (BuiltinType::Bug, Some(BuiltinType::Flying)), from = Species::Surskit)]
  Masquerain = 284,
  #[assoc(name = "Shroomish", ty = (BuiltinType::Grass, None), to = &[Species::Breloom])]
  Shroomish = 285,
  #[assoc(name = "Breloom", ty = (BuiltinType::Grass, Some(BuiltinType::Fighting)), from = Species::Shroomish)]
  Breloom = 286,
  #[assoc(name = "Slakoth", ty = (BuiltinType::Normal, None), to = &[Species::Vigoroth])]
  Slakoth = 287,
  #[assoc(name = "Vigoroth", ty = (BuiltinType::Normal, None), from = Species::Slakoth, to = &[Species::Slaking])]
  Vigoroth = 288,
  #[assoc(name = "Slaking", ty = (BuiltinType::Normal, None), from = Species::Vigoroth)]
  Slaking = 289,
  #[assoc(name = "Nincada", ty = (BuiltinType::Bug, Some(BuiltinType::Ground)), to = &[Species::Ninjask, Species::Shedinja])]
  Nincada = 290,
  #[assoc(name = "Ninjask", ty = (BuiltinType::Bug, Some(BuiltinType::Flying)), from = Species::Nincada)]
  Ninjask = 291,
  #[assoc(name = "Shedinja", ty = (BuiltinType::Bug, Some(BuiltinType::Ghost)), from = Species::Nincada)]
  Shedinja = 292,
  #[assoc(name = "Whismur", ty = (BuiltinType::Normal, None), to = &[Species::Loudred])]
  Whismur = 293,
  #[assoc(name = "Loudred", ty = (BuiltinType::Normal, None), from = Species::Whismur, to = &[Species::Exploud])]
  Loudred = 294,
  #[assoc(name = "Exploud", ty = (BuiltinType::Normal, None), from = Species::Loudred)]
  Exploud = 295,
  #[assoc(name = "Makuhita", ty = (BuiltinType::Fighting, None), to = &[Species::Hariyama])]
  Makuhita = 296,
  #[assoc(name = "Hariyama", ty = (BuiltinType::Fighting, None), from = Species::Makuhita)]
  Hariyama = 297,
  #[assoc(name = "Azurill", ty = (BuiltinType::Normal, Some(BuiltinType::Fairy)), to = &[Species::Marill])]
  Azurill = 298,
  #[assoc(name = "Nosepass", ty = (BuiltinType::Rock, None), to = &[Species::Probopass])]
  Nosepass = 299,
  #[assoc(name = "Skitty", ty = (BuiltinType::Normal, None), to = &[Species::Delcatty])]
  Skitty = 300,
  #[assoc(name = "Delcatty", ty = (BuiltinType::Normal, None), from = Species::Skitty)]
  Delcatty = 301,
  #[assoc(name = "Sableye", ty = (BuiltinType::Dark, Some(BuiltinType::Ghost)))]
  Sableye = 302,
  #[assoc(name = "Mawile", ty = (BuiltinType::Steel, Some(BuiltinType::Fairy)))]
  Mawile = 303,
  #[assoc(name = "Aron", ty = (BuiltinType::Steel, Some(BuiltinType::Rock)), to = &[Species::Lairon])]
  Aron = 304,
  #[assoc(name = "Lairon", ty = (BuiltinType::Steel, Some(BuiltinType::Rock)), from = Species::Aron, to = &[Species::Aggron])]
  Lairon = 305,
  #[assoc(name = "Aggron", ty = (BuiltinType::Steel, Some(BuiltinType::Rock)), from = Species::Lairon)]
  Aggron = 306,
  #[assoc(name = "Meditite", ty = (BuiltinType::Fighting, Some(BuiltinType::Psychic)), to = &[Species::Medicham])]
  Meditite = 307,
  #[assoc(name = "Medicham", ty = (BuiltinType::Fighting, Some(BuiltinType::Psychic)), from = Species::Meditite)]
  Medicham = 308,
  #[assoc(name = "Electrike", ty = (BuiltinType::Electric, None), to = &[Species::Manectric])]
  Electrike = 309,
  #[assoc(name = "Manectric", ty = (BuiltinType::Electric, None), from = Species::Electrike)]
  Manectric = 310,
  #[assoc(name = "Plusle", ty = (BuiltinType::Electric, None))]
  Plusle = 311,
  #[assoc(name = "Minun", ty = (BuiltinType::Electric, None))]
  Minun = 312,
  #[assoc(name = "Volbeat", ty = (BuiltinType::Bug, None))]
  Volbeat = 313,
  #[assoc(name = "Illumise", ty = (BuiltinType::Bug, None))]
  Illumise = 314,
  #[assoc(name = "Roselia", ty = (BuiltinType::Grass, Some(BuiltinType::Poison)), from = Species::Budew, to = &[Species::Roserade])]
  Roselia = 315,
  #[assoc(name = "Gulpin", ty = (BuiltinType::Poison, None), to = &[Species::Swalot])]
  Gulpin = 316,
  #[assoc(name = "Swalot", ty = (BuiltinType::Poison, None), from = Species::Gulpin)]
  Swalot = 317,
  #[assoc(name = "Carvanha", ty = (BuiltinType::Water, Some(BuiltinType::Dark)), to = &[Species::Sharpedo])]
  Carvanha = 318,
  #[assoc(name = "Sharpedo", ty = (BuiltinType::Water, Some(BuiltinType::Dark)), from = Species::Carvanha)]
  Sharpedo = 319,
  #[assoc(name = "Wailmer", ty = (BuiltinType::Water, None), to = &[Species::Wailord])]
  Wailmer = 320,
  #[assoc(name = "Wailord", ty = (BuiltinType::Water, None), from = Species::Wailmer)]
  Wailord = 321,
  #[assoc(name = "Numel", ty = (BuiltinType::Fire, Some(BuiltinType::Ground)), to = &[Species::Camerupt])]
  Numel = 322,
  #[assoc(name = "Camerupt", ty = (BuiltinType::Fire, Some(BuiltinType::Ground)), from = Species::Numel)]
  Camerupt = 323,
  #[assoc(name = "Torkoal", ty = (BuiltinType::Fire, None))]
  Torkoal = 324,
  #[assoc(name = "Spoink", ty = (BuiltinType::Psychic, None), to = &[Species::Grumpig])]
  Spoink = 325,
  #[assoc(name = "Grumpig", ty = (BuiltinType::Psychic, None), from = Species::Spoink)]
  Grumpig = 326,
  #[assoc(name = "Spinda", ty = (BuiltinType::Normal, None))]
  Spinda = 327,
  #[assoc(name = "Trapinch", ty = (BuiltinType::Ground, None), to = &[Species::Vibrava])]
  Trapinch = 328,
  #[assoc(name = "Vibrava", ty = (BuiltinType::Ground, Some(BuiltinType::Dragon)), from = Species::Trapinch, to = &[Species::Flygon])]
  Vibrava = 329,
  #[assoc(name = "Flygon", ty = (BuiltinType::Ground, Some(BuiltinType::Dragon)), from = Species::Vibrava)]
  Flygon = 330,
  #[assoc(name = "Cacnea", ty = (BuiltinType::Grass, None), to = &[Species::Cacturne])]
  Cacnea = 331,
  #[assoc(name = "Cacturne", ty = (BuiltinType::Grass, Some(BuiltinType::Dark)), from = Species::Cacnea)]
  Cacturne = 332,
  #[assoc(name = "Swablu", ty = (BuiltinType::Normal, Some(BuiltinType::Flying)), to = &[Species::Altaria])]
  Swablu = 333,
  #[assoc(name = "Altaria", ty = (BuiltinType::Dragon, Some(BuiltinType::Flying)), from = Species::Swablu)]
  Altaria = 334,
  #[assoc(name = "Zangoose", ty = (BuiltinType::Normal, None))]
  Zangoose = 335,
  #[assoc(name = "Seviper", ty = (BuiltinType::Poison, None))]
  Seviper = 336,
  #[assoc(name = "Lunatone", ty = (BuiltinType::Rock, Some(BuiltinType::Psychic)))]
  Lunatone = 337,
  #[assoc(name = "Solrock", ty = (BuiltinType::Rock, Some(BuiltinType::Psychic)))]
  Solrock = 338,
  #[assoc(name = "Barboach", ty = (BuiltinType::Water, Some(BuiltinType::Ground)), to = &[Species::Whiscash])]
  Barboach = 339,
  #[assoc(name = "Whiscash", ty = (BuiltinType::Water, Some(BuiltinType::Ground)), from = Species::Barboach)]
  Whiscash = 340,
  #[assoc(name = "Corphish", ty = (BuiltinType::Water, None), to = &[Species::Crawdaunt])]
  Corphish = 341,
  #[assoc(name = "Crawdaunt", ty = (BuiltinType::Water, Some(BuiltinType::Dark)), from = Species::Corphish)]
  Crawdaunt = 342,
  #[assoc(name = "Baltoy", ty = (BuiltinType::Ground, Some(BuiltinType::Psychic)), to = &[Species::Claydol])]
  Baltoy = 343,
  #[assoc(name = "Claydol", ty = (BuiltinType::Ground, Some(BuiltinType::Psychic)), from = Species::Baltoy)]
  Claydol = 344,
  #[assoc(name = "Lileep", ty = (BuiltinType::Rock, Some(BuiltinType::Grass)), to = &[Species::Cradily])]
  Lileep = 345,
  #[assoc(name = "Cradily", ty = (BuiltinType::Rock, Some(BuiltinType::Grass)), from = Species::Lileep)]
  Cradily = 346,
  #[assoc(name = "Anorith", ty = (BuiltinType::Rock, Some(BuiltinType::Bug)), to = &[Species::Armaldo])]
  Anorith = 347,
  #[assoc(name = "Armaldo", ty = (BuiltinType::Rock, Some(BuiltinType::Bug)), from = Species::Anorith)]
  Armaldo = 348,
  #[assoc(name = "Feebas", ty = (BuiltinType::Water, None), to = &[Species::Milotic])]
  Feebas = 349,
  #[assoc(name = "Milotic", ty = (BuiltinType::Water, None), from = Species::Feebas)]
  Milotic = 350,
  #[assoc(name = "Castform", ty = (BuiltinType::Normal, None))]
  Castform = 351,
  #[assoc(name = "Kecleon", ty = (BuiltinType::Normal, None))]
  Kecleon = 352,
  #[assoc(name = "Shuppet", ty = (BuiltinType::Ghost, None), to = &[Species::Banette])]
  Shuppet = 353,
  #[assoc(name = "Banette", ty = (BuiltinType::Ghost, None), from = Species::Shuppet)]
  Banette = 354,
  #[assoc(name = "Duskull", ty = (BuiltinType::Ghost, None), to = &[Species::Dusclops])]
  Duskull = 355,
  #[assoc(name = "Dusclops", ty = (BuiltinType::Ghost, None), from = Species::Duskull, to = &[Species::Dusknoir])]
  Dusclops = 356,
  #[assoc(name = "Tropius", ty = (BuiltinType::Grass, Some(BuiltinType::Flying)))]
  Tropius = 357,
  #[assoc(name = "Chimecho", ty = (BuiltinType::Psychic, None), from = Species::Chingling)]
  Chimecho = 358,
  #[assoc(name = "Absol", ty = (BuiltinType::Dark, None))]
  Absol = 359,
  #[assoc(name = "Wynaut", ty = (BuiltinType::Psychic, None), to = &[Species::Wobbuffet])]
  Wynaut = 360,
  #[assoc(name = "Snorunt", ty = (BuiltinType::Ice, None), to = &[Species::Glalie, Species::Froslass])]
  Snorunt = 361,
  #[assoc(name = "Glalie", ty = (BuiltinType::Ice, None), from = Species::Snorunt)]
  Glalie = 362,
  #[assoc(name = "Spheal", ty = (BuiltinType::Ice, Some(BuiltinType::Water)), to = &[Species::Sealeo])]
  Spheal = 363,
  #[assoc(name = "Sealeo", ty = (BuiltinType::Ice, Some(BuiltinType::Water)), from = Species::Spheal, to = &[Species::Walrein])]
  Sealeo = 364,
  #[assoc(name = "Walrein", ty = (BuiltinType::Ice, Some(BuiltinType::Water)), from = Species::Sealeo)]
  Walrein = 365,
  #[assoc(name = "Clamperl", ty = (BuiltinType::Water, None), to = &[Species::Huntail, Species::Gorebyss])]
  Clamperl = 366,
  #[assoc(name = "Huntail", ty = (BuiltinType::Water, None), from = Species::Clamperl)]
  Huntail = 367,
  #[assoc(name = "Gorebyss", ty = (BuiltinType::Water, None), from = Species::Clamperl)]
  Gorebyss = 368,
  #[assoc(name = "Relicanth", ty = (BuiltinType::Water, Some(BuiltinType::Rock)))]
  Relicanth = 369,
  #[assoc(name = "Luvdisc", ty = (BuiltinType::Water, None))]
  Luvdisc = 370,
  #[assoc(name = "Bagon", ty = (BuiltinType::Dragon, None), to = &[Species::Shelgon])]
  Bagon = 371,
  #[assoc(name = "Shelgon", ty = (BuiltinType::Dragon, None), from = Species::Bagon, to = &[Species::Salamence])]
  Shelgon = 372,
  #[assoc(name = "Salamence", ty = (BuiltinType::Dragon, Some(BuiltinType::Flying)), from = Species::Shelgon)]
  Salamence = 373,
  #[assoc(name = "Beldum", ty = (BuiltinType::Steel, Some(BuiltinType::Psychic)), to = &[Species::Metang])]
  Beldum = 374,
  #[assoc(name = "Metang", ty = (BuiltinType::Steel, Some(BuiltinType::Psychic)), from = Species::Beldum, to = &[Species::Metagross])]
  Metang = 375,
  #[assoc(name = "Metagross", ty = (BuiltinType::Steel, Some(BuiltinType::Psychic)), from = Species::Metang)]
  Metagross = 376,
  #[assoc(name = "Regirock", ty = (BuiltinType::Rock, None))]
  Regirock = 377,
  #[assoc(name = "Regice", ty = (BuiltinType::Ice, None))]
  Regice = 378,
  #[assoc(name = "Registeel", ty = (BuiltinType::Steel, None))]
  Registeel = 379,
  #[assoc(name = "Latias", ty = (BuiltinType::Dragon, Some(BuiltinType::Psychic)))]
  Latias = 380,
  #[assoc(name = "Latios", ty = (BuiltinType::Dragon, Some(BuiltinType::Psychic)))]
  Latios = 381,
  #[assoc(name = "Kyogre", ty = (BuiltinType::Water, None))]
  Kyogre = 382,
  #[assoc(name = "Groudon", ty = (BuiltinType::Ground, None))]
  Groudon = 383,
  #[assoc(name = "Rayquaza", ty = (BuiltinType::Dragon, Some(BuiltinType::Flying)))]
  Rayquaza = 384,
  #[assoc(name = "Jirachi", ty = (BuiltinType::Steel, Some(BuiltinType::Psychic)))]
  Jirachi = 385,
  #[assoc(name = "Deoxys", ty = (BuiltinType::Psychic, None))]
  Deoxys = 386,
  #[assoc(name = "Turtwig", ty = (BuiltinType::Grass, None), to = &[Species::Grotle])]
  Turtwig = 387,
  #[assoc(name = "Grotle", ty = (BuiltinType::Grass, None), from = Species::Turtwig, to = &[Species::Torterra])]
  Grotle = 388,
  #[assoc(name = "Torterra", ty = (BuiltinType::Grass, Some(BuiltinType::Ground)), from = Species::Grotle)]
  Torterra = 389,
  #[assoc(name = "Chimchar", ty = (BuiltinType::Fire, None), to = &[Species::Monferno])]
  Chimchar = 390,
  #[assoc(name = "Monferno", ty = (BuiltinType::Fire, Some(BuiltinType::Fighting)), from = Species::Chimchar, to = &[Species::Infernape])]
  Monferno = 391,
  #[assoc(name = "Infernape", ty = (BuiltinType::Fire, Some(BuiltinType::Fighting)), from = Species::Monferno)]
  Infernape = 392,
  #[assoc(name = "Piplup", ty = (BuiltinType::Water, None), to = &[Species::Prinplup])]
  Piplup = 393,
  #[assoc(name = "Prinplup", ty = (BuiltinType::Water, None), from = Species::Piplup, to = &[Species::Empoleon])]
  Prinplup = 394,
  #[assoc(name = "Empoleon", ty = (BuiltinType::Water, Some(BuiltinType::Steel)), from = Species::Prinplup)]
  Empoleon = 395,
  #[assoc(name = "Starly", ty = (BuiltinType::Normal, Some(BuiltinType::Flying)), to = &[Species::Staravia])]
  Starly = 396,
  #[assoc(name = "Staravia", ty = (BuiltinType::Normal, Some(BuiltinType::Flying)), from = Species::Starly, to = &[Species::Staraptor])]
  Staravia = 397,
  #[assoc(name = "Staraptor", ty = (BuiltinType::Normal, Some(BuiltinType::Flying)), from = Species::Staravia)]
  Staraptor = 398,
  #[assoc(name = "Bidoof", ty = (BuiltinType::Normal, None), to = &[Species::Bibarel])]
  Bidoof = 399,
  #[assoc(name = "Bibarel", ty = (BuiltinType::Normal, Some(BuiltinType::Water)), from = Species::Bidoof)]
  Bibarel = 400,
  #[assoc(name = "Kricketot", ty = (BuiltinType::Bug, None), to = &[Species::Kricketune])]
  Kricketot = 401,
  #[assoc(name = "Kricketune", ty = (BuiltinType::Bug, None), from = Species::Kricketot)]
  Kricketune = 402,
  #[assoc(name = "Shinx", ty = (BuiltinType::Electric, None), to = &[Species::Luxio])]
  Shinx = 403,
  #[assoc(name = "Luxio", ty = (BuiltinType::Electric, None), from = Species::Shinx, to = &[Species::Luxray])]
  Luxio = 404,
  #[assoc(name = "Luxray", ty = (BuiltinType::Electric, None), from = Species::Luxio)]
  Luxray = 405,
  #[assoc(name = "Budew", ty = (BuiltinType::Grass, Some(BuiltinType::Poison)), to = &[Species::Roselia])]
  Budew = 406,
  #[assoc(name = "Roserade", ty = (BuiltinType::Grass, Some(BuiltinType::Poison)), from = Species::Roselia)]
  Roserade = 407,
  #[assoc(name = "Cranidos", ty = (BuiltinType::Rock, None), to = &[Species::Rampardos])]
  Cranidos = 408,
  #[assoc(name = "Rampardos", ty = (BuiltinType::Rock, None), from = Species::Cranidos)]
  Rampardos = 409,
  #[assoc(name = "Shieldon", ty = (BuiltinType::Rock, Some(BuiltinType::Steel)), to = &[Species::Bastiodon])]
  Shieldon = 410,
  #[assoc(name = "Bastiodon", ty = (BuiltinType::Rock, Some(BuiltinType::Steel)), from = Species::Shieldon)]
  Bastiodon = 411,
  #[assoc(name = "Burmy", ty = (BuiltinType::Bug, None), to = &[Species::Wormadam, Species::Mothim])]
  Burmy = 412,
  #[assoc(name = "Wormadam", ty = (BuiltinType::Bug, Some(BuiltinType::Grass)), from = Species::Burmy)]
  Wormadam = 413,
  #[assoc(name = "Mothim", ty = (BuiltinType::Bug, Some(BuiltinType::Flying)), from = Species::Burmy)]
  Mothim = 414,
  #[assoc(name = "Combee", ty = (BuiltinType::Bug, Some(BuiltinType::Flying)), to = &[Species::Vespiquen])]
  Combee = 415,
  #[assoc(name = "Vespiquen", ty = (BuiltinType::Bug, Some(BuiltinType::Flying)), from = Species::Combee)]
  Vespiquen = 416,
  #[assoc(name = "Pachirisu", ty = (BuiltinType::Electric, None))]
  Pachirisu = 417,
  #[assoc(name = "Buizel", ty = (BuiltinType::Water, None), to = &[Species::Floatzel])]
  Buizel = 418,
  #[assoc(name = "Floatzel", ty = (BuiltinType::Water, None), from = Species::Buizel)]
  Floatzel = 419,
  #[assoc(name = "Cherubi", ty = (BuiltinType::Grass, None), to = &[Species::Cherrim])]
  Cherubi = 420,
  #[assoc(name = "Cherrim", ty = (BuiltinType::Grass, None), from = Species::Cherubi)]
  Cherrim = 421,
  #[assoc(name = "Shellos", ty = (BuiltinType::Water, None), to = &[Species::Gastrodon])]
  Shellos = 422,
  #[assoc(name = "Gastrodon", ty = (BuiltinType::Water, Some(BuiltinType::Ground)), from = Species::Shellos)]
  Gastrodon = 423,
  #[assoc(name = "Ambipom", ty = (BuiltinType::Normal, None), from = Species::Aipom)]
  Ambipom = 424,
  #[assoc(name = "Drifloon", ty = (BuiltinType::Ghost, Some(BuiltinType::Flying)), to = &[Species::Drifblim])]
  Drifloon = 425,
  #[assoc(name = "Drifblim", ty = (BuiltinType::Ghost, Some(BuiltinType::Flying)), from = Species::Drifloon)]
  Drifblim = 426,
  #[assoc(name = "Buneary", ty = (BuiltinType::Normal, None), to = &[Species::Lopunny])]
  Buneary = 427,
  #[assoc(name = "Lopunny", ty = (BuiltinType::Normal, None), from = Species::Buneary)]
  Lopunny = 428,
  #[assoc(name = "Mismagius", ty = (BuiltinType::Ghost, None), from = Species::Misdreavus)]
  Mismagius = 429,
  #[assoc(name = "Honchkrow", ty = (BuiltinType::Dark, Some(BuiltinType::Flying)), from = Species::Murkrow)]
  Honchkrow = 430,
  #[assoc(name = "Glameow", ty = (BuiltinType::Normal, None), to = &[Species::Purugly])]
  Glameow = 431,
  #[assoc(name = "Purugly", ty = (BuiltinType::Normal, None), from = Species::Glameow)]
  Purugly = 432,
  #[assoc(name = "Chingling", ty = (BuiltinType::Psychic, None), to = &[Species::Chimecho])]
  Chingling = 433,
  #[assoc(name = "Stunky", ty = (BuiltinType::Poison, Some(BuiltinType::Dark)), to = &[Species::Skuntank])]
  Stunky = 434,
  #[assoc(name = "Skuntank", ty = (BuiltinType::Poison, Some(BuiltinType::Dark)), from = Species::Stunky)]
  Skuntank = 435,
  #[assoc(name = "Bronzor", ty = (BuiltinType::Steel, Some(BuiltinType::Psychic)), to = &[Species::Bronzong])]
  Bronzor = 436,
  #[assoc(name = "Bronzong", ty = (BuiltinType::Steel, Some(BuiltinType::Psychic)), from = Species::Bronzor)]
  Bronzong = 437,
  #[assoc(name = "Bonsly", ty = (BuiltinType::Rock, None), to = &[Species::Sudowoodo])]
  Bonsly = 438,
  #[assoc(name = "Mime Jr.", ty = (BuiltinType::Psychic, Some(BuiltinType::Fairy)), to = &[Species::MrMime])]
  MimeJr = 439,
  #[assoc(name = "Happiny", ty = (BuiltinType::Normal, None), to = &[Species::Chansey])]
  Happiny = 440,
  #[assoc(name = "Chatot", ty = (BuiltinType::Normal, Some(BuiltinType::Flying)))]
  Chatot = 441,
  #[assoc(name = "Spiritomb", ty = (BuiltinType::Ghost, Some(BuiltinType::Dark)))]
  Spiritomb = 442,
  #[assoc(name = "Gible", ty = (BuiltinType::Dragon, Some(BuiltinType::Ground)), to = &[Species::Gabite])]
  Gible = 443,
  #[assoc(name = "Gabite", ty = (BuiltinType::Dragon, Some(BuiltinType::Ground)), from = Species::Gible, to = &[Species::Garchomp])]
  Gabite = 444,
  #[assoc(name = "Garchomp", ty = (BuiltinType::Dragon, Some(BuiltinType::Ground)), from = Species::Gabite)]
  Garchomp = 445,
  #[assoc(name = "Munchlax", ty = (BuiltinType::Normal, None), to = &[Species::Snorlax])]
  Munchlax = 446,
  #[assoc(name = "Riolu", ty = (BuiltinType::Fighting, None), to = &[Species::Lucario])]
  Riolu = 447,
  #[assoc(name = "Lucario", ty = (BuiltinType::Fighting, Some(BuiltinType::Steel)), from = Species::Riolu)]
  Lucario = 448,
  #[assoc(name = "Hippopotas", ty = (BuiltinType::Ground, None), to = &[Species::Hippowdon])]
  Hippopotas = 449,
  #[assoc(name = "Hippowdon", ty = (BuiltinType::Ground, None), from = Species::Hippopotas)]
  Hippowdon = 450,
  #[assoc(name = "Skorupi", ty = (BuiltinType::Poison, Some(BuiltinType::Bug)), to = &[Species::Drapion])]
  Skorupi = 451,
  #[assoc(name = "Drapion", ty = (BuiltinType::Poison, Some(BuiltinType::Dark)), from = Species::Skorupi)]
  Drapion = 452,
  #[assoc(name = "Croagunk", ty = (BuiltinType::Poison, Some(BuiltinType::Fighting)), to = &[Species::Toxicroak])]
  Croagunk = 453,
  #[assoc(name = "Toxicroak", ty = (BuiltinType::Poison, Some(BuiltinType::Fighting)), from = Species::Croagunk)]
  Toxicroak = 454,
  #[assoc(name = "Carnivine", ty = (BuiltinType::Grass, None))]
  Carnivine = 455,
  #[assoc(name = "Finneon", ty = (BuiltinType::Water, None), to = &[Species::Lumineon])]
  Finneon = 456,
  #[assoc(name = "Lumineon", ty = (BuiltinType::Water, None), from = Species::Finneon)]
  Lumineon = 457,
  #[assoc(name = "Mantyke", ty = (BuiltinType::Water, Some(BuiltinType::Flying)), to = &[Species::Mantine])]
  Mantyke = 458,
  #[assoc(name = "Snover", ty = (BuiltinType::Grass, Some(BuiltinType::Ice)), to = &[Species::Abomasnow])]
  Snover = 459,
  #[assoc(name = "Abomasnow", ty = (BuiltinType::Grass, Some(BuiltinType::Ice)), from = Species::Snover)]
  Abomasnow = 460,
  #[assoc(name = "Weavile", ty = (BuiltinType::Dark, Some(BuiltinType::Ice)), from = Species::Sneasel)]
  Weavile = 461,
  #[assoc(name = "Magnezone", ty = (BuiltinType::Electric, Some(BuiltinType::Steel)), from = Species::Magneton)]
  Magnezone = 462,
  #[assoc(name = "Lickilicky", ty = (BuiltinType::Normal, None), from = Species::Lickitung)]
  Lickilicky = 463,
  #[assoc(name = "Rhyperior", ty = (BuiltinType::Ground, Some(BuiltinType::Rock)), from = Species::Rhydon)]
  Rhyperior = 464,
  #[assoc(name = "Tangrowth", ty = (BuiltinType::Grass, None), from = Species::Tangela)]
  Tangrowth = 465,
  #[assoc(name = "Electivire", ty = (BuiltinType::Electric, None), from = Species::Electabuzz)]
  Electivire = 466,
  #[assoc(name = "Magmortar", ty = (BuiltinType::Fire, None), from = Species::Magmar)]
  Magmortar = 467,
  #[assoc(name = "Togekiss", ty = (BuiltinType::Fairy, Some(BuiltinType::Flying)), from = Species::Togetic)]
  Togekiss = 468,
  #[assoc(name = "Yanmega", ty = (BuiltinType::Bug, Some(BuiltinType::Flying)), from = Species::Yanma)]
  Yanmega = 469,
  #[assoc(name = "Leafeon", ty = (BuiltinType::Grass, None), from = Species::Eevee)]
  Leafeon = 470,
  #[assoc(name = "Glaceon", ty = (BuiltinType::Ice, None), from = Species::Eevee)]
  Glaceon = 471,
  #[assoc(name = "Gliscor", ty = (BuiltinType::Ground, Some(BuiltinType::Flying)), from = Species::Gligar)]
  Gliscor = 472,
  #[assoc(name = "Mamoswine", ty = (BuiltinType::Ice, Some(BuiltinType::Ground)), from = Species::Piloswine)]
  Mamoswine = 473,
  #[assoc(name = "Porygon-Z", ty = (BuiltinType::Normal, None), from = Species::Porygon2)]
  PorygonZ = 474,
  #[assoc(name = "Gallade", ty = (BuiltinType::Psychic, Some(BuiltinType::Fighting)), from = Species::Kirlia)]
  Gallade = 475,
  #[assoc(name = "Probopass", ty = (BuiltinType::Rock, Some(BuiltinType::Steel)), from = Species::Nosepass)]
  Probopass = 476,
  #[assoc(name = "Dusknoir", ty = (BuiltinType::Ghost, None), from = Species::Dusclops)]
  Dusknoir = 477,
  #[assoc(name = "Froslass", ty = (BuiltinType::Ice, Some(BuiltinType::Ghost)), from = Species::Snorunt)]
  Froslass = 478,
  #[assoc(name = "Rotom", ty = (BuiltinType::Electric, Some(BuiltinType::Ghost)))]
  Rotom = 479,
  #[assoc(name = "Uxie", ty = (BuiltinType::Psychic, None))]
  Uxie = 480,
  #[assoc(name = "Mesprit", ty = (BuiltinType::Psychic, None))]
  Mesprit = 481,
  #[assoc(name = "Azelf", ty = (BuiltinType::Psychic, None))]
  Azelf = 482,
  #[assoc(name = "Dialga", ty = (BuiltinType::Steel, Some(BuiltinType::Dragon)))]
  Dialga = 483,
  #[assoc(name = "Palkia", ty = (BuiltinType::Water, Some(BuiltinType::Dragon)))]
  Palkia = 484,
  #[assoc(name = "Heatran", ty = (BuiltinType::Fire, Some(BuiltinType::Steel)))]
  Heatran = 485,
  #[assoc(name = "Regigigas", ty = (BuiltinType::Normal, None))]
  Regigigas = 486,
  #[assoc(name = "Giratina", ty = (BuiltinType::Ghost, Some(BuiltinType::Dragon)))]
  Giratina = 487,
  #[assoc(name = "Cresselia", ty = (BuiltinType::Psychic, None))]
  Cresselia = 488,
  #[assoc(name = "Phione", ty = (BuiltinType::Water, None), to = &[Species::Manaphy])]
  Phione = 489,
  #[assoc(name = "Manaphy", ty = (BuiltinType::Water, None))]
  Manaphy = 490,
  #[assoc(name = "Darkrai", ty = (BuiltinType::Dark, None))]
  Darkrai = 491,
  #[assoc(name = "Shaymin", ty = (BuiltinType::Grass, None))]
  Shaymin = 492,
  #[assoc(name = "Arceus", ty = (BuiltinType::Normal, None))]
  Arceus = 493,
  #[assoc(name = "Victini", ty = (BuiltinType::Psychic, Some(BuiltinType::Fire)))]
  Victini = 494,
  #[assoc(name = "Snivy", ty = (BuiltinType::Grass, None), to = &[Species::Servine])]
  Snivy = 495,
  #[assoc(name = "Servine", ty = (BuiltinType::Grass, None), from = Species::Snivy, to = &[Species::Serperior])]
  Servine = 496,
  #[assoc(name = "Serperior", ty = (BuiltinType::Grass, None), from = Species::Servine)]
  Serperior = 497,
  #[assoc(name = "Tepig", ty = (BuiltinType::Fire, None), to = &[Species::Pignite])]
  Tepig = 498,
  #[assoc(name = "Pignite", ty = (BuiltinType::Fire, Some(BuiltinType::Fighting)), from = Species::Tepig, to = &[Species::Emboar])]
  Pignite = 499,
  #[assoc(name = "Emboar", ty = (BuiltinType::Fire, Some(BuiltinType::Fighting)), from = Species::Pignite)]
  Emboar = 500,
  #[assoc(name = "Oshawott", ty = (BuiltinType::Water, None), to = &[Species::Dewott])]
  Oshawott = 501,
  #[assoc(name = "Dewott", ty = (BuiltinType::Water, None), from = Species::Oshawott, to = &[Species::Samurott])]
  Dewott = 502,
  #[assoc(name = "Samurott", ty = (BuiltinType::Water, None), from = Species::Dewott)]
  Samurott = 503,
  #[assoc(name = "Patrat", ty = (BuiltinType::Normal, None), to = &[Species::Watchog])]
  Patrat = 504,
  #[assoc(name = "Watchog", ty = (BuiltinType::Normal, None), from = Species::Patrat)]
  Watchog = 505,
  #[assoc(name = "Lillipup", ty = (BuiltinType::Normal, None), to = &[Species::Herdier])]
  Lillipup = 506,
  #[assoc(name = "Herdier", ty = (BuiltinType::Normal, None), from = Species::Lillipup, to = &[Species::Stoutland])]
  Herdier = 507,
  #[assoc(name = "Stoutland", ty = (BuiltinType::Normal, None), from = Species::Herdier)]
  Stoutland = 508,
  #[assoc(name = "Purrloin", ty = (BuiltinType::Dark, None), to = &[Species::Liepard])]
  Purrloin = 509,
  #[assoc(name = "Liepard", ty = (BuiltinType::Dark, None), from = Species::Purrloin)]
  Liepard = 510,
  #[assoc(name = "Pansage", ty = (BuiltinType::Grass, None), to = &[Species::Simisage])]
  Pansage = 511,
  #[assoc(name = "Simisage", ty = (BuiltinType::Grass, None), from = Species::Pansage)]
  Simisage = 512,
  #[assoc(name = "Pansear", ty = (BuiltinType::Fire, None), to = &[Species::Simisear])]
  Pansear = 513,
  #[assoc(name = "Simisear", ty = (BuiltinType::Fire, None), from = Species::Pansear)]
  Simisear = 514,
  #[assoc(name = "Panpour", ty = (BuiltinType::Water, None), to = &[Species::Simipour])]
  Panpour = 515,
  #[assoc(name = "Simipour", ty = (BuiltinType::Water, None), from = Species::Panpour)]
  Simipour = 516,
  #[assoc(name = "Munna", ty = (BuiltinType::Psychic, None), to = &[Species::Musharna])]
  Munna = 517,
  #[assoc(name = "Musharna", ty = (BuiltinType::Psychic, None), from = Species::Munna)]
  Musharna = 518,
  #[assoc(name = "Pidove", ty = (BuiltinType::Normal, Some(BuiltinType::Flying)), to = &[Species::Tranquill])]
  Pidove = 519,
  #[assoc(name = "Tranquill", ty = (BuiltinType::Normal, Some(BuiltinType::Flying)), from = Species::Pidove, to = &[Species::Unfezant])]
  Tranquill = 520,
  #[assoc(name = "Unfezant", ty = (BuiltinType::Normal, Some(BuiltinType::Flying)), from = Species::Tranquill)]
  Unfezant = 521,
  #[assoc(name = "Blitzle", ty = (BuiltinType::Electric, None), to = &[Species::Zebstrika])]
  Blitzle = 522,
  #[assoc(name = "Zebstrika", ty = (BuiltinType::Electric, None), from = Species::Blitzle)]
  Zebstrika = 523,
  #[assoc(name = "Roggenrola", ty = (BuiltinType::Rock, None), to = &[Species::Boldore])]
  Roggenrola = 524,
  #[assoc(name = "Boldore", ty = (BuiltinType::Rock, None), from = Species::Roggenrola, to = &[Species::Gigalith])]
  Boldore = 525,
  #[assoc(name = "Gigalith", ty = (BuiltinType::Rock, None), from = Species::Boldore)]
  Gigalith = 526,
  #[assoc(name = "Woobat", ty = (BuiltinType::Psychic, Some(BuiltinType::Flying)), to = &[Species::Swoobat])]
  Woobat = 527,
  #[assoc(name = "Swoobat", ty = (BuiltinType::Psychic, Some(BuiltinType::Flying)), from = Species::Woobat)]
  Swoobat = 528,
  #[assoc(name = "Drilbur", ty = (BuiltinType::Ground, None), to = &[Species::Excadrill])]
  Drilbur = 529,
  #[assoc(name = "Excadrill", ty = (BuiltinType::Ground, Some(BuiltinType::Steel)), from = Species::Drilbur)]
  Excadrill = 530,
  #[assoc(name = "Audino", ty = (BuiltinType::Normal, None))]
  Audino = 531,
  #[assoc(name = "Timburr", ty = (BuiltinType::Fighting, None), to = &[Species::Gurdurr])]
  Timburr = 532,
  #[assoc(name = "Gurdurr", ty = (BuiltinType::Fighting, None), from = Species::Timburr, to = &[Species::Conkeldurr])]
  Gurdurr = 533,
  #[assoc(name = "Conkeldurr", ty = (BuiltinType::Fighting, None), from = Species::Gurdurr)]
  Conkeldurr = 534,
  #[assoc(name = "Tympole", ty = (BuiltinType::Water, None), to = &[Species::Palpitoad])]
  Tympole = 535,
  #[assoc(name = "Palpitoad", ty = (BuiltinType::Water, Some(BuiltinType::Ground)), from = Species::Tympole, to = &[Species::Seismitoad])]
  Palpitoad = 536,
  #[assoc(name = "Seismitoad", ty = (BuiltinType::Water, Some(BuiltinType::Ground)), from = Species::Palpitoad)]
  Seismitoad = 537,
  #[assoc(name = "Throh", ty = (BuiltinType::Fighting, None))]
  Throh = 538,
  #[assoc(name = "Sawk", ty = (BuiltinType::Fighting, None))]
  Sawk = 539,
  #[assoc(name = "Sewaddle", ty = (BuiltinType::Bug, Some(BuiltinType::Grass)), to = &[Species::Swadloon])]
  Sewaddle = 540,
  #[assoc(name = "Swadloon", ty = (BuiltinType::Bug, Some(BuiltinType::Grass)), from = Species::Sewaddle, to = &[Species::Leavanny])]
  Swadloon = 541,
  #[assoc(name = "Leavanny", ty = (BuiltinType::Bug, Some(BuiltinType::Grass)), from = Species::Swadloon)]
  Leavanny = 542,
  #[assoc(name = "Venipede", ty = (BuiltinType::Bug, Some(BuiltinType::Poison)), to = &[Species::Whirlipede])]
  Venipede = 543,
  #[assoc(name = "Whirlipede", ty = (BuiltinType::Bug, Some(BuiltinType::Poison)), from = Species::Venipede, to = &[Species::Scolipede])]
  Whirlipede = 544,
  #[assoc(name = "Scolipede", ty = (BuiltinType::Bug, Some(BuiltinType::Poison)), from = Species::Whirlipede)]
  Scolipede = 545,
  #[assoc(name = "Cottonee", ty = (BuiltinType::Grass, Some(BuiltinType::Fairy)), to = &[Species::Whimsicott])]
  Cottonee = 546,
  #[assoc(name = "Whimsicott", ty = (BuiltinType::Grass, Some(BuiltinType::Fairy)), from = Species::Cottonee)]
  Whimsicott = 547,
  #[assoc(name = "Petilil", ty = (BuiltinType::Grass, None), to = &[Species::Lilligant])]
  Petilil = 548,
  #[assoc(name = "Lilligant", ty = (BuiltinType::Grass, None), from = Species::Petilil)]
  Lilligant = 549,
  #[assoc(name = "Basculin", ty = (BuiltinType::Water, None), to = &[Species::Basculegion])]
  Basculin = 550,
  #[assoc(name = "Sandile", ty = (BuiltinType::Ground, Some(BuiltinType::Dark)), to = &[Species::Krokorok])]
  Sandile = 551,
  #[assoc(name = "Krokorok", ty = (BuiltinType::Ground, Some(BuiltinType::Dark)), from = Species::Sandile, to = &[Species::Krookodile])]
  Krokorok = 552,
  #[assoc(name = "Krookodile", ty = (BuiltinType::Ground, Some(BuiltinType::Dark)), from = Species::Krokorok)]
  Krookodile = 553,
  #[assoc(name = "Darumaka", ty = (BuiltinType::Fire, None), to = &[Species::Darmanitan])]
  Darumaka = 554,
  #[assoc(name = "Darmanitan", ty = (BuiltinType::Fire, None), from = Species::Darumaka)]
  Darmanitan = 555,
  #[assoc(name = "Maractus", ty = (BuiltinType::Grass, None))]
  Maractus = 556,
  #[assoc(name = "Dwebble", ty = (BuiltinType::Bug, Some(BuiltinType::Rock)), to = &[Species::Crustle])]
  Dwebble = 557,
  #[assoc(name = "Crustle", ty = (BuiltinType::Bug, Some(BuiltinType::Rock)), from = Species::Dwebble)]
  Crustle = 558,
  #[assoc(name = "Scraggy", ty = (BuiltinType::Dark, Some(BuiltinType::Fighting)), to = &[Species::Scrafty])]
  Scraggy = 559,
  #[assoc(name = "Scrafty", ty = (BuiltinType::Dark, Some(BuiltinType::Fighting)), from = Species::Scraggy)]
  Scrafty = 560,
  #[assoc(name = "Sigilyph", ty = (BuiltinType::Psychic, Some(BuiltinType::Flying)))]
  Sigilyph = 561,
  #[assoc(name = "Yamask", ty = (BuiltinType::Ghost, None), to = &[Species::Cofagrigus, Species::Runerigus])]
  Yamask = 562,
  #[assoc(name = "Cofagrigus", ty = (BuiltinType::Ghost, None), from = Species::Yamask)]
  Cofagrigus = 563,
  #[assoc(name = "Tirtouga", ty = (BuiltinType::Water, Some(BuiltinType::Rock)), to = &[Species::Carracosta])]
  Tirtouga = 564,
  #[assoc(name = "Carracosta", ty = (BuiltinType::Water, Some(BuiltinType::Rock)), from = Species::Tirtouga)]
  Carracosta = 565,
  #[assoc(name = "Archen", ty = (BuiltinType::Rock, Some(BuiltinType::Flying)), to = &[Species::Archeops])]
  Archen = 566,
  #[assoc(name = "Archeops", ty = (BuiltinType::Rock, Some(BuiltinType::Flying)), from = Species::Archen)]
  Archeops = 567,
  #[assoc(name = "Trubbish", ty = (BuiltinType::Poison, None), to = &[Species::Garbodor])]
  Trubbish = 568,
  #[assoc(name = "Garbodor", ty = (BuiltinType::Poison, None), from = Species::Trubbish)]
  Garbodor = 569,
  #[assoc(name = "Zorua", ty = (BuiltinType::Dark, None), to = &[Species::Zoroark])]
  Zorua = 570,
  #[assoc(name = "Zoroark", ty = (BuiltinType::Dark, None), from = Species::Zorua)]
  Zoroark = 571,
  #[assoc(name = "Minccino", ty = (BuiltinType::Normal, None), to = &[Species::Cinccino])]
  Minccino = 572,
  #[assoc(name = "Cinccino", ty = (BuiltinType::Normal, None), from = Species::Minccino)]
  Cinccino = 573,
  #[assoc(name = "Gothita", ty = (BuiltinType::Psychic, None), to = &[Species::Gothorita])]
  Gothita = 574,
  #[assoc(name = "Gothorita", ty = (BuiltinType::Psychic, None), from = Species::Gothita, to = &[Species::Gothitelle])]
  Gothorita = 575,
  #[assoc(name = "Gothitelle", ty = (BuiltinType::Psychic, None), from = Species::Gothorita)]
  Gothitelle = 576,
  #[assoc(name = "Solosis", ty = (BuiltinType::Psychic, None), to = &[Species::Duosion])]
  Solosis = 577,
  #[assoc(name = "Duosion", ty = (BuiltinType::Psychic, None), from = Species::Solosis, to = &[Species::Reuniclus])]
  Duosion = 578,
  #[assoc(name = "Reuniclus", ty = (BuiltinType::Psychic, None), from = Species::Duosion)]
  Reuniclus = 579,
  #[assoc(name = "Ducklett", ty = (BuiltinType::Water, Some(BuiltinType::Flying)), to = &[Species::Swanna])]
  Ducklett = 580,
  #[assoc(name = "Swanna", ty = (BuiltinType::Water, Some(BuiltinType::Flying)), from = Species::Ducklett)]
  Swanna = 581,
  #[assoc(name = "Vanillite", ty = (BuiltinType::Ice, None), to = &[Species::Vanillish])]
  Vanillite = 582,
  #[assoc(name = "Vanillish", ty = (BuiltinType::Ice, None), from = Species::Vanillite, to = &[Species::Vanilluxe])]
  Vanillish = 583,
  #[assoc(name = "Vanilluxe", ty = (BuiltinType::Ice, None), from = Species::Vanillish)]
  Vanilluxe = 584,
  #[assoc(name = "Deerling", ty = (BuiltinType::Normal, Some(BuiltinType::Grass)), to = &[Species::Sawsbuck])]
  Deerling = 585,
  #[assoc(name = "Sawsbuck", ty = (BuiltinType::Normal, Some(BuiltinType::Grass)), from = Species::Deerling)]
  Sawsbuck = 586,
  #[assoc(name = "Emolga", ty = (BuiltinType::Electric, Some(BuiltinType::Flying)))]
  Emolga = 587,
  #[assoc(name = "Karrablast", ty = (BuiltinType::Bug, None), to = &[Species::Escavalier])]
  Karrablast = 588,
  #[assoc(name = "Escavalier", ty = (BuiltinType::Bug, Some(BuiltinType::Steel)), from = Species::Karrablast)]
  Escavalier = 589,
  #[assoc(name = "Foongus", ty = (BuiltinType::Grass, Some(BuiltinType::Poison)), to = &[Species::Amoonguss])]
  Foongus = 590,
  #[assoc(name = "Amoonguss", ty = (BuiltinType::Grass, Some(BuiltinType::Poison)), from = Species::Foongus)]
  Amoonguss = 591,
  #[assoc(name = "Frillish", ty = (BuiltinType::Water, Some(BuiltinType::Ghost)), to = &[Species::Jellicent])]
  Frillish = 592,
  #[assoc(name = "Jellicent", ty = (BuiltinType::Water, Some(BuiltinType::Ghost)), from = Species::Frillish)]
  Jellicent = 593,
  #[assoc(name = "Alomomola", ty = (BuiltinType::Water, None))]
  Alomomola = 594,
  #[assoc(name = "Joltik", ty = (BuiltinType::Bug, Some(BuiltinType::Electric)), to = &[Species::Galvantula])]
  Joltik = 595,
  #[assoc(name = "Galvantula", ty = (BuiltinType::Bug, Some(BuiltinType::Electric)), from = Species::Joltik)]
  Galvantula = 596,
  #[assoc(name = "Ferroseed", ty = (BuiltinType::Grass, Some(BuiltinType::Steel)), to = &[Species::Ferrothorn])]
  Ferroseed = 597,
  #[assoc(name = "Ferrothorn", ty = (BuiltinType::Grass, Some(BuiltinType::Steel)), from = Species::Ferroseed)]
  Ferrothorn = 598,
  #[assoc(name = "Klink", ty = (BuiltinType::Steel, None), to = &[Species::Klang])]
  Klink = 599,
  #[assoc(name = "Klang", ty = (BuiltinType::Steel, None), from = Species::Klink, to = &[Species::Klinklang])]
  Klang = 600,
  #[assoc(name = "Klinklang", ty = (BuiltinType::Steel, None), from = Species::Klang)]
  Klinklang = 601,
  #[assoc(name = "Tynamo", ty = (BuiltinType::Electric, None), to = &[Species::Eelektrik])]
  Tynamo = 602,
  #[assoc(name = "Eelektrik", ty = (BuiltinType::Electric, None), from = Species::Tynamo, to = &[Species::Eelektross])]
  Eelektrik = 603,
  #[assoc(name = "Eelektross", ty = (BuiltinType::Electric, None), from = Species::Eelektrik)]
  Eelektross = 604,
  #[assoc(name = "Elgyem", ty = (BuiltinType::Psychic, None), to = &[Species::Beheeyem])]
  Elgyem = 605,
  #[assoc(name = "Beheeyem", ty = (BuiltinType::Psychic, None), from = Species::Elgyem)]
  Beheeyem = 606,
  #[assoc(name = "Litwick", ty = (BuiltinType::Ghost, Some(BuiltinType::Fire)), to = &[Species::Lampent])]
  Litwick = 607,
  #[assoc(name = "Lampent", ty = (BuiltinType::Ghost, Some(BuiltinType::Fire)), from = Species::Litwick, to = &[Species::Chandelure])]
  Lampent = 608,
  #[assoc(name = "Chandelure", ty = (BuiltinType::Ghost, Some(BuiltinType::Fire)), from = Species::Lampent)]
  Chandelure = 609,
  #[assoc(name = "Axew", ty = (BuiltinType::Dragon, None), to = &[Species::Fraxure])]
  Axew = 610,
  #[assoc(name = "Fraxure", ty = (BuiltinType::Dragon, None), from = Species::Axew, to = &[Species::Haxorus])]
  Fraxure = 611,
  #[assoc(name = "Haxorus", ty = (BuiltinType::Dragon, None), from = Species::Fraxure)]
  Haxorus = 612,
  #[assoc(name = "Cubchoo", ty = (BuiltinType::Ice, None), to = &[Species::Beartic])]
  Cubchoo = 613,
  #[assoc(name = "Beartic", ty = (BuiltinType::Ice, None), from = Species::Cubchoo)]
  Beartic = 614,
  #[assoc(name = "Cryogonal", ty = (BuiltinType::Ice, None))]
  Cryogonal = 615,
  #[assoc(name = "Shelmet", ty = (BuiltinType::Bug, None), to = &[Species::Accelgor])]
  Shelmet = 616,
  #[assoc(name = "Accelgor", ty = (BuiltinType::Bug, None), from = Species::Shelmet)]
  Accelgor = 617,
  #[assoc(name = "Stunfisk", ty = (BuiltinType::Ground, Some(BuiltinType::Electric)))]
  Stunfisk = 618,
  #[assoc(name = "Mienfoo", ty = (BuiltinType::Fighting, None), to = &[Species::Mienshao])]
  Mienfoo = 619,
  #[assoc(name = "Mienshao", ty = (BuiltinType::Fighting, None), from = Species::Mienfoo)]
  Mienshao = 620,
  #[assoc(name = "Druddigon", ty = (BuiltinType::Dragon, None))]
  Druddigon = 621,
  #[assoc(name = "Golett", ty = (BuiltinType::Ground, Some(BuiltinType::Ghost)), to = &[Species::Golurk])]
  Golett = 622,
  #[assoc(name = "Golurk", ty = (BuiltinType::Ground, Some(BuiltinType::Ghost)), from = Species::Golett)]
  Golurk = 623,
  #[assoc(name = "Pawniard", ty = (BuiltinType::Dark, Some(BuiltinType::Steel)), to = &[Species::Bisharp])]
  Pawniard = 624,
  #[assoc(name = "Bisharp", ty = (BuiltinType::Dark, Some(BuiltinType::Steel)), from = Species::Pawniard, to = &[Species::Kingambit])]
  Bisharp = 625,
  #[assoc(name = "Bouffalant", ty = (BuiltinType::Normal, None))]
  Bouffalant = 626,
  #[assoc(name = "Rufflet", ty = (BuiltinType::Normal, Some(BuiltinType::Flying)), to = &[Species::Braviary])]
  Rufflet = 627,
  #[assoc(name = "Braviary", ty = (BuiltinType::Normal, Some(BuiltinType::Flying)), from = Species::Rufflet)]
  Braviary = 628,
  #[assoc(name = "Vullaby", ty = (BuiltinType::Dark, Some(BuiltinType::Flying)), to = &[Species::Mandibuzz])]
  Vullaby = 629,
  #[assoc(name = "Mandibuzz", ty = (BuiltinType::Dark, Some(BuiltinType::Flying)), from = Species::Vullaby)]
  Mandibuzz = 630,
  #[assoc(name = "Heatmor", ty = (BuiltinType::Fire, None))]
  Heatmor = 631,
  #[assoc(name = "Durant", ty = (BuiltinType::Bug, Some(BuiltinType::Steel)))]
  Durant = 632,
  #[assoc(name = "Deino", ty = (BuiltinType::Dark, Some(BuiltinType::Dragon)), to = &[Species::Zweilous])]
  Deino = 633,
  #[assoc(name = "Zweilous", ty = (BuiltinType::Dark, Some(BuiltinType::Dragon)), from = Species::Deino, to = &[Species::Hydreigon])]
  Zweilous = 634,
  #[assoc(name = "Hydreigon", ty = (BuiltinType::Dark, Some(BuiltinType::Dragon)), from = Species::Zweilous)]
  Hydreigon = 635,
  #[assoc(name = "Larvesta", ty = (BuiltinType::Bug, Some(BuiltinType::Fire)), to = &[Species::Volcarona])]
  Larvesta = 636,
  #[assoc(name = "Volcarona", ty = (BuiltinType::Bug, Some(BuiltinType::Fire)), from = Species::Larvesta)]
  Volcarona = 637,
  #[assoc(name = "Cobalion", ty = (BuiltinType::Steel, Some(BuiltinType::Fighting)))]
  Cobalion = 638,
  #[assoc(name = "Terrakion", ty = (BuiltinType::Rock, Some(BuiltinType::Fighting)))]
  Terrakion = 639,
  #[assoc(name = "Virizion", ty = (BuiltinType::Grass, Some(BuiltinType::Fighting)))]
  Virizion = 640,
  #[assoc(name = "Tornadus", ty = (BuiltinType::Flying, None))]
  Tornadus = 641,
  #[assoc(name = "Thundurus", ty = (BuiltinType::Electric, Some(BuiltinType::Flying)))]
  Thundurus = 642,
  #[assoc(name = "Reshiram", ty = (BuiltinType::Dragon, Some(BuiltinType::Fire)))]
  Reshiram = 643,
  #[assoc(name = "Zekrom", ty = (BuiltinType::Dragon, Some(BuiltinType::Electric)))]
  Zekrom = 644,
  #[assoc(name = "Landorus", ty = (BuiltinType::Ground, Some(BuiltinType::Flying)))]
  Landorus = 645,
  #[assoc(name = "Kyurem", ty = (BuiltinType::Dragon, Some(BuiltinType::Ice)))]
  Kyurem = 646,
  #[assoc(name = "Keldeo", ty = (BuiltinType::Water, Some(BuiltinType::Fighting)))]
  Keldeo = 647,
  #[assoc(name = "Meloetta", ty = (BuiltinType::Normal, Some(BuiltinType::Psychic)))]
  Meloetta = 648,
  #[assoc(name = "Genesect", ty = (BuiltinType::Bug, Some(BuiltinType::Steel)))]
  Genesect = 649,
  #[assoc(name = "Chespin", ty = (BuiltinType::Grass, None), to = &[Species::Quilladin])]
  Chespin = 650,
  #[assoc(name = "Quilladin", ty = (BuiltinType::Grass, None), from = Species::Chespin, to = &[Species::Chesnaught])]
  Quilladin = 651,
  #[assoc(name = "Chesnaught", ty = (BuiltinType::Grass, Some(BuiltinType::Fighting)), from = Species::Quilladin)]
  Chesnaught = 652,
  #[assoc(name = "Fennekin", ty = (BuiltinType::Fire, None), to = &[Species::Braixen])]
  Fennekin = 653,
  #[assoc(name = "Braixen", ty = (BuiltinType::Fire, None), from = Species::Fennekin, to = &[Species::Delphox])]
  Braixen = 654,
  #[assoc(name = "Delphox", ty = (BuiltinType::Fire, Some(BuiltinType::Psychic)), from = Species::Braixen)]
  Delphox = 655,
  #[assoc(name = "Froakie", ty = (BuiltinType::Water, None), to = &[Species::Frogadier])]
  Froakie = 656,
  #[assoc(name = "Frogadier", ty = (BuiltinType::Water, None), from = Species::Froakie, to = &[Species::Greninja])]
  Frogadier = 657,
  #[assoc(name = "Greninja", ty = (BuiltinType::Water, Some(BuiltinType::Dark)), from = Species::Frogadier)]
  Greninja = 658,
  #[assoc(name = "Bunnelby", ty = (BuiltinType::Normal, None), to = &[Species::Diggersby])]
  Bunnelby = 659,
  #[assoc(name = "Diggersby", ty = (BuiltinType::Normal, Some(BuiltinType::Ground)), from = Species::Bunnelby)]
  Diggersby = 660,
  #[assoc(name = "Fletchling", ty = (BuiltinType::Normal, Some(BuiltinType::Flying)), to = &[Species::Fletchinder])]
  Fletchling = 661,
  #[assoc(name = "Fletchinder", ty = (BuiltinType::Fire, Some(BuiltinType::Flying)), from = Species::Fletchling, to = &[Species::Talonflame])]
  Fletchinder = 662,
  #[assoc(name = "Talonflame", ty = (BuiltinType::Fire, Some(BuiltinType::Flying)), from = Species::Fletchinder)]
  Talonflame = 663,
  #[assoc(name = "Scatterbug", ty = (BuiltinType::Bug, None), to = &[Species::Spewpa])]
  Scatterbug = 664,
  #[assoc(name = "Spewpa", ty = (BuiltinType::Bug, None), from = Species::Scatterbug, to = &[Species::Vivillon])]
  Spewpa = 665,
  #[assoc(name = "Vivillon", ty = (BuiltinType::Bug, Some(BuiltinType::Flying)), from = Species::Spewpa)]
  Vivillon = 666,
  #[assoc(name = "Litleo", ty = (BuiltinType::Fire, Some(BuiltinType::Normal)), to = &[Species::Pyroar])]
  Litleo = 667,
  #[assoc(name = "Pyroar", ty = (BuiltinType::Fire, Some(BuiltinType::Normal)), from = Species::Litleo)]
  Pyroar = 668,
  #[assoc(name = "Flabébé", ty = (BuiltinType::Fairy, None), to = &[Species::Floette])]
  Flabebe = 669,
  #[assoc(name = "Floette", ty = (BuiltinType::Fairy, None), from = Species::Flabebe, to = &[Species::Florges])]
  Floette = 670,
  #[assoc(name = "Florges", ty = (BuiltinType::Fairy, None), from = Species::Floette)]
  Florges = 671,
  #[assoc(name = "Skiddo", ty = (BuiltinType::Grass, None), to = &[Species::Gogoat])]
  Skiddo = 672,
  #[assoc(name = "Gogoat", ty = (BuiltinType::Grass, None), from = Species::Skiddo)]
  Gogoat = 673,
  #[assoc(name = "Pancham", ty = (BuiltinType::Fighting, None), to = &[Species::Pangoro])]
  Pancham = 674,
  #[assoc(name = "Pangoro", ty = (BuiltinType::Fighting, Some(BuiltinType::Dark)), from = Species::Pancham)]
  Pangoro = 675,
  #[assoc(name = "Furfrou", ty = (BuiltinType::Normal, None))]
  Furfrou = 676,
  #[assoc(name = "Espurr", ty = (BuiltinType::Psychic, None), to = &[Species::Meowstic])]
  Espurr = 677,
  #[assoc(name = "Meowstic", ty = (BuiltinType::Psychic, None), from = Species::Espurr)]
  Meowstic = 678,
  #[assoc(name = "Honedge", ty = (BuiltinType::Steel, Some(BuiltinType::Ghost)), to = &[Species::Doublade])]
  Honedge = 679,
  #[assoc(name = "Doublade", ty = (BuiltinType::Steel, Some(BuiltinType::Ghost)), from = Species::Honedge, to = &[Species::Aegislash])]
  Doublade = 680,
  #[assoc(name = "Aegislash", ty = (BuiltinType::Steel, Some(BuiltinType::Ghost)), from = Species::Doublade)]
  Aegislash = 681,
  #[assoc(name = "Spritzee", ty = (BuiltinType::Fairy, None), to = &[Species::Aromatisse])]
  Spritzee = 682,
  #[assoc(name = "Aromatisse", ty = (BuiltinType::Fairy, None), from = Species::Spritzee)]
  Aromatisse = 683,
  #[assoc(name = "Swirlix", ty = (BuiltinType::Fairy, None), to = &[Species::Slurpuff])]
  Swirlix = 684,
  #[assoc(name = "Slurpuff", ty = (BuiltinType::Fairy, None), from = Species::Swirlix)]
  Slurpuff = 685,
  #[assoc(name = "Inkay", ty = (BuiltinType::Dark, Some(BuiltinType::Psychic)), to = &[Species::Malamar])]
  Inkay = 686,
  #[assoc(name = "Malamar", ty = (BuiltinType::Dark, Some(BuiltinType::Psychic)), from = Species::Inkay)]
  Malamar = 687,
  #[assoc(name = "Binacle", ty = (BuiltinType::Rock, Some(BuiltinType::Water)), to = &[Species::Barbaracle])]
  Binacle = 688,
  #[assoc(name = "Barbaracle", ty = (BuiltinType::Rock, Some(BuiltinType::Water)), from = Species::Binacle)]
  Barbaracle = 689,
  #[assoc(name = "Skrelp", ty = (BuiltinType::Poison, Some(BuiltinType::Water)), to = &[Species::Dragalge])]
  Skrelp = 690,
  #[assoc(name = "Dragalge", ty = (BuiltinType::Poison, Some(BuiltinType::Dragon)), from = Species::Skrelp)]
  Dragalge = 691,
  #[assoc(name = "Clauncher", ty = (BuiltinType::Water, None), to = &[Species::Clawitzer])]
  Clauncher = 692,
  #[assoc(name = "Clawitzer", ty = (BuiltinType::Water, None), from = Species::Clauncher)]
  Clawitzer = 693,
  #[assoc(name = "Helioptile", ty = (BuiltinType::Electric, Some(BuiltinType::Normal)), to = &[Species::Heliolisk])]
  Helioptile = 694,
  #[assoc(name = "Heliolisk", ty = (BuiltinType::Electric, Some(BuiltinType::Normal)), from = Species::Helioptile)]
  Heliolisk = 695,
  #[assoc(name = "Tyrunt", ty = (BuiltinType::Rock, Some(BuiltinType::Dragon)), to = &[Species::Tyrantrum])]
  Tyrunt = 696,
  #[assoc(name = "Tyrantrum", ty = (BuiltinType::Rock, Some(BuiltinType::Dragon)), from = Species::Tyrunt)]
  Tyrantrum = 697,
  #[assoc(name = "Amaura", ty = (BuiltinType::Rock, Some(BuiltinType::Ice)), to = &[Species::Aurorus])]
  Amaura = 698,
  #[assoc(name = "Aurorus", ty = (BuiltinType::Rock, Some(BuiltinType::Ice)), from = Species::Amaura)]
  Aurorus = 699,
  #[assoc(name = "Sylveon", ty = (BuiltinType::Fairy, None), from = Species::Eevee)]
  Sylveon = 700,
  #[assoc(name = "Hawlucha", ty = (BuiltinType::Fighting, Some(BuiltinType::Flying)))]
  Hawlucha = 701,
  #[assoc(name = "Dedenne", ty = (BuiltinType::Electric, Some(BuiltinType::Fairy)))]
  Dedenne = 702,
  #[assoc(name = "Carbink", ty = (BuiltinType::Rock, Some(BuiltinType::Fairy)))]
  Carbink = 703,
  #[assoc(name = "Goomy", ty = (BuiltinType::Dragon, None), to = &[Species::Sliggoo])]
  Goomy = 704,
  #[assoc(name = "Sliggoo", ty = (BuiltinType::Dragon, None), from = Species::Goomy, to = &[Species::Goodra])]
  Sliggoo = 705,
  #[assoc(name = "Goodra", ty = (BuiltinType::Dragon, None), from = Species::Sliggoo)]
  Goodra = 706,
  #[assoc(name = "Klefki", ty = (BuiltinType::Steel, Some(BuiltinType::Fairy)))]
  Klefki = 707,
  #[assoc(name = "Phantump", ty = (BuiltinType::Ghost, Some(BuiltinType::Grass)), to = &[Species::Trevenant])]
  Phantump = 708,
  #[assoc(name = "Trevenant", ty = (BuiltinType::Ghost, Some(BuiltinType::Grass)), from = Species::Phantump)]
  Trevenant = 709,
  #[assoc(name = "Pumpkaboo", ty = (BuiltinType::Ghost, Some(BuiltinType::Grass)), to = &[Species::Gourgeist])]
  Pumpkaboo = 710,
  #[assoc(name = "Gourgeist", ty = (BuiltinType::Ghost, Some(BuiltinType::Grass)), from = Species::Pumpkaboo)]
  Gourgeist = 711,
  #[assoc(name = "Bergmite", ty = (BuiltinType::Ice, None), to = &[Species::Avalugg])]
  Bergmite = 712,
  #[assoc(name = "Avalugg", ty = (BuiltinType::Ice, None), from = Species::Bergmite)]
  Avalugg = 713,
  #[assoc(name = "Noibat", ty = (BuiltinType::Flying, Some(BuiltinType::Dragon)), to = &[Species::Noivern])]
  Noibat = 714,
  #[assoc(name = "Noivern", ty = (BuiltinType::Flying, Some(BuiltinType::Dragon)), from = Species::Noibat)]
  Noivern = 715,
  #[assoc(name = "Xerneas", ty = (BuiltinType::Fairy, None))]
  Xerneas = 716,
  #[assoc(name = "Yveltal", ty = (BuiltinType::Dark, Some(BuiltinType::Flying)))]
  Yveltal = 717,
  #[assoc(name = "Zygarde", ty = (BuiltinType::Dragon, Some(BuiltinType::Ground)))]
  Zygarde = 718,
  #[assoc(name = "Diancie", ty = (BuiltinType::Rock, Some(BuiltinType::Fairy)))]
  Diancie = 719,
  #[assoc(name = "Hoopa", ty = (BuiltinType::Psychic, Some(BuiltinType::Ghost)))]
  Hoopa = 720,
  #[assoc(name = "Volcanion", ty = (BuiltinType::Fire, Some(BuiltinType::Water)))]
  Volcanion = 721,
  #[assoc(name = "Rowlet", ty = (BuiltinType::Grass, Some(BuiltinType::Flying)), to = &[Species::Dartrix])]
  Rowlet = 722,
  #[assoc(name = "Dartrix", ty = (BuiltinType::Grass, Some(BuiltinType::Flying)), from = Species::Rowlet, to = &[Species::Decidueye])]
  Dartrix = 723,
  #[assoc(name = "Decidueye", ty = (BuiltinType::Grass, Some(BuiltinType::Ghost)), from = Species::Dartrix)]
  Decidueye = 724,
  #[assoc(name = "Litten", ty = (BuiltinType::Fire, None), to = &[Species::Torracat])]
  Litten = 725,
  #[assoc(name = "Torracat", ty = (BuiltinType::Fire, None), from = Species::Litten, to = &[Species::Incineroar])]
  Torracat = 726,
  #[assoc(name = "Incineroar", ty = (BuiltinType::Fire, Some(BuiltinType::Dark)), from = Species::Torracat)]
  Incineroar = 727,
  #[assoc(name = "Popplio", ty = (BuiltinType::Water, None), to = &[Species::Brionne])]
  Popplio = 728,
  #[assoc(name = "Brionne", ty = (BuiltinType::Water, None), from = Species::Popplio, to = &[Species::Primarina])]
  Brionne = 729,
  #[assoc(name = "Primarina", ty = (BuiltinType::Water, Some(BuiltinType::Fairy)), from = Species::Brionne)]
  Primarina = 730,
  #[assoc(name = "Pikipek", ty = (BuiltinType::Normal, Some(BuiltinType::Flying)), to = &[Species::Trumbeak])]
  Pikipek = 731,
  #[assoc(name = "Trumbeak", ty = (BuiltinType::Normal, Some(BuiltinType::Flying)), from = Species::Pikipek, to = &[Species::Toucannon])]
  Trumbeak = 732,
  #[assoc(name = "Toucannon", ty = (BuiltinType::Normal, Some(BuiltinType::Flying)), from = Species::Trumbeak)]
  Toucannon = 733,
  #[assoc(name = "Yungoos", ty = (BuiltinType::Normal, None), to = &[Species::Gumshoos])]
  Yungoos = 734,
  #[assoc(name = "Gumshoos", ty = (BuiltinType::Normal, None), from = Species::Yungoos)]
  Gumshoos = 735,
  #[assoc(name = "Grubbin", ty = (BuiltinType::Bug, None), to = &[Species::Charjabug])]
  Grubbin = 736,
  #[assoc(name = "Charjabug", ty = (BuiltinType::Bug, Some(BuiltinType::Electric)), from = Species::Grubbin, to = &[Species::Vikavolt])]
  Charjabug = 737,
  #[assoc(name = "Vikavolt", ty = (BuiltinType::Bug, Some(BuiltinType::Electric)), from = Species::Charjabug)]
  Vikavolt = 738,
  #[assoc(name = "Crabrawler", ty = (BuiltinType::Fighting, None), to = &[Species::Crabominable])]
  Crabrawler = 739,
  #[assoc(name = "Crabominable", ty = (BuiltinType::Fighting, Some(BuiltinType::Ice)), from = Species::Crabrawler)]
  Crabominable = 740,
  #[assoc(name = "Oricorio", ty = (BuiltinType::Fire, Some(BuiltinType::Flying)))]
  Oricorio = 741,
  #[assoc(name = "Cutiefly", ty = (BuiltinType::Bug, Some(BuiltinType::Fairy)), to = &[Species::Ribombee])]
  Cutiefly = 742,
  #[assoc(name = "Ribombee", ty = (BuiltinType::Bug, Some(BuiltinType::Fairy)), from = Species::Cutiefly)]
  Ribombee = 743,
  #[assoc(name = "Rockruff", ty = (BuiltinType::Rock, None), to = &[Species::Lycanroc])]
  Rockruff = 744,
  #[assoc(name = "Lycanroc", ty = (BuiltinType::Rock, None), from = Species::Rockruff)]
  Lycanroc = 745,
  #[assoc(name = "Wishiwashi", ty = (BuiltinType::Water, None))]
  Wishiwashi = 746,
  #[assoc(name = "Mareanie", ty = (BuiltinType::Poison, Some(BuiltinType::Water)), to = &[Species::Toxapex])]
  Mareanie = 747,
  #[assoc(name = "Toxapex", ty = (BuiltinType::Poison, Some(BuiltinType::Water)), from = Species::Mareanie)]
  Toxapex = 748,
  #[assoc(name = "Mudbray", ty = (BuiltinType::Ground, None), to = &[Species::Mudsdale])]
  Mudbray = 749,
  #[assoc(name = "Mudsdale", ty = (BuiltinType::Ground, None), from = Species::Mudbray)]
  Mudsdale = 750,
  #[assoc(name = "Dewpider", ty = (BuiltinType::Water, Some(BuiltinType::Bug)), to = &[Species::Araquanid])]
  Dewpider = 751,
  #[assoc(name = "Araquanid", ty = (BuiltinType::Water, Some(BuiltinType::Bug)), from = Species::Dewpider)]
  Araquanid = 752,
  #[assoc(name = "Fomantis", ty = (BuiltinType::Grass, None), to = &[Species::Lurantis])]
  Fomantis = 753,
  #[assoc(name = "Lurantis", ty = (BuiltinType::Grass, None), from = Species::Fomantis)]
  Lurantis = 754,
  #[assoc(name = "Morelull", ty = (BuiltinType::Grass, Some(BuiltinType::Fairy)), to = &[Species::Shiinotic])]
  Morelull = 755,
  #[assoc(name = "Shiinotic", ty = (BuiltinType::Grass, Some(BuiltinType::Fairy)), from = Species::Morelull)]
  Shiinotic = 756,
  #[assoc(name = "Salandit", ty = (BuiltinType::Poison, Some(BuiltinType::Fire)), to = &[Species::Salazzle])]
  Salandit = 757,
  #[assoc(name = "Salazzle", ty = (BuiltinType::Poison, Some(BuiltinType::Fire)), from = Species::Salandit)]
  Salazzle = 758,
  #[assoc(name = "Stufful", ty = (BuiltinType::Normal, Some(BuiltinType::Fighting)), to = &[Species::Bewear])]
  Stufful = 759,
  #[assoc(name = "Bewear", ty = (BuiltinType::Normal, Some(BuiltinType::Fighting)), from = Species::Stufful)]
  Bewear = 760,
  #[assoc(name = "Bounsweet", ty = (BuiltinType::Grass, None), to = &[Species::Steenee])]
  Bounsweet = 761,
  #[assoc(name = "Steenee", ty = (BuiltinType::Grass, None), from = Species::Bounsweet, to = &[Species::Tsareena])]
  Steenee = 762,
  #[assoc(name = "Tsareena", ty = (BuiltinType::Grass, None), from = Species::Steenee)]
  Tsareena = 763,
  #[assoc(name = "Comfey", ty = (BuiltinType::Fairy, None))]
  Comfey = 764,
  #[assoc(name = "Oranguru", ty = (BuiltinType::Normal, Some(BuiltinType::Psychic)))]
  Oranguru = 765,
  #[assoc(name = "Passimian", ty = (BuiltinType::Fighting, None))]
  Passimian = 766,
  #[assoc(name = "Wimpod", ty = (BuiltinType::Bug, Some(BuiltinType::Water)), to = &[Species::Golisopod])]
  Wimpod = 767,
  #[assoc(name = "Golisopod", ty = (BuiltinType::Bug, Some(BuiltinType::Water)), from = Species::Wimpod)]
  Golisopod = 768,
  #[assoc(name = "Sandygast", ty = (BuiltinType::Ghost, Some(BuiltinType::Ground)), to = &[Species::Palossand])]
  Sandygast = 769,
  #[assoc(name = "Palossand", ty = (BuiltinType::Ghost, Some(BuiltinType::Ground)), from = Species::Sandygast)]
  Palossand = 770,
  #[assoc(name = "Pyukumuku", ty = (BuiltinType::Water, None))]
  Pyukumuku = 771,
  #[assoc(name = "Type: Null", ty = (BuiltinType::Normal, None), to = &[Species::Silvally])]
  TypeNull = 772,
  #[assoc(name = "Silvally", ty = (BuiltinType::Normal, None), from = Species::TypeNull)]
  Silvally = 773,
  #[assoc(name = "Minior", ty = (BuiltinType::Rock, Some(BuiltinType::Flying)))]
  Minior = 774,
  #[assoc(name = "Komala", ty = (BuiltinType::Normal, None))]
  Komala = 775,
  #[assoc(name = "Turtonator", ty = (BuiltinType::Fire, Some(BuiltinType::Dragon)))]
  Turtonator = 776,
  #[assoc(name = "Togedemaru", ty = (BuiltinType::Electric, Some(BuiltinType::Steel)))]
  Togedemaru = 777,
  #[assoc(name = "Mimikyu", ty = (BuiltinType::Ghost, Some(BuiltinType::Fairy)))]
  Mimikyu = 778,
  #[assoc(name = "Bruxish", ty = (BuiltinType::Water, Some(BuiltinType::Psychic)))]
  Bruxish = 779,
  #[assoc(name = "Drampa", ty = (BuiltinType::Normal, Some(BuiltinType::Dragon)))]
  Drampa = 780,
  #[assoc(name = "Dhelmise", ty = (BuiltinType::Ghost, Some(BuiltinType::Grass)))]
  Dhelmise = 781,
  #[assoc(name = "Jangmo-o", ty = (BuiltinType::Dragon, None), to = &[Species::HakamoO])]
  JangmoO = 782,
  #[assoc(name = "Hakamo-o", ty = (BuiltinType::Dragon, Some(BuiltinType::Fighting)), from = Species::JangmoO, to = &[Species::KommoO])]
  HakamoO = 783,
  #[assoc(name = "Kommo-o", ty = (BuiltinType::Dragon, Some(BuiltinType::Fighting)), from = Species::HakamoO)]
  KommoO = 784,
  #[assoc(name = "Tapu Koko", ty = (BuiltinType::Electric, Some(BuiltinType::Fairy)))]
  TapuKoko = 785,
  #[assoc(name = "Tapu Lele", ty = (BuiltinType::Psychic, Some(BuiltinType::Fairy)))]
  TapuLele = 786,
  #[assoc(name = "Tapu Bulu", ty = (BuiltinType::Grass, Some(BuiltinType::Fairy)))]
  TapuBulu = 787,
  #[assoc(name = "Tapu Fini", ty = (BuiltinType::Water, Some(BuiltinType::Fairy)))]
  TapuFini = 788,
  #[assoc(name = "Cosmog", ty = (BuiltinType::Psychic, None), to = &[Species::Cosmoem])]
  Cosmog = 789,
  #[assoc(name = "Cosmoem", ty = (BuiltinType::Psychic, None), from = Species::Cosmog, to = &[Species::Solgaleo, Species::Lunala])]
  Cosmoem = 790,
  #[assoc(name = "Solgaleo", ty = (BuiltinType::Psychic, Some(BuiltinType::Steel)), from = Species::Cosmoem)]
  Solgaleo = 791,
  #[assoc(name = "Lunala", ty = (BuiltinType::Psychic, Some(BuiltinType::Ghost)), from = Species::Cosmoem)]
  Lunala = 792,
  #[assoc(name = "Nihilego", ty = (BuiltinType::Rock, Some(BuiltinType::Poison)))]
  Nihilego = 793,
  #[assoc(name = "Buzzwole", ty = (BuiltinType::Bug, Some(BuiltinType::Fighting)))]
  Buzzwole = 794,
  #[assoc(name = "Pheromosa", ty = (BuiltinType::Bug, Some(BuiltinType::Fighting)))]
  Pheromosa = 795,
  #[assoc(name = "Xurkitree", ty = (BuiltinType::Electric, None))]
  Xurkitree = 796,
  #[assoc(name = "Celesteela", ty = (BuiltinType::Steel, Some(BuiltinType::Flying)))]
  Celesteela = 797,
  #[assoc(name = "Kartana", ty = (BuiltinType::Grass, Some(BuiltinType::Steel)))]
  Kartana = 798,
  #[assoc(name = "Guzzlord", ty = (BuiltinType::Dark, Some(BuiltinType::Dragon)))]
  Guzzlord = 799,
  #[assoc(name = "Necrozma", ty = (BuiltinType::Psychic, None))]
  Necrozma = 800,
  #[assoc(name = "Magearna", ty = (BuiltinType::Steel, Some(BuiltinType::Fairy)))]
  Magearna = 801,
  #[assoc(name = "Marshadow", ty = (BuiltinType::Fighting, Some(BuiltinType::Ghost)))]
  Marshadow = 802,
  #[assoc(name = "Poipole", ty = (BuiltinType::Poison, None), to = &[Species::Naganadel])]
  Poipole = 803,
  #[assoc(name = "Naganadel", ty = (BuiltinType::Poison, Some(BuiltinType::Dragon)), from = Species::Poipole)]
  Naganadel = 804,
  #[assoc(name = "Stakataka", ty = (BuiltinType::Rock, Some(BuiltinType::Steel)))]
  Stakataka = 805,
  #[assoc(name = "Blacephalon", ty = (BuiltinType::Fire, Some(BuiltinType::Ghost)))]
  Blacephalon = 806,
  #[assoc(name = "Zeraora", ty = (BuiltinType::Electric, None))]
  Zeraora = 807,
  #[assoc(name = "Meltan", ty = (BuiltinType::Steel, None))]
  Meltan = 808,
  #[assoc(name = "Melmetal", ty = (BuiltinType::Steel, None), from = Species::Meltan)]
  Melmetal = 809,
  #[assoc(name = "Grookey", ty = (BuiltinType::Grass, None), to = &[Species::Thwackey])]
  Grookey = 810,
  #[assoc(name = "Thwackey", ty = (BuiltinType::Grass, None), from = Species::Grookey, to = &[Species::Rillaboom])]
  Thwackey = 811,
  #[assoc(name = "Rillaboom", ty = (BuiltinType::Grass, None), from = Species::Thwackey)]
  Rillaboom = 812,
  #[assoc(name = "Scorbunny", ty = (BuiltinType::Fire, None), to = &[Species::Raboot])]
  Scorbunny = 813,
  #[assoc(name = "Raboot", ty = (BuiltinType::Fire, None), from = Species::Scorbunny, to = &[Species::Cinderace])]
  Raboot = 814,
  #[assoc(name = "Cinderace", ty = (BuiltinType::Fire, None), from = Species::Raboot)]
  Cinderace = 815,
  #[assoc(name = "Sobble", ty = (BuiltinType::Water, None), to = &[Species::Drizzile])]
  Sobble = 816,
  #[assoc(name = "Drizzile", ty = (BuiltinType::Water, None), from = Species::Sobble, to = &[Species::Inteleon])]
  Drizzile = 817,
  #[assoc(name = "Inteleon", ty = (BuiltinType::Water, None), from = Species::Drizzile)]
  Inteleon = 818,
  #[assoc(name = "Skwovet", ty = (BuiltinType::Normal, None), to = &[Species::Greedent])]
  Skwovet = 819,
  #[assoc(name = "Greedent", ty = (BuiltinType::Normal, None), from = Species::Skwovet)]
  Greedent = 820,
  #[assoc(name = "Rookidee", ty = (BuiltinType::Flying, None), to = &[Species::Corvisquire])]
  Rookidee = 821,
  #[assoc(name = "Corvisquire", ty = (BuiltinType::Flying, None), from = Species::Rookidee, to = &[Species::Corviknight])]
  Corvisquire = 822,
  #[assoc(name = "Corviknight", ty = (BuiltinType::Flying, Some(BuiltinType::Steel)), from = Species::Corvisquire)]
  Corviknight = 823,
  #[assoc(name = "Blipbug", ty = (BuiltinType::Bug, None), to = &[Species::Dottler])]
  Blipbug = 824,
  #[assoc(name = "Dottler", ty = (BuiltinType::Bug, Some(BuiltinType::Psychic)), from = Species::Blipbug, to = &[Species::Orbeetle])]
  Dottler = 825,
  #[assoc(name = "Orbeetle", ty = (BuiltinType::Bug, Some(BuiltinType::Psychic)), from = Species::Dottler)]
  Orbeetle = 826,
  #[assoc(name = "Nickit", ty = (BuiltinType::Dark, None), to = &[Species::Thievul])]
  Nickit = 827,
  #[assoc(name = "Thievul", ty = (BuiltinType::Dark, None), from = Species::Nickit)]
  Thievul = 828,
  #[assoc(name = "Gossifleur", ty = (BuiltinType::Grass, None), to = &[Species::Eldegoss])]
  Gossifleur = 829,
  #[assoc(name = "Eldegoss", ty = (BuiltinType::Grass, None), from = Species::Gossifleur)]
  Eldegoss = 830,
  #[assoc(name = "Wooloo", ty = (BuiltinType::Normal, None), to = &[Species::Dubwool])]
  Wooloo = 831,
  #[assoc(name = "Dubwool", ty = (BuiltinType::Normal, None), from = Species::Wooloo)]
  Dubwool = 832,
  #[assoc(name = "Chewtle", ty = (BuiltinType::Water, None), to = &[Species::Drednaw])]
  Chewtle = 833,
  #[assoc(name = "Drednaw", ty = (BuiltinType::Water, Some(BuiltinType::Rock)), from = Species::Chewtle)]
  Drednaw = 834,
  #[assoc(name = "Yamper", ty = (BuiltinType::Electric, None), to = &[Species::Boltund])]
  Yamper = 835,
  #[assoc(name = "Boltund", ty = (BuiltinType::Electric, None), from = Species::Yamper)]
  Boltund = 836,
  #[assoc(name = "Rolycoly", ty = (BuiltinType::Rock, None), to = &[Species::Carkol])]
  Rolycoly = 837,
  #[assoc(name = "Carkol", ty = (BuiltinType::Rock, Some(BuiltinType::Fire)), from = Species::Rolycoly, to = &[Species::Coalossal])]
  Carkol = 838,
  #[assoc(name = "Coalossal", ty = (BuiltinType::Rock, Some(BuiltinType::Fire)), from = Species::Carkol)]
  Coalossal = 839,
  #[assoc(name = "Applin", ty = (BuiltinType::Grass, Some(BuiltinType::Dragon)), to = &[Species::Flapple, Species::Appletun, Species::Dipplin])]
  Applin = 840,
  #[assoc(name = "Flapple", ty = (BuiltinType::Grass, Some(BuiltinType::Dragon)), from = Species::Applin)]
  Flapple = 841,
  #[assoc(name = "Appletun", ty = (BuiltinType::Grass, Some(BuiltinType::Dragon)), from = Species::Applin)]
  Appletun = 842,
  #[assoc(name = "Silicobra", ty = (BuiltinType::Ground, None), to = &[Species::Sandaconda])]
  Silicobra = 843,
  #[assoc(name = "Sandaconda", ty = (BuiltinType::Ground, None), from = Species::Silicobra)]
  Sandaconda = 844,
  #[assoc(name = "Cramorant", ty = (BuiltinType::Flying, Some(BuiltinType::Water)))]
  Cramorant = 845,
  #[assoc(name = "Arrokuda", ty = (BuiltinType::Water, None), to = &[Species::Barraskewda])]
  Arrokuda = 846,
  #[assoc(name = "Barraskewda", ty = (BuiltinType::Water, None), from = Species::Arrokuda)]
  Barraskewda = 847,
  #[assoc(name = "Toxel", ty = (BuiltinType::Electric, Some(BuiltinType::Poison)), to = &[Species::Toxtricity])]
  Toxel = 848,
  #[assoc(name = "Toxtricity", ty = (BuiltinType::Electric, Some(BuiltinType::Poison)), from = Species::Toxel)]
  Toxtricity = 849,
  #[assoc(name = "Sizzlipede", ty = (BuiltinType::Fire, Some(BuiltinType::Bug)), to = &[Species::Centiskorch])]
  Sizzlipede = 850,
  #[assoc(name = "Centiskorch", ty = (BuiltinType::Fire, Some(BuiltinType::Bug)), from = Species::Sizzlipede)]
  Centiskorch = 851,
  #[assoc(name = "Clobbopus", ty = (BuiltinType::Fighting, None), to = &[Species::Grapploct])]
  Clobbopus = 852,
  #[assoc(name = "Grapploct", ty = (BuiltinType::Fighting, None), from = Species::Clobbopus)]
  Grapploct = 853,
  #[assoc(name = "Sinistea", ty = (BuiltinType::Ghost, None), to = &[Species::Polteageist])]
  Sinistea = 854,
  #[assoc(name = "Polteageist", ty = (BuiltinType::Ghost, None), from = Species::Sinistea)]
  Polteageist = 855,
  #[assoc(name = "Hatenna", ty = (BuiltinType::Psychic, None), to = &[Species::Hattrem])]
  Hatenna = 856,
  #[assoc(name = "Hattrem", ty = (BuiltinType::Psychic, None), from = Species::Hatenna, to = &[Species::Hatterene])]
  Hattrem = 857,
  #[assoc(name = "Hatterene", ty = (BuiltinType::Psychic, Some(BuiltinType::Fairy)), from = Species::Hattrem)]
  Hatterene = 858,
  #[assoc(name = "Impidimp", ty = (BuiltinType::Dark, Some(BuiltinType::Fairy)), to = &[Species::Morgrem])]
  Impidimp = 859,
  #[assoc(name = "Morgrem", ty = (BuiltinType::Dark, Some(BuiltinType::Fairy)), from = Species::Impidimp, to = &[Species::Grimmsnarl])]
  Morgrem = 860,
  #[assoc(name = "Grimmsnarl", ty = (BuiltinType::Dark, Some(BuiltinType::Fairy)), from = Species::Morgrem)]
  Grimmsnarl = 861,
  #[assoc(name = "Obstagoon", ty = (BuiltinType::Dark, Some(BuiltinType::Normal)), from = Species::Linoone)]
  Obstagoon = 862,
  #[assoc(name = "Perrserker", ty = (BuiltinType::Steel, None), from = Species::Meowth)]
  Perrserker = 863,
  #[assoc(name = "Cursola", ty = (BuiltinType::Ghost, None), from = Species::Corsola)]
  Cursola = 864,
  #[assoc(name = "Sirfetch’d", ty = (BuiltinType::Fighting, None), from = Species::Farfetchd)]
  Sirfetchd = 865,
  #[assoc(name = "Mr. Rime", ty = (BuiltinType::Ice, Some(BuiltinType::Psychic)), from = Species::MrMime)]
  MrRime = 866,
  #[assoc(name = "Runerigus", ty = (BuiltinType::Ground, Some(BuiltinType::Ghost)), from = Species::Yamask)]
  Runerigus = 867,
  #[assoc(name = "Milcery", ty = (BuiltinType::Fairy, None), to = &[Species::Alcremie])]
  Milcery = 868,
  #[assoc(name = "Alcremie", ty = (BuiltinType::Fairy, None), from = Species::Milcery)]
  Alcremie = 869,
  #[assoc(name = "Falinks", ty = (BuiltinType::Fighting, None))]
  Falinks = 870,
  #[assoc(name = "Pincurchin", ty = (BuiltinType::Electric, None))]
  Pincurchin = 871,
  #[assoc(name = "Snom", ty = (BuiltinType::Ice, Some(BuiltinType::Bug)), to = &[Species::Frosmoth])]
  Snom = 872,
  #[assoc(name = "Frosmoth", ty = (BuiltinType::Ice, Some(BuiltinType::Bug)), from = Species::Snom)]
  Frosmoth = 873,
  #[assoc(name = "Stonjourner", ty = (BuiltinType::Rock, None))]
  Stonjourner = 874,
  #[assoc(name = "Eiscue", ty = (BuiltinType::Ice, None))]
  Eiscue = 875,
  #[assoc(name = "Indeedee", ty = (BuiltinType::Psychic, Some(BuiltinType::Normal)))]
  Indeedee = 876,
  #[assoc(name = "Morpeko", ty = (BuiltinType::Electric, Some(BuiltinType::Dark)))]
  Morpeko = 877,
  #[assoc(name = "Cufant", ty = (BuiltinType::Steel, None), to = &[Species::Copperajah])]
  Cufant = 878,
  #[assoc(name = "Copperajah", ty = (BuiltinType::Steel, None), from = Species::Cufant)]
  Copperajah = 879,
  #[assoc(name = "Dracozolt", ty = (BuiltinType::Electric, Some(BuiltinType::Dragon)))]
  Dracozolt = 880,
  #[assoc(name = "Arctozolt", ty = (BuiltinType::Electric, Some(BuiltinType::Ice)))]
  Arctozolt = 881,
  #[assoc(name = "Dracovish", ty = (BuiltinType::Water, Some(BuiltinType::Dragon)))]
  Dracovish = 882,
  #[assoc(name = "Arctovish", ty = (BuiltinType::Water, Some(BuiltinType::Ice)))]
  Arctovish = 883,
  #[assoc(name = "Duraludon", ty = (BuiltinType::Steel, Some(BuiltinType::Dragon)), to = &[Species::Archaludon])]
  Duraludon = 884,
  #[assoc(name = "Dreepy", ty = (BuiltinType::Dragon, Some(BuiltinType::Ghost)), to = &[Species::Drakloak])]
  Dreepy = 885,
  #[assoc(name = "Drakloak", ty = (BuiltinType::Dragon, Some(BuiltinType::Ghost)), from = Species::Dreepy, to = &[Species::Dragapult])]
  Drakloak = 886,
  #[assoc(name = "Dragapult", ty = (BuiltinType::Dragon, Some(BuiltinType::Ghost)), from = Species::Drakloak)]
  Dragapult = 887,
  #[assoc(name = "Zacian", ty = (BuiltinType::Fairy, None))]
  Zacian = 888,
  #[assoc(name = "Zamazenta", ty = (BuiltinType::Fighting, None))]
  Zamazenta = 889,
  #[assoc(name = "Eternatus", ty = (BuiltinType::Poison, Some(BuiltinType::Dragon)))]
  Eternatus = 890,
  #[assoc(name = "Kubfu", ty = (BuiltinType::Fighting, None), to = &[Species::Urshifu])]
  Kubfu = 891,
  #[assoc(name = "Urshifu", ty = (BuiltinType::Fighting, Some(BuiltinType::Dark)), from = Species::Kubfu)]
  Urshifu = 892,
  #[assoc(name = "Zarude", ty = (BuiltinType::Dark, Some(BuiltinType::Grass)))]
  Zarude = 893,
  #[assoc(name = "Regieleki", ty = (BuiltinType::Electric, None))]
  Regieleki = 894,
  #[assoc(name = "Regidrago", ty = (BuiltinType::Dragon, None))]
  Regidrago = 895,
  #[assoc(name = "Glastrier", ty = (BuiltinType::Ice, None))]
  Glastrier = 896,
  #[assoc(name = "Spectrier", ty = (BuiltinType::Ghost, None))]
  Spectrier = 897,
  #[assoc(name = "Calyrex", ty = (BuiltinType::Psychic, Some(BuiltinType::Grass)))]
  Calyrex = 898,
  #[assoc(name = "Wyrdeer", ty = (BuiltinType::Normal, Some(BuiltinType::Psychic)), from = Species::Stantler)]
  Wyrdeer = 899,
  #[assoc(name = "Kleavor", ty = (BuiltinType::Bug, Some(BuiltinType::Rock)), from = Species::Scyther)]
  Kleavor = 900,
  #[assoc(name = "Ursaluna", ty = (BuiltinType::Ground, Some(BuiltinType::Normal)), from = Species::Ursaring)]
  Ursaluna = 901,
  #[assoc(name = "Basculegion", ty = (BuiltinType::Water, Some(BuiltinType::Ghost)), from = Species::Basculin)]
  Basculegion = 902,
  #[assoc(name = "Sneasler", ty = (BuiltinType::Fighting, Some(BuiltinType::Poison)), from = Species::Sneasel)]
  Sneasler = 903,
  #[assoc(name = "Overqwil", ty = (BuiltinType::Dark, Some(BuiltinType::Poison)), from = Species::Qwilfish)]
  Overqwil = 904,
  #[assoc(name = "Enamorus", ty = (BuiltinType::Fairy, Some(BuiltinType::Flying)))]
  Enamorus = 905,
  #[assoc(name = "Sprigatito", ty = (BuiltinType::Grass, None), to = &[Species::Floragato])]
  Sprigatito = 906,
  #[assoc(name = "Floragato", ty = (BuiltinType::Grass, None), from = Species::Sprigatito, to = &[Species::Meowscarada])]
  Floragato = 907,
  #[assoc(name = "Meowscarada", ty = (BuiltinType::Grass, Some(BuiltinType::Dark)), from = Species::Floragato)]
  Meowscarada = 908,
  #[assoc(name = "Fuecoco", ty = (BuiltinType::Fire, None), to = &[Species::Crocalor])]
  Fuecoco = 909,
  #[assoc(name = "Crocalor", ty = (BuiltinType::Fire, None), from = Species::Fuecoco, to = &[Species::Skeledirge])]
  Crocalor = 910,
  #[assoc(name = "Skeledirge", ty = (BuiltinType::Fire, Some(BuiltinType::Ghost)), from = Species::Crocalor)]
  Skeledirge = 911,
  #[assoc(name = "Quaxly", ty = (BuiltinType::Water, None), to = &[Species::Quaxwell])]
  Quaxly = 912,
  #[assoc(name = "Quaxwell", ty = (BuiltinType::Water, None), from = Species::Quaxly, to = &[Species::Quaquaval])]
  Quaxwell = 913,
  #[assoc(name = "Quaquaval", ty = (BuiltinType::Water, Some(BuiltinType::Fighting)), from = Species::Quaxwell)]
  Quaquaval = 914,
  #[assoc(name = "Lechonk", ty = (BuiltinType::Normal, None), to = &[Species::Oinkologne])]
  Lechonk = 915,
  #[assoc(name = "Oinkologne", ty = (BuiltinType::Normal, None), from = Species::Lechonk)]
  Oinkologne = 916,
  #[assoc(name = "Tarountula", ty = (BuiltinType::Bug, None), to = &[Species::Spidops])]
  Tarountula = 917,
  #[assoc(name = "Spidops", ty = (BuiltinType::Bug, None), from = Species::Tarountula)]
  Spidops = 918,
  #[assoc(name = "Nymble", ty = (BuiltinType::Bug, None), to = &[Species::Lokix])]
  Nymble = 919,
  #[assoc(name = "Lokix", ty = (BuiltinType::Bug, Some(BuiltinType::Dark)), from = Species::Nymble)]
  Lokix = 920,
  #[assoc(name = "Pawmi", ty = (BuiltinType::Electric, None), to = &[Species::Pawmo])]
  Pawmi = 921,
  #[assoc(name = "Pawmo", ty = (BuiltinType::Electric, Some(BuiltinType::Fighting)), from = Species::Pawmi, to = &[Species::Pawmot])]
  Pawmo = 922,
  #[assoc(name = "Pawmot", ty = (BuiltinType::Electric, Some(BuiltinType::Fighting)), from = Species::Pawmo)]
  Pawmot = 923,
  #[assoc(name = "Tandemaus", ty = (BuiltinType::Normal, None), to = &[Species::Maushold])]
  Tandemaus = 924,
  #[assoc(name = "Maushold", ty = (BuiltinType::Normal, None), from = Species::Tandemaus)]
  Maushold = 925,
  #[assoc(name = "Fidough", ty = (BuiltinType::Fairy, None), to = &[Species::Dachsbun])]
  Fidough = 926,
  #[assoc(name = "Dachsbun", ty = (BuiltinType::Fairy, None), from = Species::Fidough)]
  Dachsbun = 927,
  #[assoc(name = "Smoliv", ty = (BuiltinType::Grass, Some(BuiltinType::Normal)), to = &[Species::Dolliv])]
  Smoliv = 928,
  #[assoc(name = "Dolliv", ty = (BuiltinType::Grass, Some(BuiltinType::Normal)), from = Species::Smoliv, to = &[Species::Arboliva])]
  Dolliv = 929,
  #[assoc(name = "Arboliva", ty = (BuiltinType::Grass, Some(BuiltinType::Normal)), from = Species::Dolliv)]
  Arboliva = 930,
  #[assoc(name = "Squawkabilly", ty = (BuiltinType::Normal, Some(BuiltinType::Flying)))]
  Squawkabilly = 931,
  #[assoc(name = "Nacli", ty = (BuiltinType::Rock, None), to = &[Species::Naclstack])]
  Nacli = 932,
  #[assoc(name = "Naclstack", ty = (BuiltinType::Rock, None), from = Species::Nacli, to = &[Species::Garganacl])]
  Naclstack = 933,
  #[assoc(name = "Garganacl", ty = (BuiltinType::Rock, None), from = Species::Naclstack)]
  Garganacl = 934,
  #[assoc(name = "Charcadet", ty = (BuiltinType::Fire, None), to = &[Species::Armarouge, Species::Ceruledge])]
  Charcadet = 935,
  #[assoc(name = "Armarouge", ty = (BuiltinType::Fire, Some(BuiltinType::Psychic)), from = Species::Charcadet)]
  Armarouge = 936,
  #[assoc(name = "Ceruledge", ty = (BuiltinType::Fire, Some(BuiltinType::Ghost)), from = Species::Charcadet)]
  Ceruledge = 937,
  #[assoc(name = "Tadbulb", ty = (BuiltinType::Electric, None), to = &[Species::Bellibolt])]
  Tadbulb = 938,
  #[assoc(name = "Bellibolt", ty = (BuiltinType::Electric, None), from = Species::Tadbulb)]
  Bellibolt = 939,
  #[assoc(name = "Wattrel", ty = (BuiltinType::Electric, Some(BuiltinType::Flying)), to = &[Species::Kilowattrel])]
  Wattrel = 940,
  #[assoc(name = "Kilowattrel", ty = (BuiltinType::Electric, Some(BuiltinType::Flying)), from = Species::Wattrel)]
  Kilowattrel = 941,
  #[assoc(name = "Maschiff", ty = (BuiltinType::Dark, None), to = &[Species::Mabosstiff])]
  Maschiff = 942,
  #[assoc(name = "Mabosstiff", ty = (BuiltinType::Dark, None), from = Species::Maschiff)]
  Mabosstiff = 943,
  #[assoc(name = "Shroodle", ty = (BuiltinType::Poison, Some(BuiltinType::Normal)), to = &[Species::Grafaiai])]
  Shroodle = 944,
  #[assoc(name = "Grafaiai", ty = (BuiltinType::Poison, Some(BuiltinType::Normal)), from = Species::Shroodle)]
  Grafaiai = 945,
  #[assoc(name = "Bramblin", ty = (BuiltinType::Grass, Some(BuiltinType::Ghost)), to = &[Species::Brambleghast])]
  Bramblin = 946,
  #[assoc(name = "Brambleghast", ty = (BuiltinType::Grass, Some(BuiltinType::Ghost)), from = Species::Bramblin)]
  Brambleghast = 947,
  #[assoc(name = "Toedscool", ty = (BuiltinType::Ground, Some(BuiltinType::Grass)), to = &[Species::Toedscruel])]
  Toedscool = 948,
  #[assoc(name = "Toedscruel", ty = (BuiltinType::Ground, Some(BuiltinType::Grass)), from = Species::Toedscool)]
  Toedscruel = 949,
  #[assoc(name = "Klawf", ty = (BuiltinType::Rock, None))]
  Klawf = 950,
  #[assoc(name = "Capsakid", ty = (BuiltinType::Grass, None), to = &[Species::Scovillain])]
  Capsakid = 951,
  #[assoc(name = "Scovillain", ty = (BuiltinType::Grass, Some(BuiltinType::Fire)), from = Species::Capsakid)]
  Scovillain = 952,
  #[assoc(name = "Rellor", ty = (BuiltinType::Bug, None), to = &[Species::Rabsca])]
  Rellor = 953,
  #[assoc(name = "Rabsca", ty = (BuiltinType::Bug, Some(BuiltinType::Psychic)), from = Species::Rellor)]
  Rabsca = 954,
  #[assoc(name = "Flittle", ty = (BuiltinType::Psychic, None), to = &[Species::Espathra])]
  Flittle = 955,
  #[assoc(name = "Espathra", ty = (BuiltinType::Psychic, None), from = Species::Flittle)]
  Espathra = 956,
  #[assoc(name = "Tinkatink", ty = (BuiltinType::Fairy, Some(BuiltinType::Steel)), to = &[Species::Tinkatuff])]
  Tinkatink = 957,
  #[assoc(name = "Tinkatuff", ty = (BuiltinType::Fairy, Some(BuiltinType::Steel)), from = Species::Tinkatink, to = &[Species::Tinkaton])]
  Tinkatuff = 958,
  #[assoc(name = "Tinkaton", ty = (BuiltinType::Fairy, Some(BuiltinType::Steel)), from = Species::Tinkatuff)]
  Tinkaton = 959,
  #[assoc(name = "Wiglett", ty = (BuiltinType::Water, None), to = &[Species::Wugtrio])]
  Wiglett = 960,
  #[assoc(name = "Wugtrio", ty = (BuiltinType::Water, None), from = Species::Wiglett)]
  Wugtrio = 961,
  #[assoc(name = "Bombirdier", ty = (BuiltinType::Flying, Some(BuiltinType::Dark)))]
  Bombirdier = 962,
  #[assoc(name = "Finizen", ty = (BuiltinType::Water, None), to = &[Species::Palafin])]
  Finizen = 963,
  #[assoc(name = "Palafin", ty = (BuiltinType::Water, None), from = Species::Finizen)]
  Palafin = 964,
  #[assoc(name = "Varoom", ty = (BuiltinType::Steel, Some(BuiltinType::Poison)), to = &[Species::Revavroom])]
  Varoom = 965,
  #[assoc(name = "Revavroom", ty = (BuiltinType::Steel, Some(BuiltinType::Poison)), from = Species::Varoom)]
  Revavroom = 966,
  #[assoc(name = "Cyclizar", ty = (BuiltinType::Dragon, Some(BuiltinType::Normal)))]
  Cyclizar = 967,
  #[assoc(name = "Orthworm", ty = (BuiltinType::Steel, None))]
  Orthworm = 968,
  #[assoc(name = "Glimmet", ty = (BuiltinType::Rock, Some(BuiltinType::Poison)), to = &[Species::Glimmora])]
  Glimmet = 969,
  #[assoc(name = "Glimmora", ty = (BuiltinType::Rock, Some(BuiltinType::Poison)), from = Species::Glimmet)]
  Glimmora = 970,
  #[assoc(name = "Greavard", ty = (BuiltinType::Ghost, None), to = &[Species::Houndstone])]
  Greavard = 971,
  #[assoc(name = "Houndstone", ty = (BuiltinType::Ghost, None), from = Species::Greavard)]
  Houndstone = 972,
  #[assoc(name = "Flamigo", ty = (BuiltinType::Flying, Some(BuiltinType::Fighting)))]
  Flamigo = 973,
  #[assoc(name = "Cetoddle", ty = (BuiltinType::Ice, None), to = &[Species::Cetitan])]
  Cetoddle = 974,
  #[assoc(name = "Cetitan", ty = (BuiltinType::Ice, None), from = Species::Cetoddle)]
  Cetitan = 975,
  #[assoc(name = "Veluza", ty = (BuiltinType::Water, Some(BuiltinType::Psychic)))]
  Veluza = 976,
  #[assoc(name = "Dondozo", ty = (BuiltinType::Water, None))]
  Dondozo = 977,
  #[assoc(name = "Tatsugiri", ty = (BuiltinType::Dragon, Some(BuiltinType::Water)))]
  Tatsugiri = 978,
  #[assoc(name = "Annihilape", ty = (BuiltinType::Fighting, Some(BuiltinType::Ghost)), from = Species::Primeape)]
  Annihilape = 979,
  #[assoc(name = "Clodsire", ty = (BuiltinType::Poison, Some(BuiltinType::Ground)), from = Species::Wooper)]
  Clodsire = 980,
  #[assoc(name = "Farigiraf", ty = (BuiltinType::Normal, Some(BuiltinType::Psychic)), from = Species::Girafarig)]
  Farigiraf = 981,
  #[assoc(name = "Dudunsparce", ty = (BuiltinType::Normal, None), from = Species::Dunsparce)]
  Dudunsparce = 982,
  #[assoc(name = "Kingambit", ty = (BuiltinType::Dark, Some(BuiltinType::Steel)), from = Species::Bisharp)]
  Kingambit = 983,
  #[assoc(name = "Great Tusk", ty = (BuiltinType::Ground, Some(BuiltinType::Fighting)))]
  GreatTusk = 984,
  #[assoc(name = "Scream Tail", ty = (BuiltinType::Fairy, Some(BuiltinType::Psychic)))]
  ScreamTail = 985,
  #[assoc(name = "Brute Bonnet", ty = (BuiltinType::Grass, Some(BuiltinType::Dark)))]
  BruteBonnet = 986,
  #[assoc(name = "Flutter Mane", ty = (BuiltinType::Ghost, Some(BuiltinType::Fairy)))]
  FlutterMane = 987,
  #[assoc(name = "Slither Wing", ty = (BuiltinType::Bug, Some(BuiltinType::Fighting)))]
  SlitherWing = 988,
  #[assoc(name = "Sandy Shocks", ty = (BuiltinType::Electric, Some(BuiltinType::Ground)))]
  SandyShocks = 989,
  #[assoc(name = "Iron Treads", ty = (BuiltinType::Ground, Some(BuiltinType::Steel)))]
  IronTreads = 990,
  #[assoc(name = "Iron Bundle", ty = (BuiltinType::Ice, Some(BuiltinType::Water)))]
  IronBundle = 991,
  #[assoc(name = "Iron Hands", ty = (BuiltinType::Fighting, Some(BuiltinType::Electric)))]
  IronHands = 992,
  #[assoc(name = "Iron Jugulis", ty = (BuiltinType::Dark, Some(BuiltinType::Flying)))]
  IronJugulis = 993,
  #[assoc(name = "Iron Moth", ty = (BuiltinType::Fire, Some(BuiltinType::Poison)))]
  IronMoth = 994,
  #[assoc(name = "Iron Thorns", ty = (BuiltinType::Rock, Some(BuiltinType::Electric)))]
  IronThorns = 995,
  #[assoc(name = "Frigibax", ty = (BuiltinType::Dragon, Some(BuiltinType::Ice)), to = &[Species::Arctibax])]
  Frigibax = 996,
  #[assoc(name = "Arctibax", ty = (BuiltinType::Dragon, Some(BuiltinType::Ice)), from = Species::Frigibax, to = &[Species::Baxcalibur])]
  Arctibax = 997,
  #[assoc(name = "Baxcalibur", ty = (BuiltinType::Dragon, Some(BuiltinType::Ice)), from = Species::Arctibax)]
  Baxcalibur = 998,
  #[assoc(name = "Gimmighoul", ty = (BuiltinType::Ghost, None), to = &[Species::Gholdengo])]
  Gimmighoul = 999,
  #[assoc(name = "Gholdengo", ty = (BuiltinType::Steel, Some(BuiltinType::Ghost)), from = Species::Gimmighoul)]
  Gholdengo = 1000,
  #[assoc(name = "Wo-Chien", ty = (BuiltinType::Dark, Some(BuiltinType::Grass)))]
  WoChien = 1001,
  #[assoc(name = "Chien-Pao", ty = (BuiltinType::Dark, Some(BuiltinType::Ice)))]
  ChienPao = 1002,
  #[assoc(name = "Ting-Lu", ty = (BuiltinType::Dark, Some(BuiltinType::Ground)))]
  TingLu = 1003,
  #[assoc(name = "Chi-Yu", ty = (BuiltinType::Dark, Some(BuiltinType::Fire)))]
  ChiYu = 1004,
  #[assoc(name = "Roaring Moon", ty = (BuiltinType::Dragon, Some(BuiltinType::Dark)))]
  RoaringMoon = 1005,
  #[assoc(name = "Iron Valiant", ty = (BuiltinType::Fairy, Some(BuiltinType::Fighting)))]
  IronValiant = 1006,
  #[assoc(name = "Koraidon", ty = (BuiltinType::Fighting, Some(BuiltinType::Dragon)))]
  Koraidon = 1007,
  #[assoc(name = "Miraidon", ty = (BuiltinType::Electric, Some(BuiltinType::Dragon)))]
  Miraidon = 1008,
  #[assoc(name = "Walking Wake", ty = (BuiltinType::Water, Some(BuiltinType::Dragon)))]
  WalkingWake = 1009,
  #[assoc(name = "Iron Leaves", ty = (BuiltinType::Grass, Some(BuiltinType::Psychic)))]
  IronLeaves = 1010,
  #[assoc(name = "Dipplin", ty = (BuiltinType::Grass, Some(BuiltinType::Dragon)), from = Species::Applin, to = &[Species::Hydrapple])]
  Dipplin = 1011,
  #[assoc(name = "Poltchageist", ty = (BuiltinType::Grass, Some(BuiltinType::Ghost)), to = &[Species::Sinistcha])]
  Poltchageist = 1012,
  #[assoc(name = "Sinistcha", ty = (BuiltinType::Grass, Some(BuiltinType::Ghost)), from = Species::Poltchageist)]
  Sinistcha = 1013,
  #[assoc(name = "Okidogi", ty = (BuiltinType::Poison, Some(BuiltinType::Fighting)))]
  Okidogi = 1014,
  #[assoc(name = "Munkidori", ty = (BuiltinType::Poison, Some(BuiltinType::Psychic)))]
  Munkidori = 1015,
  #[assoc(name = "Fezandipiti", ty = (BuiltinType::Poison, Some(BuiltinType::Fairy)))]
  Fezandipiti = 1016,
  #[assoc(name = "Ogerpon", ty = (BuiltinType::Grass, None))]
  Ogerpon = 1017,
  #[assoc(name = "Archaludon", ty = (BuiltinType::Steel, Some(BuiltinType::Dragon)), from = Species::Duraludon)]
  Archaludon = 1018,
  #[assoc(name = "Hydrapple", ty = (BuiltinType::Grass, Some(BuiltinType::Dragon)), from = Species::Dipplin)]
  Hydrapple = 1019,
  #[assoc(name = "Gouging Fire", ty = (BuiltinType::Fire, Some(BuiltinType::Dragon)))]
  GougingFire = 1020,
  #[assoc(name = "Raging Bolt", ty = (BuiltinType::Electric, Some(BuiltinType::Dragon)))]
  RagingBolt = 1021,
  #[assoc(name = "Iron Boulder", ty = (BuiltinType::Rock, Some(BuiltinType::Psychic)))]
  IronBoulder = 1022,
  #[assoc(name = "Iron Crown", ty = (BuiltinType::Steel, Some(BuiltinType::Psychic)))]
  IronCrown = 1023,
  #[assoc(name = "Terapagos", ty = (BuiltinType::Normal, None))]
  Terapagos = 1024,
  #[assoc(name = "Pecharunt", ty = (BuiltinType::Poison, Some(BuiltinType::Ghost)))]
  Pecharunt = 1025,
}
