# Issue Tracker API — Diagrams

> Sequence diagrams and system overview for the Issue Tracker REST API.
> Source of truth: [`backend/openapi.yaml`](../backend/openapi.yaml)

## Files

| File | Description |
|------|-------------|
| `00_system_overview.mermaid` | High-level architecture: clients, middleware, endpoint groups, database tables |
| `01_auth.mermaid` | Register, login, token refresh — happy paths and error flows (400, 401, 409) |
| `02_users.mermaid` | Get/update current user profile — happy paths and error flows (400, 401) |
| `03_projects.mermaid` | Project CRUD + member management — happy paths and error flows (400, 401, 403, 404, 409) |
| `04_issues.mermaid` | Issue CRUD with filters — happy paths and error flows (400, 401, 403, 404) |
| `05_comments.mermaid` | Comment list/create/delete — happy paths and error flows (400, 401, 403, 404) |
| `06_health.mermaid` | Liveness and readiness probes (200, 503) |

## Rendering

Any Mermaid-compatible viewer will render these files. GitHub renders `.mermaid` files natively. For local preview:

```bash
# npm install -g @mermaid-js/mermaid-cli
mmdc -i diagrams/01_auth.mermaid -o diagrams/01_auth.svg
```
