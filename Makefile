all: build examples doc

build:
	rustc termbox.rs

examples: examples/hello examples/demo

examples/hello: build examples/hello.rs
	(cd examples && rustc -L .. hello.rs)

examples/demo: build examples/demo.rs
	(cd examples && rustc -L .. demo.rs)

doc:
	rm -f doc/*.html
	rustdoc --output doc --output-format html termbox.rs

clean:
	rm -rf nsf
	rm -f libtermbox*.so
	rm -f demo
	rm -f doc/*.html
	rm -f examples/demo examples/hello

.PHONY: clean doc nsf examples
