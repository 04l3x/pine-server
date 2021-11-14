use async_graphql::SimpleObject;

#[derive(Clone, Debug, SimpleObject)]
pub struct Info {
	count: i32,
	pages: i32,
	prev: Option<i32>,
	next: Option<i32>,
}

impl Default for Info {
	fn default() -> Self {
		Info {
			count: 0,
			pages: 0,
			prev: None,
			next: None,
		}
	}
}

pub struct InfoBuilder {
	info: Info,
}

impl InfoBuilder {
	pub fn new() -> InfoBuilder {
		InfoBuilder {
			info: Info::default(),
		}
	}

	pub fn set_count(&mut self, count: i32) {
		self.info.count = count;
	}

	pub fn set_pages(&mut self, pages: i32) {
		self.info.pages = pages;
	}

	pub fn set_prev(&mut self, prev: Option<i32>) {
		self.info.prev = prev;
	}

	pub fn set_next(&mut self, next: Option<i32>) {
		self.info.next = next;
	}

	pub fn set_values(&mut self, count: i32, pages: i32, prev: Option<i32>, next: Option<i32>) {
		self.set_count(count);
		self.set_pages(pages);
		self.set_prev(prev);
		self.set_next(next);
	}

	pub fn build(self) -> Info {
		self.info
	}
}
