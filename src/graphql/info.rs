use async_graphql::SimpleObject;

#[derive(Clone, Debug, SimpleObject)]
pub struct Info {
	count: usize,
	pages: usize,
	prev: Option<usize>,
	next: Option<usize>,
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
	pub fn info(self, count: usize, current_page: usize, page_size: usize) -> Info {
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

	fn set_count(&mut self, count: usize) {
		self.info.count = count;
	}

	fn set_pages(&mut self, pages: usize) {
		self.info.pages = pages;
	}

	fn set_prev(&mut self, prev: Option<usize>) {
		self.info.prev = prev;
	}

	fn set_next(&mut self, next: Option<usize>) {
		self.info.next = next;
	}

	fn set_values(&mut self, count: usize, pages: usize, prev: Option<usize>, next: Option<usize>) {
		self.set_count(count);
		self.set_pages(pages);
		self.set_prev(prev);
		self.set_next(next);
	}

	fn build(self) -> Info {
		self.info
	}
}
