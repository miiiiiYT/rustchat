use core::fmt;

pub struct Identification {
	pub id: u64,
	pub name: String,
	pub helo: bool
}

impl Identification {
    pub fn new(id: u64, name: String, first: bool) -> Self {
        Self { id, name, helo: first }
    }

	pub fn from(ident_string: String) -> Option<Self> {
		let ident: Vec<&str> = ident_string.split(';').collect();

		if let [id, name, greeting] = ident.as_slice() {
			if greeting == &"HELO" && id.parse::<u64>().is_ok() && check_username(name) {
				let id = id.parse::<u64>().ok()?;
				Some(Self { id: id, name: name.to_string(), helo: true })
			} else if id.parse::<u64>().is_ok() && check_username(name) {
				let id = id.parse::<u64>().ok()?;
				Some(Self { id: id, name: name.to_string(), helo: false })
			} else {
				None
			}
		} else if let [id, name] = ident.as_slice() {
			if id.parse::<u64>().is_ok() && check_username(name) {
				let id = id.parse::<u64>().ok()?;
				Some(Self { id: id, name: name.to_string(), helo: false})
			} else {
				None
			}
		} else {
			None
		}
	}

	pub fn to_ident_string(&self) -> Vec<u8> {
		let mut ident_string_vec = vec![self.id.to_string(), self.name.clone(), if self.helo {"HELO".to_string()} else {"".to_string()}];

		if ident_string_vec.last().map_or(false, |s| s.is_empty()) {
			ident_string_vec.pop();
		}

		let ident_string: String = ident_string_vec.join(";");

		ident_string.into_bytes()
	}
}

impl fmt::Display for Identification {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{};{}{}", self.id.to_string(), self.name, if self.helo {";HELO".to_string()} else {"".to_string()})
	}
}

fn check_username(username: &str) -> bool {
	username.chars().all(|c| c.is_ascii_alphanumeric())
}