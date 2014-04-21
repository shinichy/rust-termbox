all: build examples doc

build:
	mkdir -p lib
	rustc --out-dir lib src/termbox/lib.rs

examples: examples/hello examples/demo

examples/hello: build examples/hello.rs
	(cd examples && rustc -L ../lib hello.rs)

examples/demo: build examples/demo.rs
	(cd examples && rustc -L ../lib demo.rs)

doc:
	rustdoc --output doc --output-format html src/termbox/lib.rs

clean:
	rm -rf nsf
	rm -f lib/libtermbox*
	rm -f demo
	rm -rf doc/
	rm -f examples/demo examples/hello 

.PHONY: clean doc nsf examples
