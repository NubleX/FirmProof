# Contributing

## Dev Environment
- Node 20+, pnpm 9+, Rust stable, Foundry, Docker. `pnpm -w install` to bootstrap.

## Workflow
- Branch off `main` using `feat/*`, `fix/*`, `chore/*`.
- Conventional Commits. PRs require passing CI and at least one approval.

## Code Quality
- ESLint/Prettier for TS, Clippy for Rust, Solhint for Solidity. `pnpm -w test` must pass.

## Security
- Never commit secrets. Use `.env.example`. Enable 2FA on GitHub. Report vulns via SECURITY.md.