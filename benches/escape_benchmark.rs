use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use lsd::color::{self, Colors};
use lsd::flags::HyperlinkOption;
use lsd::icon::{self, Icons};
use lsd::meta::filetype::FileType;
use lsd::meta::{
    name::{DisplayOption, Name},
    permissions::Permissions,
};
use std::borrow::Cow;
use std::fs::File;
use tempfile::tempdir;

const NO_ESCAPE_NAME: &'static str = "filename12345678.txt";
const ESCAPE_NAME: &'static str = "file\tname\t12345678.txt";

fn new_escape<'a>(string: &'a str) -> Cow<'a, str> {
    if string
        .chars()
        .all(|c| c >= 0x20 as char && c != 0x7f as char)
    {
        string.into()
    } else {
        let mut chars = String::with_capacity(string.len());
        for c in string.chars() {
            // The `escape_default` method on `char` is *almost* what we want here, but
            // it still escapes non-ASCII UTF-8 characters, which are still printable.
            if c >= 0x20 as char && c != 0x7f as char {
                chars.push(c);
            } else {
                chars.extend(c.escape_default());
            }
        }
        chars.into()
    }
}

fn old_escape<'a>(string: &'a str) -> String {
    if string
        .chars()
        .all(|c| c >= 0x20 as char && c != 0x7f as char)
    {
        string.to_string()
    } else {
        let mut chars = String::new();
        for c in string.chars() {
            // The `escape_default` method on `char` is *almost* what we want here, but
            // it still escapes non-ASCII UTF-8 characters, which are still printable.
            if c >= 0x20 as char && c != 0x7f as char {
                chars.push(c);
            } else {
                chars += &c.escape_default().collect::<String>();
            }
        }
        chars
    }
}

fn bench_str_escape(c: &mut Criterion) {
    let mut group = c.benchmark_group("Escape string");
    for (p, i) in [
        ("No escape string", NO_ESCAPE_NAME),
        ("Escape string", ESCAPE_NAME),
    ] {
        group.bench_with_input(BenchmarkId::new("Old escape", p), i, |b, i| {
            b.iter(|| old_escape(black_box(i)))
        });
        group.bench_with_input(BenchmarkId::new("New escape", p), i, |b, i| {
            b.iter(|| new_escape(black_box(i)))
        });
    }
    group.finish();
}

fn bench_name_render(c: &mut Criterion) {
    let mut group = c.benchmark_group("Name Rendering");

    let tmp_dir = tempdir().expect("failed to create temp dir");
    let icons = Icons::new(icon::Theme::Fancy, " ".to_string());
    let colors = Colors::new(color::ThemeOption::NoLscolors);
    let display_option = DisplayOption::FileName;
    let hyperlink = HyperlinkOption::Never;

    // Create the files;
    let file_path_escape = tmp_dir.path().join(ESCAPE_NAME);
    File::create(&file_path_escape).expect("failed to create file");
    let meta_escape = file_path_escape.metadata().expect("failed to get metas");
    let name_escape = Name::new(
        &file_path_escape,
        FileType::new(&meta_escape, None, &Permissions::from(&meta_escape)),
    );

    let file_path_no_escape = tmp_dir.path().join(NO_ESCAPE_NAME);
    File::create(&file_path_no_escape).expect("failed to create file");
    let meta_no_escape = file_path_no_escape.metadata().expect("failed to get metas");
    let name_no_escape = Name::new(
        &file_path_no_escape,
        FileType::new(&meta_no_escape, None, &Permissions::from(&meta_no_escape)),
    );

    // Benchmarking
    group.bench_with_input(
        BenchmarkId::new("Escape", ESCAPE_NAME),
        &name_escape,
        |b, i| b.iter(|| i.render(&colors, &icons, &display_option, hyperlink)),
    );
    group.bench_with_input(
        BenchmarkId::new("No Escape", NO_ESCAPE_NAME),
        &name_no_escape,
        |b, i| b.iter(|| i.render(&colors, &icons, &display_option, hyperlink)),
    );

    group.finish();
}

criterion_group!(render_benches, bench_name_render);
criterion_group!(escape_benches, bench_str_escape);
criterion_main!(render_benches, escape_benches);
