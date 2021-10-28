(require 'uniquify)
(require 'package)

(setq is-mac (equal system-type 'darwin))

(setq custom-file (expand-file-name "custom.el" user-emacs-directory))

(fset 'yes-or-no-p 'y-or-n-p)

(pending-delete-mode t)

(add-hook 'before-save-hook 'whitespace-cleanup)

(setq warning-minimum-level :error)

(setq initial-scratch-message nil)

(setq inhibit-startup-message t)

(menu-bar-mode -1)

(if (boundp 'tool-bar-mode)
    (tool-bar-mode -1))

(if (boundp 'scroll-bar-mode)
    (scroll-bar-mode -1))

(setq backup-directory-alist
      `((".*" . ,temporary-file-directory)))
(setq auto-save-file-name-transforms
      `((".*" ,temporary-file-directory t)))

(setq uniquify-buffer-name-style 'forward)
(setq uniquify-separator "/")
(setq uniquify-after-kill-buffer-p t)    ; rename after killing uniquified
(setq uniquify-ignore-buffers-re "^\\*")

(global-auto-revert-mode 1)

;; Disable line wrapping
(set-default 'truncate-lines t)

(when is-mac
  ;; Maximize frame
  (add-hook 'emacs-startup-hook 'toggle-frame-maximized)
  ;; Set option to nil to enable extrange chars
  (setq mac-option-modifier nil)
  ;; Use control from Meta
  (setq ns-command-modifier (quote meta))
  ;; Set right option to control
  (setq mac-right-option-modifier 'control)
  (add-to-list 'default-frame-alist '(ns-transparent-titlebar . t))
  (add-to-list 'default-frame-alist '(ns-appearance . dark)))

(add-to-list 'package-archives (cons "melpa" "https://melpa.org/packages/") t)

(package-initialize)

(unless (package-installed-p 'use-package)
  (package-refresh-contents)
  (package-install 'use-package))

(setq use-package-always-ensure t)

(use-package quelpa
  :config
  (quelpa-self-upgrade))

(use-package quelpa-use-package
  :after
  (quelpa))

(use-package company
  :config
  (setq company-idle-delay 0.2)
  (global-company-mode))

(use-package ivy
  :custom
  (ivy-use-virtual-buffers t)
  :config
  (ivy-mode))

(use-package hydra)

(use-package eyebrowse
  :config
  (eyebrowse-mode)
  (dolist (number (number-sequence 1 9))
    (bind-key (format "M-%d" number) (intern (format "eyebrowse-switch-to-window-config-%d" number)))))

(use-package exec-path-from-shell
  :config
  (exec-path-from-shell-initialize))

(bind-key "M-d" 'dired-jump)

(intern "eyebrowse")


(provide 'common-sense)
