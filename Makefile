# git commit and push

ifneq ($(filter git,$(MAKECMDGOALS)),)
  GIT_MSG_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
  $(foreach _g,$(GIT_MSG_ARGS),$(eval $(_g):;@:))
endif

.PHONY: git

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