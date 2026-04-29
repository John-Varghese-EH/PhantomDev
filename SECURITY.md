# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability, please report it responsibly.

### How to Report

1. **Do not** create a public issue
2. Send an email to: security@phantomdev.io
3. Include as much detail as possible:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if known)

### What to Expect

- We will acknowledge receipt within 48 hours
- We will provide a timeline for the fix
- We will coordinate disclosure with you
- We will credit you in the release notes (if desired)

### Security Best Practices

When using PhantomDev:

1. **Keep API keys secure**: Never commit API keys to version control
2. **Use environment variables**: Store sensitive configuration in environment variables
3. **Review generated code**: Always review AI-generated code before committing
4. **Update regularly**: Keep PhantomDev updated to the latest version
5. **Audit dependencies**: Regularly audit your dependencies for vulnerabilities

### Known Security Considerations

- PhantomDev uses local ML models when available to minimize data exposure
- Cloud API fallbacks send code to external services (when enabled)
- Style learning analyzes your repository's code history
- Git hooks execute automatically on commit operations

### Dependency Vulnerabilities

We regularly audit our dependencies. If you find a vulnerability:

```bash
# Check for vulnerabilities
cargo audit

# Update dependencies
cargo update
```

## Security Features

PhantomDev includes several security-focused features:

- **Local-first detection**: Prioritizes local models over cloud APIs
- **Configurable cloud fallback**: Can disable cloud API usage entirely
- **No telemetry**: PhantomDev does not collect usage data
- **Open source**: All code is available for audit

## Disclosure Policy

We follow responsible disclosure practices:

1. Vulnerabilities are fixed before public disclosure
2. Users are notified of security updates
3. Security advisories are published with release notes
4. Credit is given to reporters (when desired)

## Contact

For security-related questions:
- Email: security@phantomdev.io
- GitHub Security: https://github.com/John-Varghese-EH/PhantomDev/security/advisories
