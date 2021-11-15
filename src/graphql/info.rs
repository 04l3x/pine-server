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

pub struct InfoFactory;

impl Default for InfoFactory {
	fn default() -> Self {
		Self {}
	}
}

impl InfoFactory {
	pub fn info(self, count: i32, current_page: i32, page_size: i32) -> Info {
		let pages = if count % page_size == 0 {
			count / page_size
		} else {
			(count / page_size) + 1
		};

		let prev = if current_page == 1 {
			None
		} else {
			Some(current_page - 1)
		};

		let next = if current_page == pages || pages <= 1 {
			None
		} else {
			Some(current_page + 1)
		};

		let mut builder = InfoBuilder::new();
		builder.set_values(count, pages, prev, next);

		builder.build()
	}
}

struct InfoBuilder {
	info: Info,
}

impl InfoBuilder {
	fn new() -> InfoBuilder {
		InfoBuilder {
			info: Info::default(),
		}
	}

	fn set_count(&mut self, count: i32) {
		self.info.count = count;
	}

	fn set_pages(&mut self, pages: i32) {
		self.info.pages = pages;
	}

	fn set_prev(&mut self, prev: Option<i32>) {
		self.info.prev = prev;
	}

	fn set_next(&mut self, next: Option<i32>) {
		self.info.next = next;
	}

	fn set_values(&mut self, count: i32, pages: i32, prev: Option<i32>, next: Option<i32>) {
		self.set_count(count);
		self.set_pages(pages);
		self.set_prev(prev);
		self.set_next(next);
	}

	fn build(self) -> Info {
		self.info
	}
}
