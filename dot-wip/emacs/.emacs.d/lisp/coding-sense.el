(require 'use-package)

(use-package magit
  :config
  (bind-key "M-g" 'magit-status))

(use-package projectile
  :config
  (projectile-mode))

(use-package counsel-projectile
  :after (projectile hydra)
  :config
  (defhydra hydra-projectile (global-map "M-p")
    "Projectile"
    ("p" counsel-projectile-switch-project "Switch project")
    ("f" counsel-projectile-find-file "Find file")
    ;;("s" counsel-projectile-find-file "Search")
    ("b" counsel-projectile-switch-to-buffer "Switch to buffer")))

(use-package editorconfig
  :config
  (editorconfig-mode 1))

(use-package editorconfig-generate)

(use-package docker
  :bind (("M-c" . docker)))


;; Process runner
(use-package prodigy)

;; Javascript
(use-package web-mode
  :mode ("\\.tsx\\'" . web-mode))

(defun sense/setup-tide ()
  (tide-setup)
  (flycheck-mode 1)
  (eldoc-mode 1)
  (tide-hl-identifier-mode))

(use-package tide
  :hook ((js-mode . sense/setup-tide)
	 (web-mode . sense/setup-tide)))

(use-package prettier-js
  :hook ((js-mode . prettier-js-mode)
	 (web-mode . prettier-js-mode)))

;; Golang
(use-package go-mode)

;; Python
(use-package elpy
  :init
  (elpy-enable))

;; Elixir
(use-package elixir-mode)
(use-package alchemist)

(provide 'coding-sense)
