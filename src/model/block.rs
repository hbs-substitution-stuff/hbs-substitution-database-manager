use std::error::Error;

#[derive(Clone, Copy)]
pub enum Block {
	Zero = 0,
	One = 1,
	Two = 2,
	Three = 3,
	Four = 4,
	Five = 5,
}

impl TryFrom<i64> for Block {
	type Error = Box<dyn Error>;

	fn try_from(value: i64) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::Zero),
			1 => Ok(Self::One),
			2 => Ok(Self::Two),
			3 => Ok(Self::Three),
			4 => Ok(Self::Four),
			5 => Ok(Self::Five),
			_ => Err("out of range".into()),
		}
	}
}