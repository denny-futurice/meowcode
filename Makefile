PREFIX ?= $(HOME)/.local
DATA_HOME ?= $(or $(XDG_DATA_HOME),$(HOME)/.local/share)
ASSETS_DIR ?= $(DATA_HOME)/meowcode/assets

.PHONY: install install-bin install-assets uninstall

install: install-bin install-assets

install-bin:
	cargo install --path . --root $(PREFIX)

install-assets:
	mkdir -p $(ASSETS_DIR)
	cp assets/*.png $(ASSETS_DIR)/

uninstall:
	cargo uninstall --root $(PREFIX) meowcode
	rm -rf $(ASSETS_DIR)
