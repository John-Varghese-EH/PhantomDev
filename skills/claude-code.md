# PhantomDev IDE Skill for Claude Code

## Overview
This skill configures Claude Code to write code with human-like patterns while maintaining professional quality.

## Instructions for Claude Code

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

### Example

```rust
// Good: Human-like comment explaining reasoning
// We use a HashMap here because we need O(1) lookups
// for the user sessions. This is more efficient than
// iterating through a Vec for each request.
let sessions: HashMap<String, Session> = HashMap::new();

// Bad: Obvious comment
// Create a HashMap
let sessions: HashMap<String, Session> = HashMap::new();
```

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

1. Install this skill in your Claude Code configuration
2. Claude will automatically follow these guidelines
3. Use `phantomdev scan` to verify human-like patterns
4. Use `phantomdev humanize` to fix any detected issues
