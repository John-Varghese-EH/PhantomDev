# PhantomDev IDE Rules for Cursor

## Overview
These rules help configure Cursor IDE to work optimally with PhantomDev.

## Settings

### AI Assistant Settings
```json
{
  "cursor.ai.enabled": true,
  "cursor.ai.model": "claude-3-opus",
  "cursor.ai.temperature": 0.7,
  "cursor.ai.maxTokens": 2000
}
```

### PhantomDev Integration
```json
{
  "phantomdev.enabled": true,
  "phantomdev.autoScan": true,
  "phantomdev.autoHumanize": false,
  "phantomdev.threshold": 0.15
}
```

## Recommended Workflow

1. **Before Committing**: Run `phantomdev scan` to check for AI patterns
2. **After AI Generation**: Run `phantomdev humanize` to match your style
3. **Regular Checks**: Use `phantomdev score` to monitor stealth

## Keybindings

```json
{
  "keybindings": [
    {
      "key": "cmd+shift+p",
      "command": "phantomdev.scan"
    },
    {
      "key": "cmd+shift+h",
      "command": "phantomdev.humanize"
    },
    {
      "key": "cmd+shift+s",
      "command": "phantomdev.score"
    }
  ]
}
```

## Tips

- Use PhantomDev's style learning to maintain consistency
- Enable jitter for temporal obfuscation when needed
- Review stealth scores before pushing to public repositories
