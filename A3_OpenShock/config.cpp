class CfgPatches {
    class A3OpenShock {
        name = "OpenShock API Wrapper for Arma 3";
		author = "Ken The Nugget";
		requiredVersion = 2.18;
		requiredAddons[] = {"cba_main", "cba_xeh", "cba_settings", "cba_keybinding"};
    };

    // Legacy addon identity for upstream dependencies and CfgPatches checks.
    // Inherit the same prerequisites; functions and init handlers are registered once.
    class A3Pishock: A3OpenShock {
        name = "Arma 3 OpenShock (PiShock addon compatibility)";
    };
};

// Declare Functions
class CfgFunctions {
	class NUG {
		tag = "NUG";
		class functions {
			file = "\A3_OpenShock\functions";
			class shock{};
			class vibrate{};
			class beep{};
			class extensionError{};
			class shockEH_handler{};
			class vibrateEH_handler{};
			class beepEH_handler{};
			class APIResponseDisplay_handler{};
			class killswitch_handler{};
			class allowRE_handler{};
		};
	};
};

class cfgRemoteExec {
	class Commands {
		mode = 0;

		// Call Extension is set for following due to safety reasons
		// remoteExec Disabled
		// Only targets clients
		// JIP Disabled
		class callExtension {
			allowedTargets = 1;
			jip = 0;
		};
	};
};

// PreInit EH, Adds Settings and Keybinds
class Extended_PreInit_EventHandlers {
    class A3OpenShock {
        init = "call compile preprocessFileLineNumbers 'A3_OpenShock\XEH_preInit.sqf'";
    };
};

// PostInit EH, Initializes OpenShock on mission start according to settings
class Extended_PostInit_EventHandlers {
    class A3OpenShock {
        init = "call compile preprocessFileLineNumbers 'A3_OpenShock\XEH_postInit.sqf'";
    };
};
