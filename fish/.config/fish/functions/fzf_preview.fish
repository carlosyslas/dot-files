function fzf_preview
    if test -d $argv[1]
        ls -a --color=always $argv
    else
        bat --color=always --plain $argv
    end
end
