# git commit and push

ifneq ($(filter git,$(MAKECMDGOALS)),)
  GIT_MSG_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
  $(foreach _g,$(GIT_MSG_ARGS),$(eval $(_g):;@:))
endif

.PHONY: git install-web build-wasm build-wasm-release dev-web build-web preview-web clean-web

# git commit and push
git:
	@set -e; \
	msg=''; \
	if [ -n "$(strip $(MSG))" ]; then \
		msg='$(subst ','\'',$(MSG))'; \
	elif [ -n "$(strip $(GIT_MSG_ARGS))" ]; then \
		msg='$(subst ','\'',$(GIT_MSG_ARGS))'; \
	else \
		printf 'input commit message: '; read -r msg; \
	fi; \
	git add . && \
	git commit -a -m "$$msg" && \
	git pull && \
	git push && \
	echo git commit and push success

## Web (WASM + Vite): requires Rust nightly + wasm32 + wasm-bindgen-cli; frontend uses bun.

install-web:
	@rustup toolchain install nightly --component rustfmt 2>/dev/null || true
	@rustup target add wasm32-unknown-unknown --toolchain nightly 2>/dev/null || true
	@command -v wasm-bindgen >/dev/null 2>&1 || cargo install wasm-bindgen-cli --version 0.2.121 -f
	@cd www && bun install

build-wasm:
	@./scripts/build-wasm.sh

build-wasm-release:
	@./scripts/build-wasm.sh --release

dev-web: build-wasm
	@cd www && bun install && bun run dev

build-web: build-wasm-release
	@cd www && bun install && bun run build

preview-web:
	@cd www && bun run preview

clean-web:
	@rm -rf www/dist www/src/wasm/*.js www/src/wasm/*.wasm
