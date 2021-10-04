USERNAME = $(shell echo $$LOGNAME)
REPO_HOME = $(shell git rev-parse --show-toplevel)
REPO_VERSION ?= $(shell  if [ -f $(REPO_HOME)/config/version ]; then cat $(REPO_HOME)/config/version ; else printf "NOT_FOUND" ; fi)
HOSTNAME = $(shell hostname)
CURRENT_GIT_REPO =$(shell basename $(shell echo $(REPO_HOME)))
COLORRED = $(shell printf "\033[0;31m")
COLORBOLDRED := $(shell printf "\033[0;41m")
COLORGREEN = $(shell printf "\033[0;32m")
COLORYELLOW = $(shell printf "\033[0;33m")
COLORBLUE = $(shell printf  "\033[0;34m")
COLORMAGENTA = $(shell printf  "\033[0;35m")
COLORCYAN = $(shell printf  "\033[0;36m")
COLOROFF = $(shell printf "\033[0m")
TPUT = $(shell tput cub 4)
HEADER = $(shell printf "$(COLORYELLOW) [make] $(COLOROFF)")
INFO = $(shell printf "$(COLORYELLOW) [INFO] $(COLOROFF)")
ERROR = $(shell printf "$(COLORRED) [ERROR] $(COLOROFF)")
OK = $(shell printf "$(COLORGREEN) [OK] $(COLOROFF)")
MKDIR = mkdir -p
SRC_DIR = $(REPO_HOME)/src
HELP += version vars-help
RULE = $@
RULE_NAME = $(RULE:-rule=)

.ONESHELL:

auto: help

help: $(HELP)

version: show-build-version

show-build-version:
	@printf " $(HEADER) Code version:$(COLORCYAN) $(REPO_VERSION)$(COLOROFF) - Current Git repo: $(COLORGREEN)$(CURRENT_GIT_REPO)$(COLORYELLOW) [INFO] $(COLOROFF) \n"


vars-help: 
	@printf " $(HEADER) make help        -$(COLORCYAN) shows this help $(COLOROFF) \n"
	@printf " $(HEADER) make version     -$(COLORCYAN) shows current version of code $(COLOROFF) \n"

GIT_ALWAYS_ADD = src config README.md makefile .gitignore LICENSE.md
GITBRANCH = $(shell basename $(shell git symbolic-ref HEAD))
COMMITINFO =$(shell printf "$(COLORYELLOW)Branch:$(COLOROFF)$(GITBRANCH) - $(COLORYELLOW)Version:$(COLOROFF)$(REPO_VERSION) - $(COLORYELLOW)From:$(COLOROFF)$(USERNAME)@$(HOSTNAME) - $(COLORYELLOW)Date:$(COLOROFF)$$(date +"%y-%m-%d %H:%M:%S")")
TAGINFO =$(shell printf "$(COLORYELLOW)Tag:$(COLOROFF)$(REPO_VERSION) - $(COLORYELLOW)Version:$(COLOROFF)$(REPO_VERSION) - $(COLORYELLOW)Server:$(COLOROFF)$(HOSTNAME)")
HELP += git-help
help: $(HELP)
tag: show-build-version git-add git-create-commit  git-create-tag git-start-push-tag git-end-push-tag
commit: show-build-version git-add git-create-commit
git: commit



HELP += main-help
help: $(HELP)

push: commit git-push-origin-start git-main-push-end
pull: git-pull-origin-start git-main-pull-end

git-push-origin-start:
	@printf " $(HEADER) git push origin $(GITBRANCH) - $(REPO_VERSION) " 
	@git push origin $(GITBRANCH) 2> /dev/null || (echo $(cred) "\n\n ----- Error pushing to origin -----"  $(coff) "\n"; exit 1) 

git-pull-origin-start:
	@printf " $(HEADER) git pull origin $(GITBRANCH) - $(REPO_VERSION) " 
	git pull origin $(GITBRANCH) 2>/dev/null >/dev/null || (echo $(cred) "\n\n ----- Error pulling from origin -----"  $(coff) "\n"; exit 1) 

git-main-push-end:
	@printf "$(COLORGREEN)[DONE] $(COLOROFF)\n"

git-main-pull-end:
	@printf "$(COLORGREEN)[DONE] $(COLOROFF)\n"

main-help:
	@printf " $(HEADER) make push        -$(COLORCYAN) creates automatic commit and pushes all changes to remote$(COLOROFF) \n"
	@printf " $(HEADER) make pull        -$(COLORCYAN) Pulls all changes from remote branch $(COLORGREEN)$(GITBRANCH)$(COLOROFF) \n"

git-add:
	@printf " $(HEADER) git add $(COLORCYAN)$(GIT_ALWAYS_ADD) $(COLOROFF) "
	@git add $(GIT_ALWAYS_ADD)
	@printf "$(COLORGREEN)[DONE] $(COLOROFF)\n"

git-create-tag:
	@git tag -a "v$(REPO_VERSION)" -m "Auto make tag - Branch:$(GITBRANCH) - Code version:$(REPO_VERSION) - Server:$(HOSTNAME)" || (echo $(cred) "\n ----- Error creating tag -----"  $(coff)"\n"; exit 1) 

git-start-push-tag:
	@printf " $(HEADER) git push tag - $(TAGINFO) " 
	@git push --tags 2> /dev/null || (echo $(cred) "\n\n ----- Error pushing tag -----"  $(coff) "\n"; exit 1) 

git-end-push-tag:
	@printf "$(COLORGREEN)[DONE] $(COLOROFF)\n"

git-create-commit:
	@printf " $(HEADER) git commit - $(COMMITINFO) " 
	@git commit -m "Auto make commit - Branch:$(GITBRANCH) - Version:$(REPO_VERSION) - From:$(USERNAME)@$(HOSTNAME) - Date:$$(date +"%Y-%m-%d %H:%M:%S")" >/dev/null
	@printf "$(COLORGREEN)[DONE] $(COLOROFF)\n"

push-vim:
	@git add $(GIT_ALWAYS_ADD) 2>&1 >/dev/null
	@git commit -m "Auto vim make commit - Branch:$(GITBRANCH) - Version:$(REPO_VERSION) - From:$(USERNAME)@$(HOSTNAME) - Date:$$(date +"%Y-%m-%d %H:%M:%S")" 2>&1 >/dev/null
	@git_result=$$(git push origin $(GITBRANCH) 2>/dev/null; echo $$? ); \
	if [ $$git_result -eq 0 ] ; then \
		printf "OK";\
	else \
	printf "ERROR"; \
	fi

git-help:
	@printf " $(HEADER) make commit      -$(COLORCYAN) creates a git commit - does not push to origin  $(COLOROFF) \n"
	@printf " $(HEADER) make tag         -$(COLORCYAN) creates a remote git tag with current version$(COLOROFF) \n"
