build:
	cargo build --release

install:
	# create the directory if it doesn't exist
	systemctl --user stop theme-watcher.service
	sudo cp ./target/release/theme-watcher /usr/bin/theme-watcher
	cp ./theme-watcher.service $(HOME)/.config/systemd/user/theme-watcher.service
	systemctl --user daemon-reload
	systemctl --user enable theme-watcher.service
	systemctl --user start theme-watcher.service
