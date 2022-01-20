hyperfine -w 5 -r 100 -n purginator -n purgecss --export-markdown benchmarks.md \
    'target/release/purginator --html benchmarks/index.html --css benchmarks/styles.css > benchmarks/output/purginator.css' \
    'purgecss --content benchmarks/index.html --css benchmarks/styles.css --output benchmarks/output/purgecss.css'