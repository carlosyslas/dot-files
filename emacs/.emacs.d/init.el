;; Keep emacs Custom-settings in separate file
(setq custom-file (expand-file-name "custom.el" user-emacs-directory))

(when (file-exists-p custom-file)
  (load-file custom-file))

(add-to-list 'load-path "~/.emacs.d/lisp")

(require 'common-sense)
(require 'custom-sense)
(require 'evil-sense)
(require 'coding-sense)
(require 'ui-sense)

(eval-when-compile
  (require 'use-package))
