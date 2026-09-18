/* Local opt-in for remotely requested actions. Remote callers cannot grant consent. */
if (!hasInterface || isNull player || isRemoteExecuted) exitWith {};

params ["_state"];
switch (_state) do {
    case 0: {
        if (player getVariable ["NUG_allowRE", false]) then {
            [2] call NUG_fnc_allowRE_handler;
        } else {
            [1] call NUG_fnc_allowRE_handler;
        };
    };
    case 1: {
        player setVariable ["NUG_allowRE", true];
        systemChat "OpenShock remote commands allowed.";
    };
    case 2: {
        player setVariable ["NUG_allowRE", false];
        systemChat "OpenShock remote commands blocked.";
    };
};
