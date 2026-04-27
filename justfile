amend-push:
	git commit -a --amend --no-edit
	git push -f

install:
	cargo build --release
	sudo cp target/release/ssg /usr/local/bin
