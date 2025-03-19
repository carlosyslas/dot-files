local home_dir = vim.fn.expand("$HOME")
local node_bin = "/.local/share/fnm/node-versions/v22.14.0/installation/bin/"
vim.g.node_host_prog = home_dir .. node_bin .. "node"
vim.cmd("let $PATH = '" .. home_dir .. node_bin .. ":' . $PATH")

return {
  {
    "neovim/nvim-lspconfig",
  },
  {
    "williamboman/mason.nvim",
    opts = {},
  },
  {
    "williamboman/mason-lspconfig.nvim",
    opts = {
      ensure_installed = {
        "lua-language-server",
        "stylua",
        --"shfmt",
        "rust_analyzer",
        "biome",
        "eslint",
        "pyright",
        "html",
        "cssls",
        "tailwindcss",
        "ruff",
        "sqlls",
        "jsonls",
        "yamlls",
        "rnix",
        "gopls",
        "jinja_lsp",
        "dockerls",
        "docker_compose_language_service",
      },
    },
  },
}
