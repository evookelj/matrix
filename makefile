comp: main.rs
	rustc main.rs

run: comp main
	./main
	display img.ppm
