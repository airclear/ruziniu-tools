# AI Agent Instructions

## Project: ruziniu-tools

Lightweight spec methodology for AI-powered development.

## Core Rules

1. **Read README.md first** - Understand project context
2. **Check specs/** - Review existing specs before starting
3. **Follow LeanSpec principles** - Clarity over documentation
4. **Keep it minimal** - If it doesn't add clarity, cut it
5. **Never use nested code blocks** - Markdown doesn't support code blocks within code blocks. Use indentation or describe the structure instead

## When to Use Specs

- Features that affect multiple parts of the system
- Breaking changes or significant refactors
- Design decisions that need team alignment
- Complex features that benefit from upfront thinking

Skip specs for:
- Bug fixes
- Trivial changes
- Self-explanatory refactors

## Discovery Commands

Before starting work, understand project context:

- `lean-spec stats` - See work distribution across specs
- `lean-spec board` - View specs organized by status
- `lean-spec list --tag=<tag>` - Find specs by tag (e.g., `--tag=api`)
- `lean-spec search "<query>"` - Full-text search across specs
- `lean-spec deps <spec>` - Check dependencies before starting work

These commands help you understand what exists, what's in progress, and what depends on what.

## Spec Frontmatter

When creating or updating specs, include YAML frontmatter at the top:

```yaml
---
status: draft|planned|in-progress|complete|blocked|cancelled
created: YYYY-MM-DD
tags: [tag1, tag2]  # helps with discovery
priority: low|medium|high  # helps with planning
assignee: username  # for team coordination
---
```

**Core fields:**
- `status` and `created` are required
- `tags` help with discovery and organization
- `priority` helps teams plan work
- `assignee` shows who's working on what

**Update status with:**
```bash
lean-spec update <spec> --status in-progress --assignee yourname
# or edit frontmatter directly
```

## Workflow

1. **Discover context** - Run `lean-spec stats` or `lean-spec board` to see current state
2. **Search existing specs** - Use `lean-spec search` or `lean-spec list` to find relevant work
3. **Check dependencies** - Run `lean-spec deps <spec>` if working on existing spec
4. **Create or update spec** - Add frontmatter with required fields and helpful metadata
5. **Implement changes** - Keep spec in sync as you learn
6. **Update status** - Mark progress: `draft` → `in-progress` → `complete`
7. **Archive when done** - `lean-spec archive <spec>` moves to archive

## Quality Standards

- Code is clear and maintainable
- Tests cover critical paths
- No unnecessary complexity
- Documentation where needed (not everywhere)
- Specs stay in sync with implementation

---

**Remember**: LeanSpec is a mindset. Adapt these guidelines to what actually helps.


<!-- OPENSPEC:START -->
# OpenSpec Instructions

These instructions are for AI assistants working in this project.

Always open `@/openspec/AGENTS.md` when the request:
- Mentions planning or proposals (words like proposal, spec, change, plan)
- Introduces new capabilities, breaking changes, architecture shifts, or big performance/security work
- Sounds ambiguous and you need the authoritative spec before coding

Use `@/openspec/AGENTS.md` to learn:
- How to create and apply change proposals
- Spec format and conventions
- Project structure and guidelines

Keep this managed block so 'openspec update' can refresh the instructions.

<!-- OPENSPEC:END -->

# AI Agent Instructions

## Project: ruziniu-tools

Lightweight spec methodology for AI-powered development.

## Core Rules

1. **Read README.md first** - Understand project context
2. **Check specs/** - Review existing specs before starting
3. **Follow LeanSpec principles** - Clarity over documentation
4. **Keep it minimal** - If it doesn't add clarity, cut it
5. **Never use nested code blocks** - Markdown doesn't support code blocks within code blocks. Use indentation or describe the structure instead

## When to Use Specs

- Features that affect multiple parts of the system
- Breaking changes or significant refactors
- Design decisions that need team alignment
- Complex features that benefit from upfront thinking

Skip specs for:
- Bug fixes
- Trivial changes
- Self-explanatory refactors

## Discovery Commands

Before starting work, understand project context:

- `lean-spec stats` - See work distribution across specs
- `lean-spec board` - View specs organized by status
- `lean-spec list --tag=<tag>` - Find specs by tag (e.g., `--tag=api`)
- `lean-spec search "<query>"` - Full-text search across specs
- `lean-spec deps <spec>` - Check dependencies before starting work

These commands help you understand what exists, what's in progress, and what depends on what.

## Spec Frontmatter

When creating or updating specs, include YAML frontmatter at the top:

```yaml
---
status: draft|planned|in-progress|complete|blocked|cancelled
created: YYYY-MM-DD
tags: [tag1, tag2]  # helps with discovery
priority: low|medium|high  # helps with planning
assignee: username  # for team coordination
---
```

**Core fields:**
- `status` and `created` are required
- `tags` help with discovery and organization
- `priority` helps teams plan work
- `assignee` shows who's working on what

**Update status with:**
```bash
lean-spec update <spec> --status in-progress --assignee yourname
# or edit frontmatter directly
```

## Workflow

1. **Discover context** - Run `lean-spec stats` or `lean-spec board` to see current state
2. **Search existing specs** - Use `lean-spec search` or `lean-spec list` to find relevant work
3. **Check dependencies** - Run `lean-spec deps <spec>` if working on existing spec
4. **Create or update spec** - Add frontmatter with required fields and helpful metadata
5. **Implement changes** - Keep spec in sync as you learn
6. **Update status** - Mark progress: `draft` → `in-progress` → `complete`
7. **Archive when done** - `lean-spec archive <spec>` moves to archive

## Quality Standards

- Code is clear and maintainable
- Tests cover critical paths
- No unnecessary complexity
- Documentation where needed (not everywhere)
- Specs stay in sync with implementation

---

**Remember**: LeanSpec is a mindset. Adapt these guidelines to what actually helps.
