# Light-League-Status
An league of legends overlay focused on performance and simplicity, so it can run in any computer and give only what you asked for!

# Development Backlog
---

## MVP (Minimum Viable Product) — Acceptance Criteria

* A visible overlay placed over the LoL client (click-through) that does not block or interfere with gameplay.
* Real-time updates (≤ 1s) of **KDA** for all players during the match, retrieved via the LoL **Live Client Data API**.
* Display of the **recommended build** (items) for each champion, sourced from **Data Dragon** (or a local cache).
* A secondary application window (GUI) that shows details (item lists, champion name, brief notes).
* Buildable with `cargo build` and clear usage instructions in the README.

---

## Backlog Structure (prioritized)

Items are ordered by priority (High → Medium → Low). Each item includes a short description and acceptance criteria.

### High Priority (essential deliveries for the prototype)

1. **Repository setup and basic CI**

   * Create `Cargo.toml`, `.gitignore`, `README`, and contribution templates.
   * Acceptance: Project builds and `cargo run` starts with a welcome message.

2. **Module: Live Client Data API access**

   * Implement an HTTP client that queries `https://127.0.0.1:2999/liveclientdata/playerlist` (or equivalent endpoints) and deserializes the returned JSON.
   * Acceptance: Console output lists players with K/D/A in real time.

3. **Module: Data Dragon (recommended builds)**

   * Implement a downloader/parser for Data Dragon JSON files to extract `recommended` items per champion. Optional local cache.
   * Acceptance: Given a champion name, return one complete recommended build.

4. **Graphical overlay (basic transparent window)**

   * Create a transparent always-on-top window using `winit` + `pixels` (or `wgpu`) and enable click-through.
   * Acceptance: Window appears on top of the game and does not intercept mouse clicks.

5. **Integration: KDA + Build → Render on overlay**

   * Map each player to their champion and render name/champion/KDA and item icons.
   * Acceptance: Overlay updates in real time and reflects changes to kills/deaths.

6. **Secondary window (detailed GUI)**

   * Simple GUI (egui/iced) that displays full builds, Data Dragon version, and logs.
   * Acceptance: Toggle between overlay and detail window via a shortcut or Alt+Tab.

### Medium Priority

1. **Item sprite caching (Data Dragon sprites)** — reduce network traffic.
2. **Cross-platform notes and scripts (Windows prioritized)** — build and run instructions.
3. **Configurable global shortcuts** — show/hide overlay, open details.
4. **Robust error handling** — handle client offline, LoL not running, invalid JSON.

### Low Priority / Future

1. **UI themes / skins** — minimal, compact, high-contrast styles.
2. **Option to fetch builds from third-party APIs (op.gg, etc.)**
3. **Packaging / distribution (GitHub releases, installers)**
4. **Host mode to share configurations (optional, without a central server)**

---

## Roadmap by Milestones (short sprints)

* **Sprint 1 (1–2 weeks):** Repo setup + Live Client Data API proof-of-concept + Data Dragon parsing + console demo.
* **Sprint 2 (1–2 weeks):** Basic transparent overlay window and static render prototype.
* **Sprint 3 (1–2 weeks):** Complete integration (real-time data → render) + secondary GUI.
* **Sprint 4 (1 week):** Testing, documentation, build scripts, and initial GitHub release.

---

## Contribution / Workflow Suggestions

1. Fork → `feature/<name>` branch → PR with description and checklist.
2. Write unit tests for parsers (Data Dragon, Live API).
3. Use issues for bugs and feature requests; tag issue with an estimated effort (1–5).

---

## Dev Dependencies & Prerequisites

* Rust toolchain (stable) and `cargo`.
* League of Legends client installed with default permissions.
* Optional tools: `curl` for tests, `jq` for JSON inspection.

---

## Minimal Prototype Checklist

* [ ] Code that queries the Live Client Data API and logs KDA.
* [ ] Data Dragon parser working for at least 5 champions.
* [ ] Transparent click-through overlay window.
* [ ] Integration that displays KDA + recommended build on the overlay.
* [ ] README with build and usage instructions.

---

## Quick Technical Notes

* Prefer `tokio` for async IO and periodic queries.
* Use `serde` for JSON deserialization.
* Avoid memory-reading the game process — use the local client API whenever possible.

---

If you want, I can turn the High Priority items into ready-to-create GitHub issues (title, description, and checklists).
