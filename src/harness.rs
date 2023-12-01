use criterion::Criterion;
use reqwest::{blocking::ClientBuilder, cookie};

use std::{
    alloc::{alloc, dealloc, handle_alloc_error, Layout},
    env,
    fmt::Debug,
    fs,
    ptr::NonNull,
    sync::Arc,
};

pub struct Harness {
    day: Option<u32>,
    input_blob: Option<(NonNull<u8>, usize)>,
    criterion: Option<Criterion>,
}

impl Drop for Harness {
    fn drop(&mut self) {
        self.free_input_blob();
    }
}

impl Harness {
    fn input_layout(len: usize) -> Layout {
        let align = 64;
        let size = (len + align - 1) & !(align - 1);
        Layout::from_size_align(size, align).unwrap()
    }

    fn free_input_blob(&mut self) {
        if let Some((ptr, len)) = self.input_blob.take() {
            unsafe { dealloc(ptr.as_ptr(), Self::input_layout(len)) }
        }
    }

    fn create_input_blob(data: &[u8]) -> (NonNull<u8>, usize) {
        assert!(!data.is_empty());

        let layout = Self::input_layout(data.len());
        let ptr = unsafe { alloc(layout) };
        let ptr = NonNull::new(ptr).unwrap_or_else(|| handle_alloc_error(layout));
        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr(), ptr.as_ptr(), data.len());
        }
        (ptr, data.len())
    }
}

impl Harness {
    pub fn new(bench: bool) -> Self {
        let criterion = bench.then(|| Criterion::default().with_output_color(true));

        Self {
            day: None,
            input_blob: None,
            criterion,
        }
    }

    pub fn begin(&mut self, day: u32) -> &mut Self {
        self.day = Some(day);
        self.free_input_blob();
        self
    }

    #[allow(dead_code)]
    pub fn input_override<I: AsRef<str>>(&mut self, input_override: I) -> &mut Self {
        self.free_input_blob();
        self.input_blob = Some(Self::create_input_blob(input_override.as_ref().as_bytes()));
        self
    }

    pub fn run_part<R>(&mut self, part_num: u32, part_func: fn(&str) -> R) -> &mut Self
    where
        R: Debug,
    {
        if self.input_blob.is_none() {
            let input_path = format!("inputs/{}.txt", self.day.unwrap());
            fs::create_dir_all("inputs").unwrap();
            let text = fs::read_to_string(&input_path).unwrap_or_else(|_| {
                let text = download_input(self.day.unwrap());
                fs::write(&input_path, &text).unwrap();
                text
            });
            self.input_blob = Some(Self::create_input_blob(text.as_bytes()));
        }

        let input_text = unsafe {
            let (ptr, size) = self.input_blob.unwrap();
            // SAFETY: utf8 validity is enforced when loading the input
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr.as_ptr(), size))
        };

        let res = part_func(input_text);
        println!("Part {}: {:?}", part_num, res);

        if let Some(criterion) = self.criterion.as_mut() {
            criterion.bench_function(
                &format!("day {} part {}", self.day.unwrap(), part_num),
                |b| b.iter(|| part_func(input_text)),
            );
        }

        self
    }
}

fn download_input(day: u32) -> String {
    const YEAR: u32 = 2023;

    let jar = Arc::new(cookie::Jar::default());
    let session =
        env::var("AOC_SESSION").expect("`AOC_SESSION` needs to be set when downloading inputs");
    jar.add_cookie_str(
        &format!("session={}", session),
        &"https://adventofcode.com".parse().unwrap(),
    );
    let client = ClientBuilder::new()
        .cookie_provider(jar)
        .gzip(true)
        .brotli(true)
        .build()
        .unwrap();

    client
        .get(format!(
            "https://adventofcode.com/{}/day/{}/input",
            YEAR, day
        ))
        .send()
        .unwrap()
        .error_for_status()
        .unwrap()
        .text()
        .unwrap()
}
