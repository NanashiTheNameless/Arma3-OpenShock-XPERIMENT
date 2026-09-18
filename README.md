# Arma 3 OpenShock — experimental fork

OpenShock extension and client addon for Arma 3, written in Rust and SQF. Shock, vibration, or sound can respond to `Hit`, `Killed`, `Suppressed`, or `Fired` events.

**THIS IS AN UNOFFICIAL MODIFIED VERSION OF [Arma3-PiShock BY TheCodeNugget / Ken The Nugget](https://github.com/TheCodeNugget/Arma3-PiShock).**
**THIS FORK USES OPENSHOCK INSTEAD OF PISHOCK. IT IS UNAFFILIATED WITH THE ORIGINAL AUTHOR OR OPENSHOCK.**
**EXPERIMENTAL. JERRY-RIGGED. NO SUPPORT. Please do not send this fork's bugs to the original author.**

## Disclaimer: AI-modified code ahead

This conversion was made with AI assistance. The AI has never worn your hardware, cannot feel pain, and passing software tests does not mean it has figured out your nervous system. What could possibly go wrong?

Sarcasm aside:

- This software controls hardware that delivers electrical shocks based on game events. Bugs, unexpected events, or network delays can cause unintended activation.
- This fork has not been validated in Arma 3 with real OpenShock hardware. Test with the shocker **off your body** first, starting with sound or vibration and the lowest settings.
- Use only with the informed consent of everyone involved. Follow your hardware manufacturer's placement and usage instructions. Stop using it if it causes pain or discomfort.
- The software killswitch blocks **new requests through this addon's functions**. It does not cancel requests already sent or stop an active hardware command. Keep the physical power switch within reach.
- Use at your own risk. This experimental fork comes with **no support and no warranty**; see [LICENSE](LICENSE). Do not treat this README or the code as a safety guarantee.

Friends don't let friends treat untested AI code as a certified safety system.

## Requirements and setup

- Arma 3 2.18 or newer and [CBA_A3](https://github.com/CBATeam/CBA_A3).
- An OpenShock account, an online hub, and a shocker accessible to your API token.
- This fork's compiled extension and addon. The original PiShock Workshop download does **not** contain this conversion.

1. Install the files using the layout below and enable CBA_A3 and `@A3OpenShock` in the launcher (or use `-mod=@CBA_A3;@A3OpenShock`). Do not load the original PiShock addon alongside this fork: both use the `NUG_fnc_*` function namespace.
2. Open **Options → Addon Options → Arma 3 OpenShock**.
3. Enter the **OpenShock Shocker ID** (the shocker's UUID, not the hub ID or a share code) and **OpenShock API Token** from your [OpenShock account](https://openshock.app/). The token needs permission to control that shocker. No PiShock username, share code, or API key is used.
4. Choose events, intensities, durations, and cooldowns. **All in-game durations are seconds, from 1 to 15.** Shock/vibration intensity is 1–100. New defaults are intensity 1 and duration 1 second.
5. Bind **Enable OpenShock Output** and **Disable OpenShock Output (Killswitch)** under **Controls → Configure Addons → Arma 3 OpenShock**. Output is disabled at mission start unless you explicitly enable **Enabled on Start**. The shock event handler is selected by default, but cannot send commands while output is disabled.
6. **Allow remote commands on Start** is off by default. Only enable it if you intend to let mission/server/other-player calls trigger the configured shocker. Local keybinds can toggle this permission. These checks are not a security boundary against arbitrary mission scripts running on your client; use trusted missions and servers.

Credentials are client settings stored by CBA in your Arma profile. They are not encrypted or hidden from local scripts. Do not share profile files, exported settings, or screenshots containing the token.

**Display Responses** shows asynchronous API acceptance or failure in system chat. An immediate `request queued` result only means the worker started. API acceptance does not confirm physical delivery. Requests have a 10-second timeout and are not automatically retried. The global cooldown applies across shock, vibration, and beep, in addition to each action's own cooldown.

## Building and installing

Install [Rust](https://www.rust-lang.org/tools/install) and Arma 3 Tools (Addon Builder). The Rust dependency `arma-rs` comes from crates.io; no sibling checkout is required.

```sh
cargo test --locked
cargo build --release --locked
```

For Windows Arma 3, build on Windows with the 64-bit Rust MSVC toolchain. Rename `target/release/arma3_openshock.dll` to `arma3_openshock_x64.dll`. A native Linux build produces `target/release/libarma3_openshock.so`; rename it to `arma3_openshock_x64.so` for a compatible Linux Arma runtime. A Linux build does not produce the Windows DLL.

Use Addon Builder to pack **A3_OpenShock/** into **A3_OpenShock.pbo**, with the PBO prefix **A3_OpenShock**. Keep the `.sqf` files in the PBO. Install:

```text
Arma 3/
└── @A3OpenShock/
    ├── arma3_openshock_x64.dll    (Windows; .so on Linux)
    └── addons/
        └── A3_OpenShock.pbo
```

Multiplayer servers may require a signed PBO and a server-approved extension. No signed release or BattlEye compatibility is promised by this source tree.

Tests validate the API payload, input bounds, extension command signatures, and HTTP success/error handling against a local mock server. They do not contact OpenShock or activate hardware. In-game event behavior still needs manual testing.

## Scripting and API

Existing SQF function names remain available:

```sqf
[1, 1] call NUG_fnc_shock;   // intensity, duration in seconds
[1, 1] call NUG_fnc_vibrate;
[1] call NUG_fnc_beep;       // duration in seconds
[2] call NUG_fnc_killswitch_handler; // disable further output
```

The addon calls `arma3_openshock` with `ops:shock` / `ops:vibrate` arguments `[shockerId, apiToken, intensity, durationSeconds]`, or `ops:beep` arguments `[shockerId, apiToken, durationSeconds]`. Callbacks use the name `arma3_openshock` and function `Shock`, `Vibrate`, or `Beep`.

Requests use `POST https://api.openshock.app/2/shockers/control`, the `OpenShockToken` header, and a descriptive User-Agent. Beep maps to OpenShock's `Sound` operation with intensity 0. Seconds are multiplied by 1000 exactly once at the API boundary: 5 seconds becomes 5000 milliseconds. The mod retains its 1–15 second limit even though the API accepts a wider range.

The checked-in [version 2 JSON](version-2.json) and [YAML](version-2.yaml) document the control endpoint and authentication; [version 1 JSON](version-1.json) and [YAML](version-1.yaml) cover endpoints not superseded by v2. See also the [OpenShock developer documentation](https://wiki.openshock.org/dev).

## License and attribution

Licensed under [GNU GPLv3](LICENSE). Original Arma3-PiShock code by Ken The Nugget / TheCodeNugget.

The original project's addon exception is retained: addons that only depend on or call these functions need not use the GPLv3. Addons that include this project's code in a PBO, or redistribute modified versions of its code or binaries, must comply with the GPLv3.
