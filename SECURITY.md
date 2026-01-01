# Security Policy

## Supported Versions

We actively maintain and provide security updates for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.4.x   | :white_check_mark: |
| < 0.4   | :x:                |

## Reporting a Vulnerability

We take security vulnerabilities seriously. If you discover a security vulnerability, please follow these steps:

### 1. **Do NOT** open a public GitHub issue

Security vulnerabilities should be reported privately to prevent exploitation before a fix is available.

### 2. Report the vulnerability

Please report security vulnerabilities by emailing the maintainer:

**Email:** odosmatthews@gmail.com

Include the following information in your report:

- **Description**: A clear description of the vulnerability
- **Impact**: The potential impact and severity of the vulnerability
- **Steps to reproduce**: Detailed steps to reproduce the issue
- **Affected versions**: Which versions of arrayops are affected
- **Suggested fix**: (Optional) If you have suggestions for how to fix the issue

### 3. Response Timeline

We will acknowledge receipt of your vulnerability report within **48 hours** and provide a timeline for addressing the issue.

Our response timeline for security vulnerabilities:

- **Critical vulnerabilities**: Response within 24 hours, fix within 7 days
- **High severity**: Response within 48 hours, fix within 14 days
- **Medium severity**: Response within 72 hours, fix within 30 days
- **Low severity**: Response within 7 days, fix within 90 days

### 4. Disclosure Policy

We follow a **coordinated disclosure** process:

1. We will acknowledge receipt of your report
2. We will investigate and verify the vulnerability
3. We will develop a fix and test it thoroughly
4. We will prepare a security advisory
5. We will coordinate with you on the disclosure timeline
6. We will release the fix and security advisory simultaneously

We aim to release fixes within the timelines above, but may adjust based on complexity. We will keep you informed of progress.

**Public disclosure**: We request that you do not publicly disclose the vulnerability until we have released a fix or 90 days have passed (whichever comes first), unless there is evidence of active exploitation.

### 5. Security Updates

Security updates will be released as:

- **Patch releases** (e.g., 0.4.0 → 0.4.1) for vulnerabilities in the current supported version
- **Security advisories** published on GitHub under the "Security" tab
- **Release notes** mentioning security fixes (without detailed vulnerability information until after disclosure)

### 6. What to Report

Please report:

- Memory safety issues (buffer overflows, use-after-free, etc.)
- Denial of service vulnerabilities
- Input validation issues that could lead to crashes or incorrect behavior
- Authentication or authorization issues (if applicable)
- Information disclosure vulnerabilities
- Any issue that violates Rust's memory safety guarantees

### 7. What NOT to Report

The following are generally not considered security vulnerabilities:

- Issues that require local code execution or physical access
- Issues requiring already-compromised dependencies
- Issues in optional dependencies not directly used by arrayops
- Denial of service through resource exhaustion when using documented APIs (e.g., processing extremely large arrays)

### 8. Recognition

We appreciate responsible disclosure and will acknowledge security researchers who help improve the security of arrayops (with your permission).

## Security Best Practices

For users of arrayops:

- Keep arrayops and its dependencies up to date
- Review the [Security Documentation](../docs/security.md) for secure usage patterns
- Report suspicious behavior or potential vulnerabilities

## Security Updates

To receive notifications about security updates:

- **GitHub**: Watch the repository for releases
- **PyPI**: Subscribe to release notifications
- **Email**: Contact the maintainer to be added to the security announcement list

## Contact

For security-related questions or to report vulnerabilities:

**Email:** odosmatthews@gmail.com

For general questions, please use GitHub Discussions or Issues.

---

_Last updated: 2024_

