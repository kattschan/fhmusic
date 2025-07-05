# fhmusic
This program controls system media when Forza Horizon telemetry ("data out") indicates that the player is in menus or similar to make it easier for a player to use external music sources.
## Compatibility
| Game            | Game Platform            | App OS       | Support Status |   |
|-----------------|--------------------------|--------------|----------------|---|
| Forza Horizon 5 | Steam                    | Windows      | Confirmed      |   |
| Forza Horizon 5 | Xbox Store, Xbox Console | Windows      | Untested       |   |
| Forza Horizon 4 | ALL                      | Windows      | Unsupported       |   |
| ALL             | ALL                      | macOS, Linux | Unsupported    |   |
- Confirmed: Tested and confirmed to work, issues will be fixed
- Untested: Untested, issues will be fixed
- Unsupported: Untested, issues may or may not be fixed

As for media sources, the program simply simulates pressing keyboard play and pause keys. Any media source compatible with that works.
## Installation
1. Download the file for your system from release. Most will want the x64 setup exe, but other options are there if you know what you're doing. See compatibility above for which options I do and don't support.
2. Install the app using that file.
3. Open the game and navigate to Settings -> HUD and Gameplay
4. Enable ``Data Out``
5. Set ``Data Out IP Address`` to ``127.0.0.1``
6. Set ``Data Out IP Port`` to ``3200``
7. Save settings and enjoy!
## Issues and Contributions
If you have any issues with the software, please first look at the compatibility table above. If you are on a supported platform (this includes untested), please open an issue and I will do my best to implement a fix. If you are on an unsupported platform, I will not implement a fix myself. If you create a pull request with code to fix an issue on an unsupported platform, I will probably merge it.

<sub>
Has this software helped you out? Consider sponsoring me!
</sub>