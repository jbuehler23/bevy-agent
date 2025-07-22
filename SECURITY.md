# Security Policy

## Supported Versions

We actively support the following versions of Bevy AI with security updates:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | ✅                |

## Reporting a Vulnerability

We take security seriously. If you discover a security vulnerability, please follow these steps:

### Private Disclosure

1. **Do not** open a public issue for security vulnerabilities
2. Send an email to: security@bevy-ai.dev (or the project maintainer)
3. Include detailed information about the vulnerability
4. Allow time for us to investigate and fix the issue

### What to Include

Please include the following information in your report:

- Description of the vulnerability
- Steps to reproduce the issue
- Potential impact and severity
- Any suggested fixes or mitigations
- Your contact information for follow-up

### Response Timeline

- **Initial Response**: Within 24 hours
- **Assessment**: Within 72 hours
- **Fix Development**: Depending on severity (critical issues prioritized)
- **Public Disclosure**: After fix is released and users have time to update

## Security Considerations

### API Keys

- Never commit API keys to version control
- Use environment variables or secure configuration files
- Rotate keys regularly
- Use least-privilege access

### Dependencies

- We regularly audit dependencies for known vulnerabilities
- Security updates are released promptly
- Use `cargo audit` to check for vulnerabilities

### AI Provider Security

- All API communications use HTTPS
- API keys are never logged or stored in plaintext
- Request/response data is handled securely

## Best Practices

### For Users

1. **Secure API Keys**: Store API keys securely, never in code
2. **Update Regularly**: Keep Bevy AI updated to the latest version
3. **Review Generated Code**: Always review AI-generated code before use
4. **Network Security**: Use secure networks when communicating with AI providers

### For Developers

1. **Input Validation**: Validate all user inputs
2. **Error Handling**: Don't expose sensitive information in error messages
3. **Logging**: Never log sensitive information like API keys
4. **Dependencies**: Keep dependencies updated and audit regularly

## Vulnerability Disclosure Policy

We believe in responsible disclosure and will:

1. Acknowledge receipt of vulnerability reports
2. Work with reporters to understand and fix issues
3. Provide credit to reporters (unless they prefer anonymity)
4. Release security advisories for significant vulnerabilities
5. Coordinate with package managers for security releases

## Contact

For security-related inquiries:
- Email: security@bevy-ai.dev
- For non-security issues, use the GitHub issue tracker

## Security Tools

We use the following tools to maintain security:

- `cargo audit` for dependency vulnerability scanning
- `clippy` for code quality and security lints
- GitHub Security Advisories for dependency monitoring
- Automated security updates via Dependabot
