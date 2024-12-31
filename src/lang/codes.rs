use citationberg::LocaleCode;
pub fn get_mapping(lang_string: &str) -> Option<LocaleCode> {
	match lang_string {
		"english" => Some(LocaleCode("en".parse().unwrap())),
		"russian" => Some(LocaleCode("ru".parse().unwrap())),
		// you can add your language here
		_ => None
	}
}