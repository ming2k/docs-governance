# Setup

This guide helps contributors configure their local development environment for `docs-governance`.

## Prerequisites

- **POSIX Shell**: `bash` (4.0+) or `/bin/sh`.
- **Python**: Python 3.8+ (standard library only).
- **Git**: 2.20+ for version control.

## Development Workflow

1. Clone and navigate to repository:
   ```bash
   git clone https://github.com/ming/docs-governance.git
   cd docs-governance
   ```

2. Ensure toolchain permissions:
   ```bash
   chmod +x tools/*.sh
   ```

3. Run automated tests:
   ```bash
   python3 -m unittest discover -s tests -v
   ```

4. Verify local governance compliance:
   ```bash
   ./tools/verify.sh .
   ```

## See Also

- [Testing](testing.md)
- [Acceptance](acceptance.md)
- [Experience Scenarios](experience_scenarios.md)
