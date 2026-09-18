/*
    Author: Ken The Nugget

    Description:
    Handles Toggle/Enable/Disable EH for OpenShock Killswitch.

    Parameter(s):
    0: Integer (0 - 2) - Selection Toggle/Enable/Disable.

    Return(s):
    None

    Example:
    [0] call NUG_fnc_killswitch_handler
*/

if (!hasInterface || isNull player || isRemoteExecuted) exitWith {};

params ["_state"];

switch (_state) do {
	case 0: { // Toggle
		if (player getVariable ["NUG_killswitch", false]) then {
			[2] call NUG_fnc_killswitch_handler;
		} else {
			[1] call NUG_fnc_killswitch_handler;
		};
	};
	
	case 1: { // Enable
		player setVariable ["NUG_killswitch", true];
		systemChat "OpenShock output enabled.";
	};
	
	case 2: { // Disable
		player setVariable ["NUG_killswitch", false];
		systemChat "OpenShock output disabled (killswitch).";
	};
};