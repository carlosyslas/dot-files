# get current branch in git repo
function parse_git_branch() {
	BRANCH=`git branch 2> /dev/null | sed -e '/^[^*]/d' -e 's/* \(.*\)/\1/'`
	if [ ! "${BRANCH}" == "" ]
	then
		STAT=`parse_git_dirty`
		echo "[${BRANCH}${STAT}]"
	else
		echo ""
	fi
}

# get current status of git repo
function parse_git_dirty {
	status=`git status 2>&1 | tee`
	dirty=`echo -n "${status}" 2> /dev/null | grep "modified:" &> /dev/null; echo "$?"`
	untracked=`echo -n "${status}" 2> /dev/null | grep "Untracked files" &> /dev/null; echo "$?"`
	ahead=`echo -n "${status}" 2> /dev/null | grep "Your branch is ahead of" &> /dev/null; echo "$?"`
	newfile=`echo -n "${status}" 2> /dev/null | grep "new file:" &> /dev/null; echo "$?"`
	renamed=`echo -n "${status}" 2> /dev/null | grep "renamed:" &> /dev/null; echo "$?"`
	deleted=`echo -n "${status}" 2> /dev/null | grep "deleted:" &> /dev/null; echo "$?"`
	bits=''
	if [ "${renamed}" == "0" ]; then
		bits="💩"
	fi
	if [ "${ahead}" == "0" ]; then
		bits="💩"
	fi
	if [ "${newfile}" == "0" ]; then
		bits="💩"
	fi
	if [ "${untracked}" == "0" ]; then
		bits="💩"
	fi
	if [ "${deleted}" == "0" ]; then
		bits="💩"
	fi
	if [ "${dirty}" == "0" ]; then
		bits="💩"
	fi
	if [ ! "${bits}" == "" ]; then
		echo " ${bits}"
	else
		echo ""
	fi
}

export PS1="\w\[\e[32m\]\` parse_git_branch \`\[\e[m\] • "

# Auto CD
#shopt -s autocd
export CDPATH=$HOME

# Aliases
alias ls="ls -G"
alias la="ls -lah"
alias ll="ls -lh"
alias rd="rmdir"
alias rmr="rm -rf"
alias plz="sudo"

alias gst="git status"
alias gl="git pull"
alias gp="git push"
alias gpsup='git push --set-upstream origin $(git_current_branch)'
alias glg="git log --stat"
alias glgg="git log --graph"
alias gco="git checkout"
alias gm="git merge"
alias grep='grep  --color=auto --exclude-dir={.bzr,CVS,.git,.hg,.svn}'
alias ..="cd .."
alias ...="cd ../.."
alias ....="cd ../../.."
alias .....="cd ../../../.."
alias ......="cd ../../../../.."
