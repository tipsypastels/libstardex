require "json"

def to_enum_member_name(key)
  key.split("-").map(&:capitalize).join
end

def to_species_enum_member_name(key)
  "Species::#{to_enum_member_name(key)}"
end

def to_type_enum_member_name(key)
  "BuiltinType::#{to_enum_member_name(key)}"
end

def to_types_tuple(types)
  types = types.map(&method(:to_type_enum_member_name))

  case types.length
  when 1
    "(#{types[0]}, None)"
  when 2
    "(#{types[0]}, Some(#{types[1]}))"
  end
end

OUT_PATH = "./src/species.gen.rs"
INDENT = "  "

ENUM_HEADER = <<~RUST
  #[repr(u16)]
  #[derive(Assoc, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
  #[func(pub const fn name(self) -> &'static str)]
  #[func(pub(crate) const fn ty(self) -> (BuiltinType, Option<BuiltinType>))]
  #[func(pub(crate) const fn from(self) -> Option<Species>)]
  #[func(pub(crate) const fn to(self) -> Option<&'static [Species]>)]
  #[rustfmt::skip]
  pub enum Species {
RUST

ENUM_FOOTER = "}"

data = JSON.load_file! "./species.json"

File.open "./src/species.gen.rs", "w" do |file|
  file << ENUM_HEADER

  data.each do |key, species|
    member_name = to_enum_member_name(key)

    id = species["id"]
    name = species["name"]
    ty = to_types_tuple(species["type"])

    file << "#{INDENT}#[assoc("
    file << "name = \"#{name}\""
    file << ", ty = #{ty}"

    if (evos = species["evos"])
      if (from = evos["from"])
        file << ", from = #{to_species_enum_member_name(from)}"
      end

      if (to = evos["to"])
        file << ", to = &[#{to.map(&method(:to_species_enum_member_name)).join(", ")}]"
      end
    end

    file << ")]\n"
    file << "#{INDENT}#{member_name} = #{id},\n"
  end

  file << ENUM_FOOTER
end
