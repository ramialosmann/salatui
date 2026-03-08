---
phase: quick
plan: 01
type: execute
wave: 1
depends_on: []
files_modified: [README.md]
autonomous: true
requirements: []

must_haves:
  truths:
    - "README explains what the project is and shows a screenshot placeholder"
    - "README lists all features and supported calculation methods"
    - "README covers installation, usage, configuration, and keybinds"
  artifacts:
    - path: "README.md"
      provides: "Project documentation for GitHub/crates.io"
      min_lines: 120
  key_links: []
---

<objective>
Create a comprehensive README.md for tui-adhan, preparing the project for publication to GitHub and crates.io.

Purpose: First impression for potential users -- must clearly communicate what the app does, how to install/configure it, and what calculation methods are supported.
Output: README.md at project root
</objective>

<execution_context>
@/home/rami/.claude/get-shit-done/workflows/execute-plan.md
@/home/rami/.claude/get-shit-done/templates/summary.md
</execution_context>

<context>
@Cargo.toml
@src/cli.rs
@src/config.rs (especially the default config template in generate_default_config and the calculation method list in CLI doc comment)
</context>

<tasks>

<task type="auto">
  <name>Task 1: Write README.md</name>
  <files>README.md</files>
  <action>
Create README.md at the project root with these sections in order:

1. **Title and tagline** -- `# tui-adhan` with a one-line description: "Islamic prayer times TUI clock for the terminal" (or similar). Add badges placeholder for crates.io version and license once those exist.

2. **Screenshot placeholder** -- An HTML comment or placeholder image tag: `<!-- screenshot: add terminal screenshot here before publishing -->` followed by a note like `![tui-adhan screenshot](screenshot.png)` (the file won't exist yet, that's fine).

3. **Features** -- Bulleted list covering:
   - Big ASCII art clock display (tty-clock style)
   - Live countdown to next prayer with prayer name
   - Hijri (Islamic calendar) date display
   - Full prayer schedule view (toggle with 's')
   - Desktop notifications via notify-send at prayer time
   - Terminal bell/flash alerts
   - Configurable per-prayer pre-alert notifications (default 15 min)
   - 12 calculation methods (list them)
   - Shafi and Hanafi Asr madhab support
   - High-latitude safe calculation
   - TOML configuration with auto-generation on first run
   - CLI overrides for quick use

4. **Installation** -- Two sub-sections:
   - **From crates.io**: `cargo install tui-adhan`
   - **From source**: `git clone` + `cargo build --release`
   - Note: requires `notify-send` (from libnotify) for desktop notifications on Linux

5. **Usage** -- Show basic invocation examples:
   - `tui-adhan` (uses config file)
   - `tui-adhan --lat 21.4225 --lon 39.8262` (CLI override)
   - `tui-adhan --lat 21.4225 --lon 39.8262 --method umm_al_qura --madhab hanafi`
   - Note that lat/lon are required (either in config or via CLI)

6. **Configuration** -- Explain:
   - Config auto-generated at `~/.config/tui-adhan/config.toml` on first run
   - Show the full default config template (copy from `generate_default_config` in config.rs verbatim -- it already has great comments)

7. **Calculation Methods** -- Table with method key and full name:
   | Key | Method |
   | mwl | Muslim World League |
   | egyptian | Egyptian General Authority of Survey |
   | karachi | University of Islamic Sciences, Karachi |
   | umm_al_qura | Umm al-Qura University, Makkah |
   | dubai | Dubai |
   | moonsighting_committee | Moonsighting Committee |
   | north_america / isna | Islamic Society of North America |
   | kuwait | Kuwait |
   | qatar | Qatar |
   | singapore | Singapore |
   | tehran | Institute of Geophysics, Tehran |
   | turkey | Diyanet Isleri Baskanligi, Turkey |

8. **Keybinds** -- Simple table:
   | Key | Action |
   | s | Toggle prayer schedule view |
   | q | Quit |

9. **Dependencies** -- Brief mention that the app uses the `salah` crate for prayer time calculation and `ratatui` for the TUI. Link to both crates.

10. **License** -- Use "MIT" as placeholder (common for Rust projects). Note: no LICENSE file exists yet; the user can adjust this.

Style notes:
- Keep it scannable -- use headers, bullets, and code blocks
- No emojis
- Write in second person ("you") for instructions
- Keep the tone practical and direct
  </action>
  <verify>
    <automated>test -f README.md && wc -l README.md | awk '{if ($1 >= 120) print "PASS: " $1 " lines"; else print "FAIL: only " $1 " lines"}'</automated>
  </verify>
  <done>README.md exists at project root with all 10 sections, is at least 120 lines, and accurately reflects the project's actual features, config format, and CLI options</done>
</task>

</tasks>

<verification>
- README.md exists and is well-structured
- All calculation methods from cli.rs are listed
- Config example matches generate_default_config in config.rs
- CLI flags match cli.rs definitions
</verification>

<success_criteria>
README.md is ready for GitHub publication (modulo adding an actual screenshot)
</success_criteria>

<output>
After completion, create `.planning/quick/1-create-a-readme-for-the-project-preparin/1-SUMMARY.md`
</output>
