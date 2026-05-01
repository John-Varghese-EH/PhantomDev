# PhantomDev Universal Skill

## Overview
This skill configures AI agents to write code with human-like patterns while maintaining professional quality. It works with Claude Code, Cursor, Windsurf, Antigravity, and other AI coding assistants.

## Instructions for AI Agents

When generating code, follow these guidelines to maintain a human-like coding style:

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

## Usage Instructions

1. Install this skill in your AI agent configuration
2. The AI agent will automatically follow these guidelines
3. Use `phantomdev scan` to verify human-like patterns
4. Use `phantomdev humanize` to fix any detected issues

## Code Generation Guidelines

### Commit Messages
- Strip "feat:", "fix:", "chore:" prefixes unless specifically requested
- Focus on the 'what' in 5-7 words max
- Example: "Refactor user login logic for better speed" → "fixed login lag"

### Code Comments
- Shift from "What it does" to "Why it exists"
- Use inline comments (//) instead of docblocks (/** */) for internal logic
- Occasionally include "todo" or "fixme" if the code looks like a workaround
- Example: "// Increments the counter by one" → "// temp fix for race condition"

### Variable Naming
- Reduce "over-descriptiveness"
- Example: "userAuthenticationToken" → "authToken" or "user_tk"
- Match the existing codebase's naming convention (camelCase vs snake_case)

### Adversarial Logic

- **INCREASE_ENTROPY**: Vary sentence lengths significantly
- **STOCHASTIC_DEGRADATION**: Occasionally (5% chance) use a common developer typo or shorthand (e.g., "config" → "conf", "synchronous" → "sync")
- **PERSONA_SHIFT**: Default to "Tired Senior Developer"

### Vocabulary Filter
Strictly ban these words in generated content:
- "comprehensive"
- "ensure"
- "robust"
- "leveraging"
- "meticulous"
- "seamless"
- "enhanced"

### Output Format
Return ONLY the transformed text. No preamble, no "Sure, here is the humanized version," and no markdown blocks unless the input was a full code file.

## Examples

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

## Integration Points

This skill can be integrated with:
- Claude Code (/claude-code)
- Cursor (/cursor)
- Windsurf (/windsurf)
- Antigravity (/antigravity)
- Other AI coding assistants

## Environment Variables

Set `CLAUDE_CODE_UNDERCOVER=1` in your `.zshrc` or `.bashrc` to enable this mode.

## Additional Resources

- Documentation: https://john-varghese-eh.github.io/PhantomDev/
- GitHub: https://github.com/John-Varghese-EH/PhantomDev
- crates.io: https://crates.io/crates/phantomdev