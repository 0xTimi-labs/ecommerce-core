# Repository guidance

This repository starts as Bootstrap governance only. Do not add ecommerce domain models, application code, contracts, Cargo workspaces, or locked BDD assets until an approved Artifact/Spec defines them.

- Use the Issue Forms to preserve the Artifact → Feature → Slice → Final Integration boundary.
- Keep Skills generic and repository-local; GitHub-only review prompts belong in `.github/review-prompts/`.
- Run `./scripts/bootstrap-check.sh` after changing `.github/`, templates, or validators.
- Record actual commands and remaining risk in the PR template. Human review remains required.
- Do not stage, commit, push, or change GitHub settings unless explicitly asked.
