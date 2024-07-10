extern crate swc_malloc;

use std::{fs::read_to_string, path::Path};

use codspeed_criterion_compat::{black_box, criterion_group, criterion_main, Criterion};
use swc_common::{errors::HANDLER, FileName};
use swc_css_ast::Stylesheet;
use swc_css_codegen::{writer::basic::BasicCssWriter, Emit};
use swc_css_minifier::minify;
use swc_css_parser::parse_file;

pub fn bench_files(c: &mut Criterion) {
    let mut group = c.benchmark_group("css/minify/libraries");
    group.sample_size(10);

    let mut bench_file = |name: &str, path: &Path| {
        let src = read_to_string(path).unwrap();

        group.bench_function(name, |b| {
            b.iter(|| {
                // We benchmark full time, including time for creating cm, handler
                run(&src)
            })
        });
    };

    bench_file(
        "bootstrap",
        Path::new("../../node_modules/bootstrap/dist/css/bootstrap.css"),
    );
}

criterion_group!(files, bench_files);
criterion_main!(files);

fn run(src: &str) {
    testing::run_test2(false, |cm, handler| {
        HANDLER.set(&handler, || {
            let fm = cm.new_source_file(FileName::Anon, src.into());

            let mut errors = vec![];
            let mut ss: Stylesheet =
                parse_file(&fm, None, Default::default(), &mut errors).unwrap();

            minify(&mut ss, Default::default());

            let mut buf = String::new();
            {
                let wr = BasicCssWriter::new(&mut buf, None, Default::default());
                let mut generator = swc_css_codegen::CodeGenerator::new(
                    wr,
                    swc_css_codegen::CodegenConfig { minify: true },
                );

                generator.emit(&ss).unwrap();
            }

            black_box(buf);
            Ok(())
        })
    })
    .unwrap();
}
