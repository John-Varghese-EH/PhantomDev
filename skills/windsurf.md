# PhantomDev IDE Skill for Windsurf

## Overview
This skill configures Windsurf IDE to write code with human-like patterns while maintaining professional quality.

## Instructions for Windsurf

When generating code, follow these guidelines:

### Code Style
- Use snake_case for variables and functions (Rust, Python, Go)
- Use camelCase for variables and functions (JavaScript, TypeScript)
- Use PascalCase for types and structs
- Keep functions focused and small (under 50 lines when possible)
- Add doc comments for public APIs

### Comments
- Write comments that explain WHY, not WHAT
- Use natural language in comments
- Avoid excessive comments for obvious code
- Add TODO/FIXME comments sparingly
- Use inline comments for complex logic

### Variable Naming
- Use descriptive names that explain purpose
- Avoid single-letter variables except in loops
- Use meaningful abbreviations (e.g., `config` instead of `cfg`)
- Be consistent with naming conventions

### Patterns to Avoid
- Excessive emoji usage (max 1-2 per file)
- Uniform comment style (vary comment placement)
- Predictable variable naming patterns
- Lack of entropy in code structure
- Watermark patterns (TODO, FIXME overuse)

### Professional Quality
- Write clean, maintainable code
- Follow language-specific best practices
- Include error handling
- Add tests when appropriate
- Consider performance implications

## Configuration

```json
{
  "phantomdev.enabled": true,
  "phantomdev.human_like": true,
  "phantomdev.no_emojis": true,
  "phantomdev.descriptive_names": true
}
```

## Usage

1. Install this skill in your Windsurf configuration
2. Windsurf will automatically follow these guidelines
3. Use `phantomdev scan` to verify human-like patterns
4. Use `phantomdev humanize` to fix any detected issues

---

Built with ❤️ by John Varghese (J0X)
GitHub: https://github.com/John-Varghese-EH
LinkedIn: https://linkedin.com/in/John--Varghese
