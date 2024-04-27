local is_transparent = false
if vim.fn.has("unix") == 1 then
  is_transparent = true
end

return {
  "neanias/everforest-nvim",
  lazy = false,
  priority = 998,
  config = function() 
    require("everforest").setup({
      transparent = is_transparent,
      italic_comments = true,
      terminal_colors = true,
      borderless_telescope = false,
      background = "hard",
      italics = true,
    })
    vim.cmd("colorscheme everforest")
  end,
}
