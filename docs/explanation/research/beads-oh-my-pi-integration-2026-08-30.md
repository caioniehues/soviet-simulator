# Beads (`bd`) compatibility with Oh My Pi

**Kind:** explanation  
**Authority:** explanatory  
**Status:** research snapshot  
**Owner:** project lead  
**Last verified:** 2026-08-30

**Question:** Is there a maintained, drop-in Beads plugin, extension, skill, MCP server, or official adapter for **Oh My Pi** (`omp`)?

## Verdict

**No dedicated, maintained Oh My Pi Beads integration was found.** The official Beads setup catalogue names recipes for other agents but not Oh My Pi, and Oh My Pi's upstream tree has no Beads-specific capability.[^1][^2]

There is, however, one safe protocol-level composition: the Beads-maintained `beads-mcp` stdio server can be hosted by Oh My Pi's documented stdio MCP configuration. This provides MCP tools backed by `bd`; it is **not** an Oh My Pi-native plugin, skill, or official cross-project adapter.[^3][^4]

The lower-overhead approach for a shell-capable Oh My Pi session is to use `bd` directly and make Beads workflow guidance available through an Oh My Pi-discovered context file. That is workflow guidance, not native Beads tooling: Oh My Pi loads `AGENTS.md` as session context, whereas a native extension would use its `ExtensionAPI` to register tools, commands, or events.[^5][^6]

## Examined candidates

| Candidate | Owner and surface | Oh My Pi result |
|---|---|---|
| `beads-mcp` | Official Beads MCP server; exposes Beads operations as MCP tools backed by the `bd` CLI.[^3] | **Compatible via generic stdio MCP.** Oh My Pi supports a stdio server declared with `command`, optional `args`, `env`, and `cwd` in `.omp/mcp.json`.[^4] This is the only compatible structured-tool adapter found. |
| Beads CLI plus instructions | Beads documents `bd prime` as the workflow reference and treats instruction files as the primary surface for AGENTS-first agents.[^1] | **Compatible workflow guidance only.** Oh My Pi automatically discovers and injects `AGENTS.md`, but that supplies instructions; it does not register structured Beads tools, lifecycle hooks, or a Beads extension.[^5] |
| Beads Claude Code plugin | Official Beads plugin for Claude Code slash commands, skill, and lifecycle hooks; it explicitly does not bundle an MCP server.[^7] | **Rejected.** It is a Claude Code plugin, not an Oh My Pi plugin or adapter. |
| `pi-workgraph` | Third-party Beads extension for **Earendil Pi**; its package peers on `@earendil-works/pi-ai` and `@earendil-works/pi-coding-agent`.[^8][^9] | **Rejected comparator.** Oh My Pi accepts a legacy `pi.extensions` *manifest key*, but its documented extension contract is `@oh-my-pi/pi-coding-agent`/`omp.extensions`; neither upstream provides an Earendil-Pi-to-Oh-My-Pi compatibility adapter.[^6][^9] |

## Compatible configuration: `beads-mcp`

Use this only when MCP tools are specifically wanted. The Beads project publishes `beads-mcp` and documents installation with `uv tool install beads-mcp` (or `pip install beads-mcp`).[^3]

Oh My Pi documents `.omp/mcp.json` as its preferred project MCP file and supports the following stdio shape. Combining its host contract with Beads' documented executable yields:

```json
{
  "mcpServers": {
    "beads": {
      "type": "stdio",
      "command": "beads-mcp"
    }
  }
}
```

This note does not add that configuration. `beads-mcp` still depends on a usable `bd` installation; its tools translate operations to `bd` commands.[^3][^4]

## Limitations and safe choice

1. No source reviewed documents an Oh My Pi-specific Beads extension, skill, marketplace package, or `bd setup omp` recipe.[^1][^2]
2. The MCP composition is standards-compatible but carries Beads' documented MCP schema overhead of roughly 10–50k tokens, compared with roughly 1–2k tokens for the CLI-and-hooks route in a shell-capable environment.[^3]
3. Beads' hook recipes target particular agent runtimes. Do not install a Claude Code or Earendil Pi integration merely because Oh My Pi can discover some legacy conventions.[^6][^7][^9]
4. Therefore, the nearest safe default is direct `bd` use with explicitly maintained project guidance in an Oh My Pi-discovered context file. Adopt `beads-mcp` only for a deliberate structured-tool requirement; neither choice is a drop-in Oh My Pi Beads plugin.[^3][^5]

[^1]: Beads, [IDE setup](https://github.com/gastownhall/beads/blob/main/docs/getting-started/ide-setup.md) — official recipe catalogue, AGENTS-first instructions, and `bd prime` workflow reference.
[^2]: Oh My Pi, [upstream source tree](https://github.com/can1357/oh-my-pi/tree/main) — reviewed for Beads-specific first-party assets; none were present on 2026-08-30.
[^3]: Beads, [MCP server](https://github.com/gastownhall/beads/blob/main/docs/integrations/mcp-server.md) — official `beads-mcp` installation, command configuration, tool surface, CLI-first recommendation, and token-cost comparison.
[^4]: Oh My Pi, [MCP configuration](https://github.com/can1357/oh-my-pi/blob/main/docs/mcp-config.md) — `.omp/mcp.json` and stdio transport contract.
[^5]: Oh My Pi, [context files](https://github.com/can1357/oh-my-pi/blob/main/docs/context-files.md) — `AGENTS.md` discovery and injection behavior.
[^6]: Oh My Pi, [authoring extensions](https://github.com/can1357/oh-my-pi/blob/main/docs/skills/authoring-extensions.md) — native `ExtensionAPI`, `omp.extensions`, and legacy `pi.extensions` manifest handling.
[^7]: Beads, [Claude Code plugin](https://github.com/gastownhall/beads/blob/main/docs/integrations/claude-code-plugin.md) — Claude Code-only plugin scope and CLI-only operation.
[^8]: pi-workgraph, [README](https://github.com/gjtorikian/pi-workgraph/blob/main/README.md) — Earendil Pi requirement and `pi install` installation target.
[^9]: pi-workgraph, [package manifest](https://github.com/gjtorikian/pi-workgraph/blob/main/package.json) — `pi.extensions` and Earendil Pi peer dependencies.
