# Security Policy

## Supported Versions

| Version | Supported |
|----------|-----------|
| 1.0.x    | ✅         |
| < 1.0     | ❌         |

## Reporting a Vulnerability

If you discover a security vulnerability in N501-TTS Lite, please report it responsibly.

### How to Report

**Do NOT open a public issue.** Instead, send a private report to:

📧 Email: security@example.com

Or send a private message to:
- GitHub: [@Nombah501](https://github.com/Nombah501)

### What to Include

Please include:

- Description of the vulnerability
- Steps to reproduce the vulnerability
- Affected version(s)
- Potential impact
- Any proposed fix (if available)

### What to Expect

- We will acknowledge receipt of your report within 48 hours
- We will provide an estimated timeline for a fix
- We will notify you when a fix is released
- You will be credited in the release notes (if you wish)

## Security Best Practices

### For Users

- Only download releases from [official GitHub releases](https://github.com/Nombah501/n501-tts-lite/releases)
- Verify checksums of downloaded files (we provide SHA256 in releases)
- Keep your software updated to latest version
- Only grant necessary permissions to the application

### For Developers

- Follow [Contributing Guidelines](CONTRIBUTING.md) for secure coding practices
- Never commit secrets or API keys (use environment variables)
- Use pre-commit hooks to catch accidental secret commits
- Keep dependencies updated (we use Dependabot for this)

### Vulnerability Types We Care About

- Remote code execution
- Local privilege escalation
- Information disclosure (transcriptions exposed)
- Denial of service
- Audio capture vulnerabilities
- Clipboard manipulation
- Model poisoning or adversarial attacks

## Security Features

- **Local-only mode**: No data sent to external servers
- **Config encryption**: API keys stored securely
- **Input validation**: All user inputs validated
- **Error handling**: No sensitive data in error messages
- **Permissions**: Minimal required permissions

## Dependency Security

We regularly audit our dependencies. Security updates are automatically monitored via:

- [GitHub Dependabot](https://github.com/Nombah501/n501-tts-lite/security/dependabot)
- [Rust Advisory Database](https://rustsec.org/advisories)

We aim to address critical security issues within 72 hours of discovery.

---

Thank you for helping keep N501-TTS Lite secure! 🔒
