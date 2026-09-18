// Account Settings

[
	"NUG_openShock_enable",
	"CHECKBOX",
	["Enabled on Start", "Enables OpenShock on mission start"],
	["Arma 3 OpenShock", "1.OpenShock Settings"],
	false,
	2
] call CBA_fnc_addSetting;

[
	"NUG_allow_remoteExec",
	"CHECKBOX",
	["Allow remote commands on Start", "Allows other players or the server to send shock commands"],
	["Arma 3 OpenShock", "1.OpenShock Settings"],
	false,
	2
] call CBA_fnc_addSetting;

[
    "NUG_openShock_shockerId",
    "EDITBOX",
    ["OpenShock Shocker ID", "Shocker UUID from your OpenShock dashboard (not the hub ID or a share code)."],
    ["Arma 3 OpenShock", "1.OpenShock Settings"],
    "",
    2
] call CBA_fnc_addSetting;

[
    "NUG_openShock_apiToken",
    "EDITBOX",
    ["OpenShock API Token", "API token created in your OpenShock account settings. Keep this private."],
    ["Arma 3 OpenShock", "1.OpenShock Settings"],
    "",
    2
] call CBA_fnc_addSetting;

[
    "NUG_global_cooldown",
	"SLIDER",
	["Global Cooldown", "Cooldown between all actions."],
	["Arma 3 OpenShock", "1.OpenShock Settings"],
	[1, 15, 1, 0],
	2
] call CBA_fnc_addSetting;

// 2.Shock Settings

[
	"NUG_shock_enabled",
	"CHECKBOX",
	["Enabled on Start", "Enables Shock on mission start"],
	["Arma 3 OpenShock", "2.Shock Settings"],
	true,
	2
] call CBA_fnc_addSetting;

[
    "NUG_shock_handler",
	"LIST",
	["Event Handler", "Event Handler for shocking."],
	["Arma 3 OpenShock", "2.Shock Settings"],
	[["Hit", "Killed", "Suppressed", "Fired"], ["Hit", "Killed", "Suppressed", "Fired"], 0],
	2,
	{
		if ((player getVariable ["NUG_shockEHIndex", -1]) >= 0) then {
			[1] call NUG_fnc_shockEH_handler;
		};
	}
] call CBA_fnc_addSetting;

[
    "NUG_shock_intensity",
	"SLIDER",
	["Shock Intensity", "Intensity of the shock."],
	["Arma 3 OpenShock", "2.Shock Settings"],
	[1, 100, 1, 0],
	2
] call CBA_fnc_addSetting;

[
    "NUG_shock_duration",
	"SLIDER",
	["Shock Duration", "Number of seconds to shock the wearer."],
	["Arma 3 OpenShock", "2.Shock Settings"],
	[1, 15, 1, 0],
	2
] call CBA_fnc_addSetting;

[
    "NUG_shock_cooldown",
	"SLIDER",
	["Shock Cooldown", "Cooldown between shocks."],
	["Arma 3 OpenShock", "2.Shock Settings"],
	[1, 15, 1, 0],
	2
] call CBA_fnc_addSetting;

// Vibrate Settings

[
	"NUG_vibrate_enabled",
	"CHECKBOX",
	["Enabled on Start", "Enables vibration on mission start"],
	["Arma 3 OpenShock", "3.Vibration Settings"],
	false,
	2
] call CBA_fnc_addSetting;

[
    "NUG_vibrate_handler",
	"LIST",
	["Event Handler", "Event Handler for vibrating."],
	["Arma 3 OpenShock", "3.Vibration Settings"],
	[["Hit", "Killed", "Suppressed", "Fired"], ["Hit", "Killed", "Suppressed", "Fired"], 0],
	2,
	{
		if ((player getVariable ["NUG_vibrateEHIndex", -1]) >= 0) then {
			[1] call NUG_fnc_vibrateEH_handler;
		};
	}
] call CBA_fnc_addSetting;

[
    "NUG_vibrate_intensity",
	"SLIDER",
	["Vibration Intensity", "Intensity of the vibration."],
	["Arma 3 OpenShock", "3.Vibration Settings"],
	[1, 100, 1, 0],
	2
] call CBA_fnc_addSetting;

[
    "NUG_vibrate_duration",
	"SLIDER",
	["Vibration Duration", "Number of seconds to vibrate the collar."],
	["Arma 3 OpenShock", "3.Vibration Settings"],
	[1, 15, 1, 0],
	2
] call CBA_fnc_addSetting;

[
    "NUG_vibrate_cooldown",
	"SLIDER",
	["Vibration Cooldown", "Cooldown between vibrations."],
	["Arma 3 OpenShock", "3.Vibration Settings"],
	[1, 15, 1, 0],
	2
] call CBA_fnc_addSetting;

// Beep Settings

[
	"NUG_beep_enabled",
	"CHECKBOX",
	["Enabled on Start", "Enables Beeping on mission start"],
	["Arma 3 OpenShock", "4.Beeping Settings"],
	false,
	2
] call CBA_fnc_addSetting;

[
    "NUG_beep_handler",
	"LIST",
	["Event Handler", "Event Handler for beeping."],
	["Arma 3 OpenShock", "4.Beeping Settings"],
	[["Hit", "Killed", "Suppressed", "Fired"], ["Hit", "Killed", "Suppressed", "Fired"], 0],
	2,
	{
		if ((player getVariable ["NUG_beepEHIndex", -1]) >= 0) then {
			[1] call NUG_fnc_beepEH_handler;
		};
	}
] call CBA_fnc_addSetting;

[
    "NUG_beep_duration",
	"SLIDER",
	["Beeping Duration", "Number of seconds to beep."],
	["Arma 3 OpenShock", "4.Beeping Settings"],
	[1, 15, 1, 0],
	2
] call CBA_fnc_addSetting;

[
    "NUG_beep_cooldown",
	"SLIDER",
	["Beeping Cooldown", "Cooldown between beeps."],
	["Arma 3 OpenShock", "4.Beeping Settings"],
	[1, 15, 2, 0],
	2
] call CBA_fnc_addSetting;

// Debug Settings

[
	"NUG_response_display",
	"CHECKBOX",
	["Display Responses", "Display API Responses"],
	["Arma 3 OpenShock", "5. Debug Settings"],
	false,
	2
] call CBA_fnc_addSetting;