# Usability test protocol and results

## Scenarios tested

| Scenario | Expected outcome | Automated coverage |
| --- | --- | --- |
| First-time user accepts defaults | Reaches review with no error | `first_run_user_flow_reaches_review` |
| User raises player count | RAM recommendation updates from 2 GB to 4 GB | `player_choice_updates_recommended_memory` |
| Docker user confirms setup | Compose file includes the maintained server image and settings | `docker_install_writes_runnable_files` |
| Native user needs another OS | Both PowerShell and POSIX scripts are generated | `native_install_has_both_platform_scripts` |
| Host needs advanced world controls | Port, seed, distance, hardcore, flight, command-block, world-size, and spawn settings are emitted | `advanced_settings_are_written_to_both_runtime_configs` |
| User tries an existing populated folder | Install stops before overwriting data | installer guard (manual negative-case checklist) |

## Human test script

1. Start the program in a 100x30 terminal and press Enter.
2. On *Server name* and *Install folder*, press `e`, type a replacement, and press Enter. Confirm the review reflects both values.
3. Change *Expected players* to 15. Confirm the adjacent memory value becomes 4 GB.
4. Set a seed, non-default port, and advanced world options. Confirm they appear on the review screen.
5. Switch *How to run* between Docker and Native Java; ensure the choice is understandable without documentation.
6. Navigate to *Continue*, review the summary, and press `b` to correct one setting.
7. Confirm installation in an empty temporary folder and verify the displayed start command matches the generated files.
8. Attempt the same installation in a non-empty folder. Confirm no existing file changes.

## Findings incorporated

- The review screen states that creation makes no network or system changes; downloading only happens when the generated start command is used.
- Docker is the default because it has the least platform-specific setup.
- RAM is automatically suggested after player-count changes so first-time hosts do not need to estimate it.
- Keyboard controls are always visible, rather than hidden behind a help screen.

Automated checks are executable user-flow proxies, not a substitute for recruiting real users. Run the human script before a public release and record feedback in GitHub Issues.
