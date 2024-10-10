return {
    {
        "folke/noice.nvim",
        dependencies = {
            "rcarriga/nvim-notify",
            "MunifTanjim/nui.nvim",
        },
        event = "VeryLazy",
        config = function()
            require("noice").setup({
                lsp = {
                    override = {
                        ["vim.lsp.util.convert_input_to_markdown_lines"] = true,
                        ["vim.lsp.util.stylize_markdown"] = true,
                        ["cmp.entry.get_documentation"] = true,
                    },
                },
                routes = {
                    {
                        filter = {
                            event = "msg_show",
                            any = {
                                { find = "%d+L, %d+B" },
                                { find = "; after #%d+" },
                                { find = "; before #%d+" },
                            },
                        },
                        view = "mini",
                    },
                },
                messages = {
                    view = "mini",
                    view_error = "mini",
                    view_warn = "mini",
                },
                notify = {
                    view = "mini",
                },
                cmdline = {
                    view = "mini",
                    enabled = true,
                },

                views = {
                    mini = {
                        win_options = {
                            winblend = 0,
                        },
                    },
                },
            })
        end,
    },
    {
        "rcarriga/nvim-notify",
        event = "VeryLazy",
        config = function()
            require("notify").setup({
                background_colour = "#000000",
                fps = "30",
                render = "default",
                stage = "fade_in_slide_out",
                minimum_width = 10,
                max_width = 40,
                max_height = 20,
                top_down = false,
                timeout = 500,
            })
        end,
    },
}
