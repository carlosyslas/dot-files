(require 'common-sense)

(defgroup sense nil
  "Emacs Sense Configuration"
  :group 'emacs)

(defgroup evil-sense nil
  "Evil Mode Configuration"
  :group 'sense)

(defcustom evil-sense/enabled nil
  "Enables or disables evil-mode"
  :type 'boolean
  :group 'evil-sense)

(provide 'custom-sense)
