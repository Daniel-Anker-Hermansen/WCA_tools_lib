use std::ops::{Deref, DerefMut};

use crate::{Error, OAuth, WcifContainer};

pub struct WcifOAuth {
	pub(crate) cont: WcifContainer,
	pub(crate) oauth: OAuth,
}

impl Deref for WcifOAuth {
	type Target = WcifContainer;

	fn deref(&self) -> &Self::Target {
		&self.cont
	}
}

impl DerefMut for WcifOAuth {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.cont
	}
}

impl WcifOAuth {
	pub async fn patch(&self) -> core::result::Result::<String, Error> {
		self.cont.patch(&self.oauth).await
	}

	pub fn disassemble(self) -> (WcifContainer, OAuth) {
		(self.cont, self.oauth)
	}
}
