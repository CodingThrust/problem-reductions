# Shell completions

Enable tab completion by adding one line to your shell config:

```bash
# bash (~/.bashrc)
eval "$(pred completions bash)"

# zsh (~/.zshrc)
eval "$(pred completions zsh)"

# fish (~/.config/fish/config.fish)
pred completions fish | source
```

If the shell argument is omitted, `pred completions` auto-detects your current shell.
