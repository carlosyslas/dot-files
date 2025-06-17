(require 'use-package)

(defhydra hydra-frames (global-map "M-f")
  "Frames"
  ("o" other-frame "Other frame")
  ("n" make-frame-command "New frame")
  ("d" delete-frame "Delete frame"))

(use-package doom-themes
  :config
  (load-theme 'doom-vibrant t))

(use-package doom-modeline
  :defer t
  :hook (after-init . doom-modeline-init))

(set-face-attribute 'default nil :height 125)
(set-frame-font "Monaco")

(provide 'ui-sense)
