DEPENDENCIES=zsh vim tmux ranger jq httpie

install-dependencies:
	/usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"
	brew install $(DEPENDENCIES)
