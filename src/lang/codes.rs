use citationberg::LocaleCode;
pub fn get_mapping(lang_string: &str) -> Option<LocaleCode> {
	match lang_string {
		"english"     => Some(LocaleCode(String::from("en"))),
		"russian"     => Some(LocaleCode(String::from("ru"))),
		"abkhazian"   => Some(LocaleCode(String::from("ab"))),
		"belarussian" => Some(LocaleCode(String::from("be"))),
		"bashkir"     => Some(LocaleCode(String::from("ba"))),
		"chinese"     => Some(LocaleCode(String::from("zh"))),
		"chuvash"     => Some(LocaleCode(String::from("cv"))),
		"moldavian"   => Some(LocaleCode(String::from("mo"))),
		"ossetian"    => Some(LocaleCode(String::from("os"))),
		"serbian"     => Some(LocaleCode(String::from("sr"))),
		"tatar"       => Some(LocaleCode(String::from("tt"))),
		// you can add your language here
		_ => None
	}
}