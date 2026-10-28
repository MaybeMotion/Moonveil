# Moonveil 1.1 verification — 28 October 2026

Built and checked on the user's Apple Silicon Mac.

- `cargo test --locked`: **15 passed, 0 failed**. Includes progression and persistence checks from 1.0, all-wave enemy identity/guardian coverage, realm roster fixtures, aspect-preserving fit at multiple screen ratios, inverse mouse-coordinate mapping, padding rejection, and finite transforms for minimized windows.
- `cargo clippy --all-targets --locked -- -D warnings`: passed.
- `cargo fmt --check`: passed.
- Optimized release build, Mac app packaging, and strict ad-hoc code-signature verification: passed.
- Render smoke test completed: six main views, five realms, all eight enemy sprites, and the full six-member recruited battle party.
- Wide, square, portrait, and fullscreen captures were generated and inspected. The entire 16:10 canvas stays proportional and centered with padding. No screen stretching was observed.
- Full-body companion art was checked against the existing character portraits. All 12 new assets have alpha channels; 24 original illustrations are now bundled. Prompt and reference records are included in `ART_PROMPTS_V1_1.json`.
- The user's existing save was backed up and remained byte-for-byte unchanged throughout development and the isolated smoke test.
- The packaged app loaded the user's existing wave, equipment, training, and recruited companion levels successfully. Recruited allies and a new enemy appeared on the native app's battlefield.
- F11 fullscreen was verified visually in the packaged app at 1920×1080. Proportions and centered padding were correct.
- The UI automation connection timed out during a subsequent fullscreen pointer-navigation check. Screen-to-control mapping passed its automated tests; the game process and autosaving continued normally during that connection timeout. No claim is made that the timed-out pointer check completed.
- The updated game remains running with the user's progression preserved. User progress was not reset or replaced with smoke-test fixtures.

The included binary targets Apple Silicon macOS. Other operating systems and architectures have not been run here.
