/*
    Author: Ken The Nugget

    Description:
    Sends a shock request to the arma3_openshock extension.

    Parameter(s):
    0: Integer (1 - 100) - Intensity of the shock.
	1: Integer (1 - 15) - Duration of the shock.

    Return(s):
    None

    Example:
    [50, 5] call NUG_fnc_shock
*/

if (!hasInterface || isNull player) exitWith {};

params ["_intensity", "_duration"];

// Consent check

if (isRemoteExecuted && !(player getVariable ["NUG_allowRE", false])) exitWith {
	private _remoteExecutor = remoteExecutedOwner; // Store Executor ID
	
	if (_remoteExecutor <= 2) then { // Check if the executor is the server
		["<t color='#ff0000' size='.8'>Warning!<br />Unconsented remote exec detected from the server</t>",-1,-1,4,1,0,789] spawn BIS_fnc_dynamicText;
		playSound "3DEN_notificationWarning";
	} else {
		private _allUserIDs = allUsers;
		for "_i" from 0 to (count _allUserIds - 1) do { // Iterate player list to find the executor
			private _userInfo = getUserInfo (_allUserIDs select _i);
			_userInfo params ["_playerID", "_ownerId", "_SteamID", "_profileName", "_displayName", "_steamName", "_clientState", "_isHC", "_adminState", "_networkInfo", "_unit"];
			
			if ((_ownerId) == _remoteExecutor) then {
				_warnInfo = format ["Unconsented remote exec detected from: (%1), SteamID: (%2), SteamID: (%3)", _profileName, _steamName, _SteamID];
				_warnMsg = "<t color='#ff0000' size='.8'>Warning!<br />" + _warnInfo +"</t>";
				[_warnMsg,-1,-1,4,1,0,789] spawn BIS_fnc_dynamicText;
				playSound "3DEN_notificationWarning";
				break;
			};
		};
	};
};

// OpenShock Check

if !(player getVariable ["NUG_killswitch", false]) exitWith {
	systemChat "OpenShock disabled; command not sent.";
};

// Intensity and Duration checks

if (_intensity > 100 || _intensity < 1) exitWith {
	systemChat format ["Intensity out of bounds"];
};

if (_duration > 15 || _duration < 1) exitWith {
	systemChat format ["Duration out of bounds"];
};

// Shared and per-action cooldowns both apply.
private _now = diag_tickTime;
if ((_now - (player getVariable ["NUG_lastActionTime", -1e9])) < (round NUG_global_cooldown)
    || {(_now - (player getVariable ["NUG_lastShockTime", -1e9])) < (round NUG_shock_cooldown)}) exitWith {
    systemChat "Shock on cooldown";
};

private _result = "arma3_openshock" callExtension ["ops:shock", [NUG_openShock_shockerId, NUG_openShock_apiToken, round _intensity, round _duration]];
_result params ["_message", "_returnCode", "_errorCode"];
if (_returnCode != 0 || _errorCode != 0) exitWith {
    _result call NUG_fnc_extensionError;
};
player setVariable ["NUG_lastShockTime", _now];
player setVariable ["NUG_lastActionTime", _now];
