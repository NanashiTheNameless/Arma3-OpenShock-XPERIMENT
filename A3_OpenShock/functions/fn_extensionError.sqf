/* Explain callExtension failures separately from asynchronous HTTP responses. */
params ["_message", "_returnCode", "_errorCode"];

if (_errorCode != 0) exitWith {
    // https://community.bistudio.com/wiki/callExtension
    private _reason = switch (_errorCode) do {
        case 400: { "Extension could not load; check dependencies and the Arma RPT log." };
        case 403: { "BattlEye blocked the OpenShock extension. No API request was sent. Local testing requires BattlEye disabled; protected servers require an approved extension." };
        case 404: { "OpenShock extension not found; check that @A3OpenShock is enabled and contains arma3_openshock_x64.dll." };
        case 412: { "A mission script blocked the OpenShock extension; check the mission's extension policy." };
        case 415: { "Wrong extension architecture; use the x64 extension with 64-bit Arma." };
        case 301: { "Extension call took too long; delivery is unknown. Do not automatically retry." };
        default { "Extension call failed; check the Arma RPT log." };
    };
    systemChat format ["OpenShock engine error %1: %2", _errorCode, _reason];
};

systemChat format ["OpenShock command rejected: %1 (extension %2)", _message, _returnCode];
